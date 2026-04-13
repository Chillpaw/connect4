<<<<<<< minimax-implementation
//! Minimax search with alpha–beta pruning and a light heuristic based on open pairs of discs.
//!
//! Scores are always from the perspective of the player chosen as `perspective` at the root
//! (the side to move when [`best_move`](fn@best_move) is called).

use crate::board::Bitboard;
use crate::move_gen::legal_columns_ordered;
use crate::position::{Player, Position};
use crate::win_detection::is_win;

const WINNING_SCORE: f32 = 1.0;
const LOSING_SCORE: f32 = -1.0;
const DRAW_SCORE: f32 = 0.0;
const PAIR_WEIGHT_SELF: f32 = 0.00001;
const PAIR_WEIGHT_OPP: f32 = 0.000012;

/// Returns a strong legal move for the side to move, searching `depth` plies from each child.
///
/// `depth` counts full plies to search **after** making a candidate move (the child position is
/// evaluated with `depth - 1`, and so on). With `depth == 0`, only the heuristic is used on
/// resulting positions.
///
/// Returns [`None`] only when there are no legal moves.
pub fn best_move(pos: &Position, depth: usize) -> Option<usize> {
    let root = pos.player_to_move();
    let mut best: Option<usize> = None;
    let mut best_score = f32::NEG_INFINITY;

    for col in legal_columns_ordered(pos) {
        let mut child = *pos;
        if child.try_play(col).is_err() {
            continue;
        }
        let score = search(
            child,
            depth.saturating_sub(1),
            f32::NEG_INFINITY,
            f32::INFINITY,
            root,
        );
        if score > best_score || best.is_none() {
            best_score = score;
            best = Some(col);
        }
    }

    best
}

/// Static evaluation and recursive minimax with alpha–beta pruning.
fn search(
    pos: Position,
    depth: usize,
    mut alpha: f32,
    mut beta: f32,
    perspective: Player,
) -> f32 {
    if let Some(s) = terminal_score(&pos, perspective) {
        return s;
    }
    if depth == 0 {
        return heuristic_eval(&pos, perspective);
    }

    let to_move = pos.player_to_move();
    let cols: Vec<usize> = legal_columns_ordered(&pos).collect();

    if to_move == perspective {
        let mut value = f32::NEG_INFINITY;
        for col in cols {
            let mut p = pos;
            if p.try_play(col).is_err() {
                continue;
            }
            let score = search(p, depth - 1, alpha, beta, perspective);
            value = value.max(score);
            alpha = alpha.max(value);
            if beta <= alpha {
                break;
            }
        }
        value
    } else {
        let mut value = f32::INFINITY;
        for col in cols {
            let mut p = pos;
            if p.try_play(col).is_err() {
                continue;
            }
            let score = search(p, depth - 1, alpha, beta, perspective);
            value = value.min(score);
            beta = beta.min(value);
            if beta <= alpha {
                break;
            }
        }
        value
    }
}

/// Win / loss / draw from `perspective` if the game has ended; otherwise [`None`].
fn terminal_score(pos: &Position, perspective: Player) -> Option<f32> {
    let red = pos.get_bitboard(Player::Red);
    let blue = pos.get_bitboard(Player::Blue);

    if is_win(red) {
        return Some(if perspective == Player::Red {
            WINNING_SCORE
        } else {
            LOSING_SCORE
        });
    }
    if is_win(blue) {
        return Some(if perspective == Player::Blue {
            WINNING_SCORE
        } else {
            LOSING_SCORE
        });
    }
    if pos.board_full() {
        return Some(DRAW_SCORE);
    }
    None
}

fn occupied_mask(pos: &Position) -> Bitboard {
    pos.get_bitboard(Player::Red) | pos.get_bitboard(Player::Blue)
}

/// Heuristic leaf evaluation: favours open pairs for `perspective` and penalises the opponent's.
fn heuristic_eval(pos: &Position, perspective: Player) -> f32 {
    let occ = occupied_mask(pos);
    let empties = (!occ) & Bitboard::from_u64(Position::FULL_BOARD);
    let mine = pos.get_bitboard(perspective);
    let opp = pos.get_bitboard(perspective.other());

    let pair_mine = find_pairs(mine, empties) as f32;
    let pair_opp = find_pairs(opp, empties) as f32;

    pair_mine * PAIR_WEIGHT_SELF - pair_opp * PAIR_WEIGHT_OPP
}

/// Counts two-disc patterns that can extend toward an empty cell (used by [`heuristic_eval`]).
fn find_pairs(b: Bitboard, empties: Bitboard) -> u32 {
    let mut pairs_count = 0;
    pairs_count += count_horizontal_pairs(b, empties);
    pairs_count += count_vertical_pairs(b, empties);
    pairs_count += count_diag_left_pairs(b, empties);
    pairs_count += count_diag_right_pairs(b, empties);
    pairs_count
}

