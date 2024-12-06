#![no_main]
#![no_std]

use ludo_core::{LudoGameState, WinnersCommit};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // Read the game state from the host
    let state: LudoGameState = env::read();

    // Verify we have exactly 3 winners
    assert_eq!(state.winners.len(), 3, "Game must have exactly 3 winners");

    // Create winners commit
    let winners_commit = WinnersCommit {
        winners: state.winners.clone(),
    };

    // Commit the winners
    env::commit(&winners_commit);
}
