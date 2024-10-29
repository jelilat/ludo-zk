use bincode;
use ludo_core::{InitializeGameStateCommit, LudoGameState, Play, PlayGameCommit, PlayGameParams};
use methods::{INIT_ELF, INIT_ID, PLAY_ELF, PLAY_ID};
use risc0_zkvm::{default_prover, serde::from_slice, ExecutorEnv, Receipt, Result};
use std::fs;

const PROOF_FILE_PATH: &str = "play_receipt.proof";
const IMAGE_ID_FILE_PATH: &str = "play_receipt_id.bin";
const PUB_INPUT_FILE_PATH: &str = "play_receipt.pub";

mod players;
pub struct InitMessage {
    receipt: Receipt,
}

impl InitMessage {
    pub fn get_state(&self) -> Result<InitializeGameStateCommit> {
        Ok(self.receipt.journal.decode()?)
    }

    pub fn verify_and_get_commit(&self) -> Result<InitializeGameStateCommit> {
        self.receipt.verify(INIT_ID)?;
        self.get_state()
    }
}

pub struct PlayMessage {
    receipt: Receipt,
}

impl PlayMessage {
    pub fn get_state(&self) -> Result<PlayGameCommit> {
        Ok(self.receipt.journal.decode()?)
    }

    pub fn verify_and_get_commit(&self) -> Result<PlayGameCommit> {
        self.receipt.verify(PLAY_ID)?;
        self.get_state()
    }
}

#[derive(Debug)]
pub struct Game {
    state: LudoGameState,
}

impl Game {
    pub fn new(state: LudoGameState) -> Self {
        Self { state }
    }

    pub fn init(&self) -> Result<InitMessage> {
        let env = ExecutorEnv::builder().write(&self.state)?.build()?;
        let prover = default_prover();
        let receipt = prover.prove(env, INIT_ELF)?.receipt;

        let serialized = bincode::serialize(&receipt.inner).unwrap();
        fs::write(PROOF_FILE_PATH, serialized).expect("Failed to write proof file");
        fs::write(IMAGE_ID_FILE_PATH, convert(&INIT_ID)).expect("Failed to write image_id file");

        fs::write(PUB_INPUT_FILE_PATH, &receipt.journal.bytes)
            .expect("Failed to write pub_input file");

        Ok(InitMessage { receipt })
    }

    pub fn play(&mut self, play: &Play) -> Result<PlayMessage> {
        let params = PlayGameParams::new(self.state.clone(), play.clone());
        let mut output = Vec::new();
        let env = ExecutorEnv::builder()
            .write(&params)?
            .stdout(&mut output)
            .build()?;
        let prover = default_prover();
        let receipt = prover.prove(env, PLAY_ELF)?.receipt;

        let serialized = bincode::serialize(&receipt.inner).unwrap();
        // Write the serialized receipt to a file
        fs::write(PROOF_FILE_PATH, serialized).expect("Failed to write proof file");
        fs::write(IMAGE_ID_FILE_PATH, convert(&PLAY_ID)).expect("Failed to write image_id file");

        fs::write(PUB_INPUT_FILE_PATH, &receipt.journal.bytes)
            .expect("Failed to write pub_input file");

        self.state = from_slice(&output)?;
        Ok(PlayMessage { receipt })
    }
}

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

pub fn convert(data: &[u32; 8]) -> [u8; 32] {
    let mut res = [0; 32];
    for i in 0..8 {
        res[4 * i..4 * (i + 1)].copy_from_slice(&data[i].to_le_bytes());
    }
    res
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
