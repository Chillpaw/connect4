use crate::position::Position;

fn valid_moves(position: Position) -> [bool; Position::WIDTH] {
    let mut valid_moves = [false; Position::WIDTH];

    for col in 0..Position::WIDTH {
        if Position::can_play(&position, col) {
            valid_moves[col] = true;
        }
    }

    valid_moves
}

