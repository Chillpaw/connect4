use crate::board::Bitboard;
use crate::position::{Player, Position};

pub enum GameState {
    InProgress,
    Won(Player),
    Draw,
}

impl GameState {
    pub fn new() -> Self {
        GameState::InProgress
    }
}

pub fn is_win(bitboard: Bitboard) -> bool {
    horizontal_win(bitboard) || vertical_win(bitboard) || diag_left_win(bitboard) || diag_right_win(bitboard)
}

fn horizontal_win(b: Bitboard) -> bool {
    let m = b & Bitboard::from_u64(Position::NOT_RIGHT_EDGE);
    let pairs = m & (m >> 1);
    (pairs & (pairs >> 2)).is_not_empty()
}

fn vertical_win(b: Bitboard) -> bool {
    let pairs = b & (b >> Position::WIDTH as u8);
    (pairs & (pairs >> Position::WIDTH as u8 * 2)).is_not_empty()
}

fn diag_left_win(b: Bitboard) -> bool {
    let m = b & Bitboard::from_u64(Position::NOT_LEFT_EDGE);
    let offset = Position::WIDTH as u8 - 1;
    let pairs = m & (m >> offset);

    (pairs & (pairs >> (2 * offset))).is_not_empty()
}

fn diag_right_win(b: Bitboard) -> bool {
    let m = b & Bitboard::from_u64(Position::NOT_RIGHT_EDGE);
    let offset = Position::WIDTH as u8 + 1;
    let pairs = m & (m >> offset);

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

    }

    #[test]
    fn horizontal_win() {
        let mut b = Bitboard::empty();

        for index in 0..4 {
            b.set(index);
        }
        println!("{}", b);
        assert!(is_win(b));
    }

    #[test]
    fn vertical_win() {
        let mut b = Bitboard::empty();
        let width = Position::WIDTH as u8;
        for index in 0..4 {
            b.set(index * width)
        }
        println!("{}", b);
        assert!(is_win(b))
    }

    #[test]
    fn diag_right_win() {
        let mut b = Bitboard::empty();
        let width = Position::WIDTH as u8;
        for index in 0..4 {
            b.set(index + width * index);
        }
        println!("{}", b);
        assert!(is_win(b))
    }

    #[test]
    fn diag_left_win() {
        let mut b = Bitboard::empty();
        let width = Position::WIDTH as u8;
        for index in 0..4 {
            b.set((index + 1) * (width - 1));
        }
        println!("{}", b);
        assert!(is_win(b))
    }
}





