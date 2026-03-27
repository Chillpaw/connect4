use crate::engine::game_start;

mod board;
mod position;
mod win_detection;
mod engine;
mod move_gen;
mod minimax;

pub fn run() {
    game_start();
}

