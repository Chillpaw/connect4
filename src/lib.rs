//! Connect Four engine: bitboard state, rules, win detection, and minimax-based move search.
//!
//! Run the terminal client with the `connect4` binary (`cargo run`). For programmatic use,
//! construct a [`Position`], call [`Position::try_play`], and optionally query [`best_move`] for the side to move.

pub mod board;
pub mod engine;
pub mod minimax;
pub mod move_gen;
pub mod position;
pub mod win_detection;

pub use minimax::best_move;
pub use position::{PlayError, Player, Position};

/// Clears the screen and runs the interactive terminal game loop.
pub fn run() {
    engine::game_start();
}
