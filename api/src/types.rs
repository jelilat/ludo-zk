use ludo_core::{InitializeGameStateCommit, LudoGameState, PlayGameCommit};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct InitResponse {
    pub game_id: String,
    pub commit: InitializeGameStateCommit,
}

#[derive(Serialize)]
pub struct PlayResponse {
    pub commit: PlayGameCommit,
    pub state: LudoGameState,
}

#[derive(Deserialize)]
pub struct PlayRequest {
    pub game_id: String,
    pub current_player: usize,
    pub dice_roll: u8,
    pub piece_index: u8,
}
