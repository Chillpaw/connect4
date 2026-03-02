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

    pub fn get_bitboard(&self) -> Bitboard {
        let player = self.player_to_move();
        match player {
            Player::Red => self.bitboards[0],
            Player::Blue => self.bitboards[1]
        }
    }

    pub fn can_play(&self, column: usize) -> bool {
        if column > Self::WIDTH {
            println!("Invalid column index.");
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

        // (x: 4, y: 2) would become
        // 0 0 0 0 0 0 0
        // 0 0 0 1 0 0 0
        // 0 0 0 0 0 0 0
        // expected index = 11

        // (x: 6, y: 3) would become
        // 0 0 0 0 0 1 0
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // expected index = 16

        (coord.y * Self::WIDTH - coord.x + 1) as u8
    }

    pub fn play(&mut self, column: usize) {
        if self.can_play(column) {
            //update the current player's bitboard to record their move
            let mut b = self.get_bitboard();
            let coord = CoOrdinate::new(column, self.heights[column]);
            let index = self.index_from_coord(coord);
            b.set(index);

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
    use crate::position::{Player, Position};

    #[test]
    fn new_position_is_empty() {
        let pos = Position::new();

        assert_eq!(pos.player_to_move, Player::Red);

        for column in pos.heights.iter() {
            assert_eq!(pos.heights[*column], 0);
        }
    }
}