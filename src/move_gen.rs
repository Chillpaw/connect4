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
