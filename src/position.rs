use crate::board::Bitboard;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Player {
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

#[derive(Debug)]
struct CoOrdinate {
    x: usize,
    y: usize,
}

impl CoOrdinate {
    fn new(x: usize, y: usize) -> Self {
        CoOrdinate { x, y}
    }
}

pub struct Position {
    bitboards: [Bitboard; 2],
    heights: [usize; Position::WIDTH],
    player_to_move: Player
}

impl Position {
    const WIDTH: usize = 7;
    const HEIGHT: usize = 6;
    const MAX_MOVES: usize = Position::WIDTH * Position::HEIGHT;

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
        if column >= Self::WIDTH {
            println!("Invalid column index {column}.");
            return false
        }

        let height = self.heights[column];
        if height >= Self::HEIGHT {
            println!("Column {column} is full");
            return false
        }

        //if the column index is within bounds and the given column is not full then return valid state
        true
    }

    fn index_from_coord(&self, coord: CoOrdinate) -> u8 {
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

    pub fn play(&mut self, column: usize) {
        if self.can_play(column) {
            //update the current player's bitboard to record their move
            let player_index = self.player_to_move.index();
            let coord = CoOrdinate::new(column, self.heights[column]);
            let index = self.index_from_coord(coord);
            self.bitboards[player_index].set(index);

            //increment board height occupancy
            self.heights[column] += 1;
            //update player to move to next player
            self.player_to_move = self.player_to_move.other();


        } else {
            println!("Invalid move.");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Bitboard;
    use crate::position::{Player, Position};

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

        for turn in 0..(Position::HEIGHT + 4) {
            assert_eq!(pos.heights[0], turn.clamp(0,Position::HEIGHT));
            println!("{:?}", pos.heights[0]);
            pos.play(0);
        }
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
            pos.play(0);
        }
    }

    #[test]
    fn player_bitboard_is_updated() {
        let mut pos = Position::new();

        //Red's turn
        pos.play(2);
        assert_eq!(pos.get_bitboard(Player::Red), Bitboard::from_u64(0b100));

        //Blue's turn
        pos.play(2);
        assert_eq!(pos.get_bitboard(Player::Blue), Bitboard::from_u64(0x200));
    }

    #[test]
    fn can_play_any_column() {
        let mut pos = Position::new();

        for column in 0..(Position::WIDTH + 4) {
            pos.play(column);
        }

        for column in 0..Position::WIDTH {
            assert_eq!(pos.heights[column], 1);
        }
    }

}