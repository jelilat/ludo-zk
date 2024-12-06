use axum::{extract::State, Json};
use uuid::Uuid;

use crate::types::{InitResponse, PlayRequest, PlayResponse};
use host::{players, Game};
use ludo_core::{LudoGameState, Play};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type GameStore = Arc<Mutex<HashMap<String, Game>>>;

pub async fn initialize_game(State(games): State<GameStore>) -> Json<InitResponse> {
    let players = players::get_players();
    let ludo_game_state = LudoGameState {
        players,
        current_player: 0,
        dice_roll: 0,
        winners: vec![],
        sixes: 0,
    };

    let game_instance = Game::new(ludo_game_state);
    let init_message = game_instance.init().expect("Failed to initialize game");
    let commit = init_message
        .verify_and_get_commit()
        .expect("Failed to verify init commit");

    let game_id = Uuid::new_v4().to_string();
    games.lock().await.insert(game_id.clone(), game_instance);

    Json(InitResponse { game_id, commit })
}

pub async fn play_game(
    State(games): State<GameStore>,
    Json(play_request): Json<PlayRequest>,
) -> Json<PlayResponse> {
    let mut games = games.lock().await;

    let game_instance = games
        .get_mut(&play_request.game_id)
        .expect("Game not found");

    let play = Play {
        current_player: play_request.current_player,
        dice_roll: play_request.dice_roll,
        piece_index: play_request.piece_index,
    };

    let play_message = game_instance.play(&play).expect("Failed to make play");
    let commit = play_message
        .verify_and_get_commit()
        .expect("Failed to verify play commit");

    Json(PlayResponse {
        commit,
        state: game_instance.state.clone(),
    })
}
