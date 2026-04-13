<<<<<<< minimax-implementation
//! Legal moves for a [`Position`], ordered for search (center columns first).

use crate::position::Position;

/// Column indices from center outward; strong default move ordering for alpha–beta.
const COLUMN_ORDER: [usize; Position::WIDTH] = [3, 2, 4, 1, 5, 0, 6];

/// Yields legal columns in center-out order (see `COLUMN_ORDER` in this module).
pub fn legal_columns_ordered(pos: &Position) -> impl Iterator<Item = usize> + '_ {
    COLUMN_ORDER
        .iter()
        .copied()
        .filter(move |&c| pos.can_play(c))
}

=======
use crate::position::Position;

>>>>>>> main
/// Identify which board columns are playable for a position.
///
/// # Returns
///
/// An array of length `Position::WIDTH` where each element is `true` if the corresponding column can be played, `false` otherwise.
///
/// # Examples
///
/// ```
/// use connect4_core::position::Position;
/// use connect4_core::move_gen::valid_moves;
///
/// let pos = Position::new();
/// let moves = valid_moves(&pos);
/// assert!(moves.iter().all(|&b| b)); // empty board: every column is playable
/// ```
pub fn valid_moves(position: &Position) -> [bool; Position::WIDTH] {
    let mut valid_moves = [false; Position::WIDTH];

    for col in 0..Position::WIDTH {
        if position.can_play(col) {
            valid_moves[col] = true;
        }
    }

    valid_moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_moves_valid_on_empty_board() {
        let pos = Position::new();
        let moves = valid_moves(&pos);
        assert!(moves.iter().all(|&m| m));
    }

    #[test]
    fn valid_moves_has_correct_length() {
        let pos = Position::new();
        let moves = valid_moves(&pos);
        assert_eq!(moves.len(), Position::WIDTH);
    }

    #[test]
    fn full_column_shows_as_invalid() {
        let mut pos = Position::new();
        // Fill column 3 completely
        for _ in 0..Position::HEIGHT {
            pos.play(3);
        }
        let moves = valid_moves(&pos);
        assert!(!moves[3]);
        // All other columns should still be valid
        for col in 0..Position::WIDTH {
            if col != 3 {
                assert!(moves[col], "column {col} should still be valid");
            }
        }
    }

    #[test]
    fn all_columns_full_no_valid_moves() {
        let mut pos = Position::new();
        // Fill every column
        for _ in 0..Position::HEIGHT {
            for col in 0..Position::WIDTH {
                pos.play(col);
            }
        }
        let moves = valid_moves(&pos);
        assert!(moves.iter().all(|&m| !m));
    }
}
