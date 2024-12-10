#![no_main]
#![no_std]

use ludo_core::{LudoGameState, WinnersCommit};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // Read the game state from the host
    let state: LudoGameState = env::read();

    // Verify we have exactly 3 winners
    // assert_eq!(state.winners.len(), 3, "Game must have exactly 3 winners");

    // Create winners commit
    // let winners_commit = WinnersCommit {
    //     winners: state.winners.clone(),
    // };

    if !state.winners.is_empty() {
        let winner = state.winners[0];
        let winner_color = &state.players[winner].color;

        // Commit the winners
        env::commit(&winner_color);
    }
}
