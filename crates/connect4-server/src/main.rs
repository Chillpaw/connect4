use std::cmp::PartialEq;
use std::io;
use connect4_core::position::{Player, Position};
use connect4_core::win_detection::is_win;

#[derive(Eq, PartialEq)]
enum GameState {
    InProgress,
    Won(Player),
    Draw,
}

/// Runs the interactive Connect Four game loop in the terminal.
///
/// The function repeatedly renders the board, prompts the current player to enter a column (0–6),
/// applies the chosen move, and updates the game state. It detects and announces a win or a draw
/// and exits when the game is finished.
///
/// # Examples
///
/// ```no_run
/// // Starts the interactive game loop; use in a terminal.
/// main();
/// ```
fn main() {
    let mut game_state = GameState::InProgress;
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
        println!("Enter which column you wish to place your token (1-{}):", Position::WIDTH);
        let mut column_input = String::new();
        io::stdin().read_line(&mut column_input).expect("Failed to read column");
        let column = match column_input.trim().parse::<usize>() {
            Ok(col) if (col < (Position::WIDTH - 1)) && (col != 0) => col - 1, // column index 0 treated as starting point in the engine so convert user input
            _ => {
                println!("Invalid input. Enter a number between 1 and {}.", Position::WIDTH);
                continue;
            }
        };

        if !pos.can_play(column) {
            println!("Column is full.");
            continue;
        }


        //play move if valid
        let current_player = pos.player_to_move();
        pos.play(column);

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
