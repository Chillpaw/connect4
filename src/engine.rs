use std::cmp::PartialEq;
use std::io;
use crate::position::{Player, Position};

#[derive(Eq, PartialEq)]
pub enum GameState {
    InProgress,
    Won(Player),
    Draw,
}

impl GameState {
    pub fn new() -> Self {
        GameState::InProgress
    }
}

pub fn game_start() {
    let mut game_state = GameState::new();
    let mut pos = Position::new();

    // randomly select the first player
    if rand::random_bool(0.5) {
        pos.player_to_move = Player::Blue;
    }

    while game_state == GameState::InProgress {
        //display game board
        println!("{}", pos);
        //prompt current player input
        println!("Enter which column you wish to place your token (0-6):");
        let mut column_input = String::new();
        io::stdin().read_line(&mut column_input);
        //play move if valid

    }

}

