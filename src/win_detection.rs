use crate::board::Bitboard;
use crate::position::{Player, Position};



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

    #[test]
    fn vertical_three_in_a_row_no_win() {
        let mut b = Bitboard::empty();
        let width = Position::WIDTH as u8;
        // 3 vertical pieces in column 3 — not enough for a win
        b.set(3);
        b.set(3 + width);
        b.set(3 + 2 * width);
        assert!(!is_win(b));
    }

    #[test]
    fn diagonal_three_in_a_row_no_win() {
        let mut b = Bitboard::empty();
        let width = Position::WIDTH as u8;
        // 3 diagonal pieces (/) starting at column 1, row 0
        b.set(1);
        b.set(1 + 1 + width);
        b.set(1 + 2 + 2 * width);
        assert!(!is_win(b));
    }

    #[test]
    fn vertical_win_middle_column() {
        let mut b = Bitboard::empty();
        let width = Position::WIDTH as u8;
        // 4 vertical pieces in column 3 (middle column)
        for row in 0..4u8 {
            b.set(3 + row * width);
        }
        println!("{}", b);
        assert!(is_win(b));
    }

    #[test]
    fn horizontal_win_middle_row() {
        let mut b = Bitboard::empty();
        let width = Position::WIDTH as u8;
        // 4 horizontal pieces in row 3, columns 1-4
        for col in 1..5u8 {
            b.set(col + 3 * width);
        }
        println!("{}", b);
        assert!(is_win(b));
    }

    #[test]
    fn pieces_with_gap_no_win() {
        let mut b = Bitboard::empty();
        // Bits 0, 1, 3, 4 — gap at bit 2 breaks any 4-in-a-row
        b.set(0);
        b.set(1);
        b.set(3);
        b.set(4);
        assert!(!is_win(b));
    }

    #[test]
    fn single_piece_no_win() {
        let mut b = Bitboard::empty();
        b.set(0);
        assert!(!is_win(b));
    }
}





