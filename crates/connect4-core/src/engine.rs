use crate::position::{Player, Position};
use crate::win_detection::is_win;
use std::io;
use crate::minimax;

#[derive(Eq, PartialEq)]
enum GameState {
    InProgress,
    Won(Player),
    Draw,
}

#[derive(Eq, PartialEq)]
enum GameMode {
    AIMode,
    TwoPlayer,
    OnePlayer,
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

    println!("Welcome to Connect 4.");
    // select game mode
    println!("Select which game mode you would like to play. 1P: Player v CPU, 2P: Local multiplayer, CPU: let the CPU battle it out.");
    let game_mode = loop {
      let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read game mode");

        match input.trim().to_lowercase().as_str() {
            "1p" => break GameMode::OnePlayer,
            "2p" => break GameMode::TwoPlayer,
            "cpu" => break GameMode::AIMode,
            _ => println!("Invalid input, select 1P, 2P or CPU.")
        }
    };

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

    let mut difficulty = 0;

    if game_mode != GameMode::TwoPlayer {
        //let player select game difficulty, at the moment this is based on the depth the minimax algorithm will assess to, TODO: make a more robust "difficulty" as level 3 is enough to be hard to beat.
        println!("Select difficulty: 1-10");
        difficulty = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read difficulty");

            match input.trim().parse::<usize>() {
                Ok(diff) if diff >= 1 && diff <= 10 => break diff,
                _ => println!("Invalid difficulty entered, select a number between 1-10.")
            }
        };
    }

    // randomly select the first player
    if rand::random_bool(0.5) {
        pos.player_to_move = Player::Blue;
    }

    while game_state == GameState::InProgress {
        //display game board
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", pos);

        let current_player = pos.player_to_move();
        if game_mode != GameMode::AIMode {
            if current_player == player_colour {
                //prompt current player input
                println!("Enter which column you wish to place your token (1-{}):", Position::WIDTH);
                let column = loop {
                    let mut column_input = String::new();
                    io::stdin().read_line(&mut column_input).expect("Failed to read column");
                    match column_input.trim().parse::<usize>() {
                        Ok(col) if (col <= (Position::WIDTH)) && (col != 0) => break col - 1, // column index 0 treated as starting point in the engine so convert user input
                        _ => println!("Invalid input. Enter a number between 1 and {}.", Position::WIDTH)
                    }
                };

                if !pos.can_play(column) {
                    println!("Column is full.");
                    continue;
                }

            pos.play(column);
            }
        }
        else if game_mode != GameMode::TwoPlayer { // AI to move
            let search = minimax::best_move(&pos, difficulty);
            match search.best_move {
                Some(column) => pos.play(column),
                _ => panic!("AI could not select a best move.")
            }
            println!("AI chose best move as column: {} based on searching {} nodes in {}ms with a score of {}.",
                     search.best_move.unwrap() + 1,
                     search.nodes,
                     search.elapsed_ms,
                     search.move_score
            );
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