use crate::board::Bitboard;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Player {
    Red,
    Blue
}

impl Player {
    /// Returns the opposing player.
    ///
    /// # Returns
    ///
    /// `Player::Blue` if `self` is `Player::Red`, `Player::Red` if `self` is `Player::Blue`.
    ///
    /// # Examples
    ///
    /// ```
    /// let p = Player::Red;
    /// assert_eq!(p.other(), Player::Blue);
    /// ```
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
    /// Creates a coordinate with the given x (column) and y (row) indices.
    ///
    /// The returned CoOrdinate holds the provided `x` and `y` values.
    ///
    /// # Examples
    ///
    /// ```
    /// let c = CoOrdinate::new(3, 2);
    /// assert_eq!(c.x, 3);
    /// assert_eq!(c.y, 2);
    /// ```
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

    /// Creates a new empty game position.
    ///
    /// The returned `Position` has both players' bitboards cleared, all column heights set to zero,
    /// and `Player::Red` set to move first.
    ///
    /// # Examples
    ///
    /// ```
    /// let pos = Position::new();
    /// assert_eq!(pos.player_to_move(), Player::Red);
    /// for col in 0..Position::WIDTH {
    ///     assert!(pos.can_play(col));
    /// }
    /// ```
    pub fn new() -> Self {
        Position {
            bitboards: [Bitboard::empty(); 2],
            heights: [0; Self::WIDTH],
            player_to_move: Player::Red,
        }
    }

    /// Get the player who is to move.
    ///
    /// # Examples
    ///
    /// ```
    /// let pos = Position::new();
    /// assert_eq!(pos.player_to_move(), Player::Red);
    /// ```
    ///
    /// # Returns
    ///
    /// `Player` representing the player whose turn it is.
    pub fn player_to_move(&self) -> Player {
        self.player_to_move
    }

    /// Returns the bitboard belonging to the player whose turn it is.
    ///
    /// # Returns
    ///
    /// `Bitboard` for the player whose turn it is.
    ///
    /// # Examples
    ///
    /// ```
    /// let pos = Position::new();
    /// // At start Red moves first; the bitboard for Red is returned.
    /// assert_eq!(pos.player_to_move(), Player::Red);
    /// let bb = pos.get_bitboard();
    /// // New position has an empty bitboard (no pieces played).
    /// assert_eq!(bb, 0);
    /// ```
    pub fn get_bitboard(&self) -> Bitboard {
        let player = self.player_to_move();
        match player {
            Player::Red => self.bitboards[0],
            Player::Blue => self.bitboards[1]
        }
    }

    /// Returns whether a move can be played in the given column.
    ///
    /// Validates that `column` is within the range 0..WIDTH and that the column is not full.
    ///
    /// # Parameters
    /// - `column`: Column index where the move would be played; valid values are 0 through `WIDTH - 1`.
    ///
    /// # Returns
    /// `true` if the column index is valid and the column is not full, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let pos = Position::new();
    /// assert!(pos.can_play(0));
    /// assert!(!pos.can_play(Position::WIDTH));
    /// ```
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

    /// Compute the flat bit index in the bitboard corresponding to a board coordinate.
    ///
    /// The index maps a (x, y) coordinate into the single-dimensional bitboard layout used
    /// by Position, using the formula `(y * WIDTH - x + 1)`. The result is the bit position
    /// where a piece at the given coordinate should be set.
    ///
    /// # Returns
    ///
    /// The computed bit index as a `u8`.
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

    /// Applies a move for the current player in the given column, updating the board state and
    /// switching the active player. If the column is invalid or full, prints "Invalid move." and
    /// leaves the position unchanged.
    ///
    /// # Parameters
    ///
    /// - `column`: Column index where the current player attempts to play; valid values are
    ///   `0..Position::WIDTH`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::position::{Position, Player};
    ///
    /// let mut pos = Position::new();
    /// assert_eq!(pos.player_to_move(), Player::Red);
    /// pos.play(0);
    /// assert_eq!(pos.player_to_move(), Player::Blue);
    /// ```
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