use std::fmt;
use std::fmt::Formatter;
use crate::board::Bitboard;

/// Side to move or disc colour.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Player {
    Red,
    Blue
}

impl Player {
    pub fn other(self) -> Self {
        match self {
            Player::Red => Player::Blue,
            Player::Blue => Player::Red
        }
    }

    pub fn index(&self) ->  usize {
        match self {
            Player::Red => 0,
            Player::Blue => 1,
        }
    }
}

/// Returned when a disc cannot be placed in the requested column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayError {
    ColumnOutOfBounds,
    ColumnFull,
}

#[derive(Debug)]
pub struct CoOrdinate {
    x: usize,
    y: usize,
}

impl CoOrdinate {
    fn new(x: usize, y: usize) -> Self {
        CoOrdinate { x, y}
    }
}

#[derive(Copy, Clone)]
pub struct Position {
    bitboards: [Bitboard; 2],
    heights: [usize; Position::WIDTH],
    pub(crate) player_to_move: Player
}

impl Position {
    pub(crate) const WIDTH: usize = 7;
    pub(crate) const HEIGHT: usize = 6;
    const MAX_MOVES: usize = Position::WIDTH * Position::HEIGHT;
    const FULL_BOARD: u64 = (1u64 << (Position::WIDTH * Position::HEIGHT)) - 1;

    const fn edge_mask(col: usize) -> u64 {
        let mut mask = 0u64;
        let mut bit = 0;
        while bit < Self::WIDTH * Self::HEIGHT {
            if bit % Self::WIDTH != col {
                mask |= 1u64 << bit;
            }
            bit += 1;
        }
        mask
    }

    pub const NOT_RIGHT_EDGE: u64 = Self::edge_mask(Self::WIDTH - 1);
    pub const NOT_LEFT_EDGE: u64 = Self::edge_mask(0);

    pub fn new() -> Self {
        Position {
            bitboards: [Bitboard::empty(); 2],
            heights: [0; Self::WIDTH],
            player_to_move: Player::Red,
        }
    }

    pub fn player_to_move(&self) -> Player {
        self.player_to_move
    }

    pub fn get_bitboard(&self, player: Player) -> Bitboard {
        self.bitboards[player.index()]
    }

    pub fn can_play(&self, column: usize) -> bool {
        column < Self::WIDTH && self.heights[column] < Self::HEIGHT
    }

    pub fn index_from_coord(&self, coord: CoOrdinate) -> u8 {
        // the bitboard index is determined by the x and y position of the target
        // this is calculated by wrapping the game grid around the width to determine the flat index of the bitmask

        // (x: 3, y: 1) would become
        // 0 0 0 0 0 0 0 20
        // 0 0 0 1 0 0 0 13
        // 0 0 0 0 0 0 0 6
        // expected index = 10

        // (x: 5, y: 2) would become
        // 0 1 0 0 0 0 0 20
        // 0 0 0 0 0 0 0 13
        // 0 0 0 0 0 0 0 6
        // expected index = 19

        (coord.y * Self::WIDTH + coord.x) as u8
    }

    /// Places a disc for the current player in `column`, or returns why the move is illegal.
    pub fn try_play(&mut self, column: usize) -> Result<(), PlayError> {
        if column >= Self::WIDTH {
            return Err(PlayError::ColumnOutOfBounds);
        }
        if self.heights[column] >= Self::HEIGHT {
            return Err(PlayError::ColumnFull);
        }

        let player_index = self.player_to_move.index();
        let coord = CoOrdinate::new(column, self.heights[column]);
        let index = self.index_from_coord(coord);
        self.bitboards[player_index].set(index);
        self.heights[column] += 1;
        self.player_to_move = self.player_to_move.other();
        Ok(())
    }

    pub fn board_full(&self) -> bool {
        let red_board = self.bitboards[0];
        let blue_board = self.bitboards[1];
        (red_board | blue_board) == Bitboard::from_u64(Self::FULL_BOARD)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // show consolidated game board with B for blue and R for red
        for y in (0..Position::HEIGHT).rev() {
            for x in 0..Position::WIDTH {
                let index = y * Position::WIDTH + x;
                write!(f, "{}", if self.bitboards[0].is_set(index as u8)
                    {"R "}
                else if self.bitboards[1].is_set(index as u8)
                    {"B "}
                else
                    {". "})?;
            }
            writeln!(f)?;
        }
        // show who the current player is
        writeln!(f, "Current player: {:?}", self.player_to_move())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Bitboard;
    use crate::position::*;

    #[test]
    fn new_position_is_empty() {
        let pos = Position::new();

        assert_eq!(pos.player_to_move, Player::Red);

        for column in pos.heights.iter() {
            assert_eq!(*column, 0);
        }
    }

    #[test]
    fn add_to_column() {
        let mut pos = Position::new();

        for turn in 0..Position::HEIGHT {
            assert_eq!(pos.heights[0], turn);
            assert!(pos.try_play(0).is_ok());
        }
        assert_eq!(pos.heights[0], Position::HEIGHT);
        assert_eq!(pos.try_play(0), Err(PlayError::ColumnFull));
    }

    #[test]
    fn player_other_changes_players() {
        let red = Player::Red;
        let blue = Player::Blue;

        assert_eq!(red.other(), Player::Blue);
        assert_eq!(blue.other(), Player::Red);
        assert_eq!(red.other().other(), Player::Red);
    }

    #[test]
    fn can_play_valid_column() {
        let pos = Position::new();

        for column in 0..Position::WIDTH {
            assert!(pos.can_play(column));
        }
    }

    #[test]
    fn can_not_play_invalid_column() {
        let pos = Position::new();

        assert!(!pos.can_play(Position::WIDTH));
        assert!(!pos.can_play(Position::WIDTH + 1));
        assert!(!pos.can_play(50));
    }

    #[test]
    fn player_advances() {
        let mut pos = Position::new();

        for turn in 0..4 {
            if turn % 2 == 0 { //test that play is alternating between players each turn
                assert_eq!(pos.player_to_move.index(), 0); //first turn Red
            } else {
                assert_eq!(pos.player_to_move.index(), 1); //second turn Blue
            }
            pos.try_play(0).unwrap();
        }
    }

    #[test]
    fn player_bitboard_is_updated() {
        let mut pos = Position::new();

        //Red's turn
        pos.try_play(2).unwrap();
        assert_eq!(pos.get_bitboard(Player::Red), Bitboard::from_u64(0b100));

        //Blue's turn
        pos.try_play(2).unwrap();
        assert_eq!(pos.get_bitboard(Player::Blue), Bitboard::from_u64(0x200));
    }

    #[test]
    fn can_play_any_column() {
        let mut pos = Position::new();

        for column in 0..(Position::WIDTH + 4) {
            let _ = pos.try_play(column);
        }

        for column in 0..Position::WIDTH {
            assert_eq!(pos.heights[column], 1);
        }
    }

    #[test]
    fn full_board() {
        let mut pos = Position::new();

        pos.bitboards[0] = Bitboard::from_u64(0x5555555555555555) & Bitboard::from_u64(Position::FULL_BOARD);
        pos.bitboards[1] = Bitboard::from_u64(0xAAAAAAAAAAAAAAAA) & Bitboard::from_u64(Position::FULL_BOARD);
        println!("{}", pos.bitboards[0]);
        println!("{}", pos.bitboards[1]);
        println!("{}", (pos.bitboards[0] | pos.bitboards[1]));
        assert!(pos.board_full())
    }

}