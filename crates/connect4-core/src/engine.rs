use crate::position::{Player, Position};
use crate::win_detection::is_win;
use rand::prelude::*;
use std::io;
use crate::minimax;

#[derive(Eq, PartialEq)]
enum GameState {
    InProgress,
    Won(Player),
    Draw,
}

/// Runs the interactive Connect Four game loop in the terminal.
///
/// The function repeatedly renders the board, prompts the current player to enter a column (1–7),
/// applies the chosen move, and updates the game state. It detects and announces a win or a draw
/// and exits when the game is finished.
///
/// # Examples
///
/// ```no_run
/// // Starts the interactive game loop; use in a terminal.
/// use connect4_core::engine::run;
/// run();
/// ```
pub fn run() {
    let mut game_state = GameState::InProgress;
    let mut pos = Position::new();

    // let the player select their colour
    println!("Welcome to Connect 4. Would you like to play as Red or Blue?");

    let player_colour = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read colour");

        match input.trim().to_lowercase().as_str() {
            "red" => break Player::Red,
            "blue" => break Player::Blue,
            _ => println!("Invalid input, try again (red or blue).")
        }
    };

    // randomly select the first player
    if rand::random_bool(0.5) {
        pos.player_to_move = Player::Blue;
    }

    while game_state == GameState::InProgress {
        //display game board
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", pos);

        let current_player = pos.player_to_move();

        if current_player == player_colour {
            //prompt current player input
            println!("Enter which column you wish to place your token (1-{}):", Position::WIDTH);
            let mut column_input = String::new();
            }
            io::stdin().read_line(&mut column_input).expect("Failed to read column");
            let column = match column_input.trim().parse::<usize>() {
                Ok(col) if (col <= (Position::WIDTH)) && (col != 0) => col - 1, // column index 0 treated as starting point in the engine so convert user input
                _ => {
                    println!("Invalid input. Enter a number between 1 and {}.", Position::WIDTH);
                    continue;
                }
            };

            if !pos.can_play(column) {
                println!("Column is full.");
                continue;
            }

            pos.play(column);
        }
        else { // AI to move
            let search = minimax::best_move(&pos, 3);
            match search.best_move {
                Some(column) => pos.play(column),
                _ => panic!("AI could not select a best move.")
            }
            println!("AI chose best move as column: {} based on searching {} nodes in {}ms.", search.best_move.unwrap(), search.nodes, search.elapsed_ms);
        }

        //check if a player has won

        if is_win(pos.get_bitboard(current_player)) {
            print!("\x1B[2J\x1B[1;1H");
            println!("{}", pos);
            println!("Player: {:?} wins!", current_player);
            game_state = GameState::Won(current_player);
        }

        //check if the board is full (draw)
        if pos.board_full() {
            println!("Draw.");
            game_state = GameState::Draw;
        }
    }
}