fn count_horizontal_pairs(b: Bitboard, empties: Bitboard) -> u32 {
    let m = b & Bitboard::from_u64(Position::NOT_RIGHT_EDGE);
    let pairs = m & (b >> 1);
    let mut pairs_count = 0;
    pairs_count += ((pairs << 2) & empties & Bitboard::from_u64(Position::NOT_LEFT_EDGE)).count();
    pairs_count += ((pairs >> 1) & empties).count();
    pairs_count
}

fn count_vertical_pairs(b: Bitboard, empties: Bitboard) -> u32 {
    let width = Position::WIDTH as u8;
    let pairs = b & (b >> width);
    (pairs & (empties >> (width * 2))).count()
}

fn count_diag_left_pairs(b: Bitboard, empties: Bitboard) -> u32 {
    let mask = Bitboard::from_u64(Position::NOT_LEFT_EDGE);
    let m = b & mask;
    let offset = Position::WIDTH as u8 - 1;
    let pairs = m & (b >> offset);
    let one_step_ahead = pairs & (empties >> offset * 2);
    let two_steps_ahead = pairs & (empties >> offset * 3);
    (pairs & one_step_ahead & two_steps_ahead).count()
}

fn count_diag_right_pairs(b: Bitboard, empties: Bitboard) -> u32 {
    let mask = Bitboard::from_u64(Position::NOT_RIGHT_EDGE);
    let m = b & mask;
    let offset = Position::WIDTH as u8 + 1;
    let pairs = m & (b >> offset);
    let one_step_ahead = pairs & (empties >> offset * 2);
    let two_steps_ahead = pairs & (empties >> offset * 3);
    let one_step_behind = pairs & (empties << offset * 2);
    ((one_step_ahead & two_steps_ahead) | (one_step_behind & one_step_ahead)).count()
}

#[cfg(test)]
mod tests {
    use super::{best_move, find_pairs};
    use crate::board::Bitboard;
    use crate::position::{Player, Position};

    #[test]
    fn best_move_finds_immediate_win() {
        let mut pos = Position::new();
        pos.try_play(0).unwrap();
        pos.try_play(0).unwrap();
        pos.try_play(1).unwrap();
        pos.try_play(1).unwrap();
        pos.try_play(2).unwrap();
        pos.try_play(2).unwrap();
        assert_eq!(pos.player_to_move(), Player::Red);
        assert_eq!(best_move(&pos, 8), Some(3));
    }

    #[test]
    fn best_move_opening_returns_legal_center_bias() {
        let pos = Position::new();
        let col = best_move(&pos, 6).expect("opening has moves");
        assert!(pos.can_play(col));
        assert_eq!(col, 3);
    }

    #[test]
    fn pair_found_horizontal() {
        let player_board = Bitboard::from_u64(0x18);
        let empty_board = !player_board;
        assert_eq!(find_pairs(player_board, empty_board), 2);
    }

    #[test]
    fn pairs_not_found_horizontal() {
        let player_board = Bitboard::from_u64(0x18);
        let empty_board = Bitboard::from_u64(0x5b);
        assert_eq!(find_pairs(player_board, empty_board), 0);
    }

    #[test]
    fn pair_found_horizontal_left_edge() {
        let player_board = Bitboard::from_u64(0x60);
        let empty_board = !player_board;
        assert_eq!(find_pairs(player_board, empty_board), 1);
    }

    #[test]
    fn pair_found_horizontal_right_edge() {
        let player_board = Bitboard::from_u64(0x3);
        let empty_board = !player_board;
        assert_eq!(find_pairs(player_board, empty_board), 1);
    }

    #[test]
    fn pair_found_vertical_central() {
        let player_board = Bitboard::from_u64(0x408);
        let empty_board = !player_board;
        assert_eq!(find_pairs(player_board, empty_board), 1);
    }

    #[test]
    fn pair_found_vertical_left_edge() {
        let player_board = Bitboard::from_u64(0x2040);
        let empty_board = !player_board;
        assert_eq!(find_pairs(player_board, empty_board), 1);
    }

    #[test]
    fn pair_found_vertical_right_edge() {
        let player_board = Bitboard::from_u64(0x081);
        let empty_board = !player_board;
        assert_eq!(find_pairs(player_board, empty_board), 1);
    }

    #[test]
    fn pair_found_diag_bottom_left() {
        let player_board = Bitboard::from_u64(0x1040);
        let empty_board = !player_board;
        assert_eq!(find_pairs(player_board, empty_board), 1);
    }

    #[test]
    fn pair_found_diag_bottom_right() {
        let player_board = Bitboard::from_u64(0x101);
        let empty_board = !player_board;
        assert_eq!(find_pairs(player_board, empty_board), 1);
    }

    #[test]
    fn pair_found_diag_top_left() {
        let player_board = Bitboard::from_u64(0x20200000000u64);
        let empty_board = !player_board;
        assert_eq!(find_pairs(player_board, empty_board), 1);
    }

    #[test]
    fn pair_found_diag_top_right() {
        let player_board = Bitboard::from_u64(0x820000000u64);
        let empty_board = !player_board;
        assert_eq!(find_pairs(player_board, empty_board), 1);
    }
}
=======
// Minimax algorithm implementation (stub)
>>>>>>> main
