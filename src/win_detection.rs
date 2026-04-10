//! Four-in-a-row detection for a **single** player's [`Bitboard`].

use crate::board::Bitboard;
use crate::position::Position;

/// `true` if `bitboard` contains four connected discs in any line (horizontal, vertical, or diagonal).
pub fn is_win(bitboard: Bitboard) -> bool {
    horizontal_win(bitboard) || vertical_win(bitboard) || diag_left_win(bitboard) || diag_right_win(bitboard)
}

fn horizontal_win(b: Bitboard) -> bool {
    let m = b & Bitboard::from_u64(Position::NOT_RIGHT_EDGE);
    let pairs = m & (b >> 1);
    (pairs & (pairs >> 2)).is_not_empty()
}

fn vertical_win(b: Bitboard) -> bool {
    let pairs = b & (b >> Position::WIDTH as u8);
    (pairs & (pairs >> Position::WIDTH as u8 * 2)).is_not_empty()
}

fn diag_left_win(b: Bitboard) -> bool {
    let m = b & Bitboard::from_u64(Position::NOT_LEFT_EDGE);
    let offset = Position::WIDTH as u8 - 1;
    let pairs = m & (b >> offset);

    (pairs & (pairs >> (2 * offset))).is_not_empty()
}

fn diag_right_win(b: Bitboard) -> bool {
    let m = b & Bitboard::from_u64(Position::NOT_RIGHT_EDGE);
    let offset = Position::WIDTH as u8 + 1;
    let pairs = m & (b >> offset);

    (pairs & (pairs >> (2 * offset))).is_not_empty()
}

#[cfg(test)]
mod tests {
    use crate::board::Bitboard;
    use crate::position::Position;
    use crate::win_detection::is_win;

    #[test]
    fn empty_board_no_win() {
        let b = Bitboard::empty();
        assert!(!is_win(b))
    }

    #[test]
    fn three_in_a_row_no_win() {
        let b = Bitboard::from_u64(0x38);
        println!("{}", b);
        assert!(!is_win(b));
    }

    #[test]
    fn horizontal_win_bottom_right() {
        let mut b = Bitboard::empty();

        for index in 0..4 {
            b.set(index);
        }
        println!("{}", b);
        assert!(is_win(b));
    }

    #[test]
    fn horizontal_win_bottom_left() {
        let b = Bitboard::from_u64(0x78);
        println!("{}", b);
        assert!(is_win(b));
    }

    #[test]
    fn horizontal_win_top_right() {
        let b = Bitboard::from_u64(0x3c000000000);
        println!("{}", b);
        assert!(is_win(b));
    }

    #[test]
    fn horizontal_win_top_left() {
        let b = Bitboard::from_u64(0x07800000000);
        println!("{}", b);
        assert!(is_win(b));
    }

    #[test]
    fn vertical_win_right_edge() {
        let mut b = Bitboard::empty();
        let width = Position::WIDTH as u8;
        for index in 0..4 {
            b.set(index * width)
        }
        println!("{}", b);
        assert!(is_win(b))
    }

    #[test]
    fn vertical_win_left_edge() {
        let b = Bitboard::from_u64(0x20408100000);
        println!("{}", b);
        assert!(is_win(b))
    }

    #[test]
    fn diag_win_bottom_right() {
        let mut b = Bitboard::empty();
        let width = Position::WIDTH as u8;
        for index in 0..4 {
            b.set(index + width * index);
        }
        println!("{}", b);
        assert!(is_win(b))
    }

    #[test]
    fn diag_win_top_right() {
        let b = Bitboard::from_u64(0x00820820000);
        println!("{}", b);
        assert!(is_win(b))
    }

    #[test]
    fn diag_win_bottom_left() {
        let mut b = Bitboard::empty();
        let width = Position::WIDTH as u8;
        for index in 0..4 {
            b.set((index + 1) * (width - 1));
        }
        println!("{}", b);
        assert!(is_win(b))
    }

    #[test]
    fn diag_win_top_left() {
        // 1 0|0 0 0 0|0
        // 0 1 0|0 0 0 0
        // 0 0 1 0|0 0 0
        // 0|0 0 1 0|0 0
        // 0 0|0 0 0 0|0
        // 0 0 0|0 0 0 0
        let b = Bitboard::from_u64(0x20202020000);
        println!("{}", b);
        assert!(is_win(b))
    }
}





