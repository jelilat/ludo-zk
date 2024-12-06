use host::{players, Game};
use ludo_core::LudoGameState;
fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let players = players::get_players();

    let ludo_game_state = LudoGameState {
        players,
        current_player: 0,
        dice_roll: 0,
        winners: vec![],
        sixes: 0,
    };
    let mut game = Game::new(ludo_game_state);
    match game.init() {
        Ok(init_message) => match init_message.verify_and_get_commit() {
            Ok(commit) => println!("Commit: {:?}", commit),
            Err(e) => eprintln!("Failed to verify init commit: {:?}", e),
        },
        Err(e) => eprintln!("Failed to init game: {:?}", e),
    }

    // let play1 = Play {
    //     current_player: 0,
    //     dice_roll: 6,
    //     piece_index: 0,
    // };

    // match game.play(&play1) {
    //     Ok(play_message) => match play_message.verify_and_get_commit() {
    //         Ok(commit) => println!("Commit: {:?}", commit),
    //         Err(e) => eprintln!("Failed to verify play commit: {:?}", e),
    //     },
    //     Err(e) => eprintln!("Failed to play game: {:?}", e),
    // }

    // let play2 = Play {
    //     current_player: 0,
    //     dice_roll: 2,
    //     piece_index: 0,
    // };

    // let play_message2 = game.play(&play2).unwrap();
    // println!("");
    // let commit2 = play_message2.verify_and_get_commit().unwrap();
    // println!("Commit2: {:?}", commit2);
    // println!("");
    // println!("Game state: {:?}", game.state);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ludo_core::{LudoGameState, Play};

    #[test]
    fn test_game_initialization_and_play() {
        // Setup initial game state
        let players = players::get_players();
        let ludo_game_state = LudoGameState {
            players,
            current_player: 0,
            dice_roll: 0,
            winners: vec![],
            sixes: 0,
        };

        let mut game = Game::new(ludo_game_state);

        // Test game initialization
        match game.init() {
            Ok(init_message) => match init_message.verify_and_get_commit() {
                Ok(commit) => println!("Commit: {:?}", commit),
                Err(e) => panic!("Failed to verify init commit: {:?}", e),
            },
            Err(e) => panic!("Failed to init game: {:?}", e),
        }

        // Test first play
        let play1 = Play {
            current_player: 0,
            dice_roll: 6,
            piece_index: 0,
        };

        match game.play(&play1) {
            Ok(play_message) => match play_message.verify_and_get_commit() {
                Ok(commit) => println!("Commit: {:?}", commit),
                Err(e) => panic!("Failed to verify play commit: {:?}", e),
            },
            Err(e) => panic!("Failed to play game: {:?}", e),
        }

        // Test second play
        let play2 = Play {
            current_player: 0,
            dice_roll: 2,
            piece_index: 0,
        };

        match game.play(&play2) {
            Ok(play_message2) => match play_message2.verify_and_get_commit() {
                Ok(commit2) => println!("Commit2: {:?}", commit2),
                Err(e) => panic!("Failed to verify play commit: {:?}", e),
            },
            Err(e) => panic!("Failed to play game: {:?}", e),
        }

        println!("Game state: {:?}", game.state);
    }
}
