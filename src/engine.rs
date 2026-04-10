use std::cmp::PartialEq;
use std::io;
use crate::position::{Player, Position};
use crate::win_detection::is_win;

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
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", pos);



        //prompt current player input
        println!("Enter which column you wish to place your token (0-6):");
        let mut column_input = String::new();
        io::stdin().read_line(&mut column_input).expect("Failed to read column");
        let column = column_input.trim().parse().expect("Please enter a valid number");
        //play move if valid
        let current_player = pos.player_to_move();
        if pos.try_play(column).is_err() {
            continue;
        }
        //check if a player has won or draw
        if is_win(pos.get_bitboard(current_player)) {
            println!("Player: {:?} wins!", current_player);
            game_state = GameState::Won(current_player);
        } else if pos.board_full() {
            println!("Draw.");
            game_state = GameState::Draw;
        }

    }

}

