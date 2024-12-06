use bincode;
use ludo_core::{InitializeGameStateCommit, LudoGameState, Play, PlayGameCommit, PlayGameParams};
use methods::{INIT_ELF, INIT_ID, PLAY_ELF, PLAY_ID};
use risc0_zkvm::{default_prover, serde::from_slice, ExecutorEnv, Receipt, Result};
use std::fs;

const PROOF_FILE_PATH: &str = "play_receipt.proof";
const IMAGE_ID_FILE_PATH: &str = "play_receipt_id.bin";
const PUB_INPUT_FILE_PATH: &str = "play_receipt.pub";

pub mod players;
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
    pub state: LudoGameState,
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

pub fn convert(data: &[u32; 8]) -> [u8; 32] {
    let mut res = [0; 32];
    for i in 0..8 {
        res[4 * i..4 * (i + 1)].copy_from_slice(&data[i].to_le_bytes());
    }
    res
}
