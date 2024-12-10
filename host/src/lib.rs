use bincode;
use ludo_core::{
    Color, InitializeGameStateCommit, LudoGameState, Play, PlayGameCommit, PlayGameParams,
    WinnersCommit,
};
use methods::{INIT_ELF, INIT_ID, PLAY_ELF, PLAY_ID, WINNERS_ELF, WINNERS_ID};
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

    // Helper function to write receipts to files
    fn write_receipt_to_files(receipt: &Receipt, image_id: &[u32; 8]) -> Result<()> {
        // let serialized = bincode::serialize(&receipt.inner)?;
        // fs::write(PROOF_FILE_PATH, serialized)?;
        // fs::write(IMAGE_ID_FILE_PATH, convert(image_id))?;
        // fs::write(PUB_INPUT_FILE_PATH, &receipt.journal.bytes)?;

        let receipt_inner_bytes_array = bincode::serialize(&receipt.inner).unwrap();
        fs::write("receipt_inner.hex", hex::encode(&receipt_inner_bytes_array)).unwrap();

        let receipt_journal_bytes_array = bincode::serialize(&receipt.journal).unwrap();
        println!(
            "Serialized bytes array (hex) JOURNAL: {:?}\n",
            hex::encode(&receipt_journal_bytes_array)
        );
        let mut image_id_hex = String::new();
        for &value in image_id {
            image_id_hex.push_str(&format!("{:08x}", value.to_be()));
        }
        println!(
            "Serialized bytes array (hex) IMAGE_ID: {:?}\n",
            image_id_hex
        );
        Ok(())
    }

    pub fn init(&self) -> Result<InitMessage> {
        let env = ExecutorEnv::builder().write(&self.state)?.build()?;
        let prover = default_prover();
        let receipt = prover.prove(env, INIT_ELF)?.receipt;
        Self::write_receipt_to_files(&receipt, &INIT_ID)?;
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
        Self::write_receipt_to_files(&receipt, &PLAY_ID)?;
        self.state = from_slice(&output)?;
        Ok(PlayMessage { receipt })
    }

    pub fn verify_winners(&self) -> Result<WinnersMessage> {
        // Check if we have up to 3 winners
        // if self.state.winners.len() >= 3 {
        //     return Err(anyhow::anyhow!("Game must have up to 3 winners"));
        // }

        let env = ExecutorEnv::builder().write(&self.state)?.build()?;

        let prover = default_prover();
        let receipt = prover.prove(env, WINNERS_ELF)?.receipt;
        Self::write_receipt_to_files(&receipt, &WINNERS_ID)?;

        let winner = self.state.winners[0];
        let winner_color = &self.state.players[winner].color;
        println!("{:?}", winner_color);
        Ok(WinnersMessage { receipt })
    }
}

pub struct WinnersMessage {
    pub receipt: Receipt,
}

impl WinnersMessage {
    pub fn verify_and_get_commit(&self) -> Result<Color> {
        self.receipt.verify(WINNERS_ID)?;
        Ok(self.receipt.journal.decode()?)
    }
}

pub fn convert(data: &[u32; 8]) -> [u8; 32] {
    let mut res = [0; 32];
    for i in 0..8 {
        res[4 * i..4 * (i + 1)].copy_from_slice(&data[i].to_le_bytes());
    }
    res
}
