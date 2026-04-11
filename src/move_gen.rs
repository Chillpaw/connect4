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

/// Bitmask of legal columns (indexed by column); used only by unit tests in this module.
#[cfg(test)]
fn valid_moves(position: Position) -> [bool; Position::WIDTH] {
    let mut out = [false; Position::WIDTH];
    for col in 0..Position::WIDTH {
        if position.can_play(col) {
            out[col] = true;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_moves_valid_on_empty_board() {
        let pos = Position::new();
        let moves = valid_moves(pos);
        assert!(moves.iter().all(|&m| m));
    }

    #[test]
    fn valid_moves_has_correct_length() {
        let pos = Position::new();
        let moves = valid_moves(pos);
        assert_eq!(moves.len(), Position::WIDTH);
    }

    #[test]
    fn full_column_shows_as_invalid() {
        let mut pos = Position::new();
        for _ in 0..Position::HEIGHT {
            pos.try_play(3).unwrap();
        }
        let moves = valid_moves(pos);
        assert!(!moves[3]);
        for col in 0..Position::WIDTH {
            if col != 3 {
                assert!(moves[col], "column {col} should still be valid");
            }
        }
    }

    #[test]
    fn all_columns_full_no_valid_moves() {
        let mut pos = Position::new();
        for _ in 0..Position::HEIGHT {
            for col in 0..Position::WIDTH {
                let _ = pos.try_play(col);
            }
        }
        let moves = valid_moves(pos);
        assert!(moves.iter().all(|&m| !m));
    }
}
