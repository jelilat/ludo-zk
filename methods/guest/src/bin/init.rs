#![no_main]
#![no_std]

use ludo_core::{InitializeGameStateCommit, LudoGameState};
use risc0_zkvm::{
    guest::env,
    serde::to_vec,
    sha::{Impl, Sha256},
};

risc0_zkvm::guest::entry!(main);

fn main() {
    let state: LudoGameState = env::read();
    env::commit(&InitializeGameStateCommit {
        current_player: state.current_player,
        dice_roll: state.dice_roll,
        winners: state.winners.clone(),
        sixes: state.sixes.clone(),
        state_hash: *Impl::hash_words(&to_vec(&state).unwrap()),
    });
}
