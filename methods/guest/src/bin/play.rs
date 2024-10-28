#![no_main]
#![no_std]

use ludo_core::{PlayGameCommit, PlayGameParams};
use risc0_zkvm::{
    guest::env,
    serde::to_vec,
    sha::{Impl, Sha256},
};

risc0_zkvm::guest::entry!(main);

fn main() {
    let params: PlayGameParams = env::read();
    let result = params.process();
    env::write(&result.state);
    env::commit(&PlayGameCommit {
        old_state: *Impl::hash_words(&to_vec(&params.state).unwrap()),
        new_state: *Impl::hash_words(&to_vec(&result.state).unwrap()),
        current_player: params.play.current_player,
        dice_roll: params.play.dice_roll,
        piece_index: params.play.piece_index,
    });
}
