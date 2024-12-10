// #![cfg_attr(not(test), no_std)]

use risc0_zkp::core::digest::Digest;
// use risc0_zkvm::{
//     serde::to_vec,
//     sha::{Impl, Sha256},
// };
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PieceStatus {
    Home,   // Piece is in starting position
    Active, // Piece is on the board
    Win,    // Piece has reached the end
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Piece {
    pub position: i8, // Current position on the path (-1 for home)
    pub status: PieceStatus,
}

impl Piece {
    pub fn new(position: i8, status: PieceStatus) -> Self {
        Piece { position, status }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub color: Color,
    pub pieces: [Piece; 4], // Each player has 4 pieces
    pub path: Vec<u8>,      // The sequence of board positions this player must follow
}

impl Player {
    pub fn new(name: String, color: Color, pieces: [Piece; 4], path: Vec<u8>) -> Self {
        Player {
            name,
            color,
            pieces,
            path,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LudoGameState {
    pub players: Vec<Player>,
    pub current_player: usize,
    pub dice_roll: u8,
    pub winners: Vec<usize>, // Tracks players who have won
    pub sixes: u8,           // Counts consecutive sixes rolled
}

impl LudoGameState {
    // Handles dice roll and determines if player's turn should end
    // Returns true if the turn should end automatically
    pub fn roll_dice(&mut self, dice_roll: u8) -> bool {
        self.dice_roll = dice_roll;

        if dice_roll == 6 {
            self.winners.push(self.current_player);
            self.sixes += 1;
        }

        let active_pieces = self.players[self.current_player]
            .pieces
            .iter_mut()
            .filter(|p| p.status == PieceStatus::Active)
            .collect::<Vec<&mut Piece>>();

        if (active_pieces.len() == 0 && dice_roll != 6) || self.sixes >= 3 {
            self.current_player = self.get_next_turn();
            self.sixes = 0;
            return true;
        } else if active_pieces.len() == 1 && dice_roll != 6 {
            self.move_piece(0);
            return true;
        }

        false
    }

    // Determines the next player's turn, skipping any winners
    pub fn get_next_turn(&self) -> usize {
        let mut next_index = (self.current_player + 1) % self.players.len();
        while self.winners.contains(&next_index) {
            next_index = (next_index + 1) % self.players.len();
        }
        next_index
    }

    // Handles piece movement logic including:
    // - Moving active pieces forward
    // - Bringing pieces out of home on rolling 6
    // - Handling winning conditions
    pub fn move_piece(&mut self, piece_index: usize) {
        let player = &mut self.players[self.current_player];
        let piece = &mut player.pieces[piece_index];

        if piece.status == PieceStatus::Active {
            let new_position = piece.position + self.dice_roll as i8;
            if new_position as usize == player.path.len() - 1 {
                piece.status = PieceStatus::Win;
                piece.position = new_position;
                if player.pieces.iter().all(|p| p.status == PieceStatus::Win) {
                    self.winners.push(self.current_player);
                }
            } else if (new_position as usize) < player.path.len() {
                piece.position = new_position;
                self.handle_collision(new_position);
            }
        } else if piece.status == PieceStatus::Home && self.dice_roll == 6 {
            piece.status = PieceStatus::Active;
            piece.position = 0;
            self.handle_collision(0);
        }
    }

    // Handles collisions between pieces:
    // - Sends opponent pieces back home if landed on
    // - Updates turn based on dice roll
    fn handle_collision(&mut self, new_position: i8) {
        let current_player = self.current_player;
        let current_position = self.players[current_player].path[new_position as usize];

        for (i, player) in self.players.iter_mut().enumerate() {
            if i != current_player {
                for piece in &mut player.pieces {
                    if piece.status == PieceStatus::Active
                        && player.path[piece.position as usize] == current_position
                    {
                        piece.status = PieceStatus::Home;
                        piece.position = -1;
                        return;
                    }
                }
            }
        }

        if self.dice_roll != 6 || self.sixes >= 3 {
            self.current_player = self.get_next_turn();
            self.sixes = 0;
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InitializeGameStateCommit {
    pub current_player: usize,
    pub dice_roll: u8,
    pub winners: Vec<usize>,
    pub sixes: u8,
    pub state_hash: Digest,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Play {
    pub current_player: usize,
    pub dice_roll: u8,
    pub piece_index: u8, // Which piece the player chose to move
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlayGameCommit {
    pub old_state: Digest,
    pub new_state: Digest,
    pub current_player: usize,
    pub dice_roll: u8,
    pub piece_index: u8,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlayGameParams {
    pub state: LudoGameState,
    pub play: Play,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlayGameResult {
    pub state: LudoGameState,
    pub next_player: usize,
}

impl PlayGameParams {
    pub fn new(state: LudoGameState, play: Play) -> Self {
        PlayGameParams { state, play }
    }

    // Processes a single play action:
    // 1. Rolls the dice
    // 2. Moves the chosen piece if necessary
    // 3. Returns the new game state and next player
    pub fn process(&self) -> PlayGameResult {
        // TODO: require state.current_player == self.play.current_player
        let mut state = self.state.clone();
        let player_moved = state.roll_dice(self.play.dice_roll);
        if !player_moved {
            state.move_piece(self.play.piece_index as usize);
        }

        // state.dice_roll = 0;
        let next_player = state.current_player;

        PlayGameResult { state, next_player }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WinnersCommit {
    pub winners: Vec<usize>,
}
