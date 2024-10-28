use ludo_core::{Color, Piece, PieceStatus, Player};

const WIN: u8 = 100;

const RED_PATH: &[u8] = &[
    19, 20, 21, 22, 23, 15, 12, 9, 6, 3, 0, 1, 2, 5, 8, 11, 14, 17, 24, 25, 26, 27, 28, 29, 41, 53,
    52, 51, 50, 49, 48, 56, 59, 62, 65, 68, 71, 70, 69, 66, 63, 60, 57, 54, 47, 46, 45, 44, 43, 42,
    30, 31, 32, 33, 34, 35, WIN,
];

const GREEN_PATH: &[u8] = &[
    5, 8, 11, 14, 17, 24, 25, 26, 27, 28, 29, 41, 53, 52, 51, 50, 49, 48, 56, 59, 62, 65, 68, 71,
    70, 69, 66, 63, 60, 57, 54, 47, 46, 45, 44, 43, 42, 30, 18, 19, 20, 21, 22, 23, 15, 12, 9, 6,
    3, 0, 1, 4, 7, 10, 13, 16, WIN,
];

const BLUE_PATH: &[u8] = &[
    66, 63, 60, 57, 54, 47, 46, 45, 44, 43, 42, 30, 18, 19, 20, 21, 22, 23, 15, 12, 9, 6, 3, 0, 1,
    2, 5, 8, 11, 14, 17, 24, 25, 26, 27, 28, 29, 41, 53, 52, 51, 50, 49, 48, 56, 59, 62, 65, 68,
    71, 70, 67, 64, 61, 58, 55, WIN,
];

const YELLOW_PATH: &[u8] = &[
    52, 51, 50, 49, 48, 56, 59, 62, 65, 68, 71, 70, 69, 66, 63, 60, 57, 54, 47, 46, 45, 44, 43, 42,
    30, 18, 19, 20, 21, 22, 23, 15, 12, 9, 6, 3, 0, 1, 2, 5, 8, 11, 14, 17, 24, 25, 26, 27, 28, 29,
    41, 40, 39, 38, 37, 36, WIN,
];

const PIECES: [Piece; 4] = [
    Piece {
        position: -1,
        status: PieceStatus::Home,
    },
    Piece {
        position: -1,
        status: PieceStatus::Home,
    },
    Piece {
        position: -1,
        status: PieceStatus::Home,
    },
    Piece {
        position: -1,
        status: PieceStatus::Home,
    },
];

pub fn get_players() -> Vec<Player> {
    vec![
        Player::new(
            "Player 1".to_string(),
            Color::Red,
            PIECES,
            RED_PATH.to_vec(),
        ),
        Player::new(
            "Player 2".to_string(),
            Color::Green,
            PIECES,
            GREEN_PATH.to_vec(),
        ),
        Player::new(
            "Player 3".to_string(),
            Color::Blue,
            PIECES,
            BLUE_PATH.to_vec(),
        ),
        Player::new(
            "Player 4".to_string(),
            Color::Yellow,
            PIECES,
            YELLOW_PATH.to_vec(),
        ),
    ]
}
