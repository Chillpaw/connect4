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
            let coord = CoOrdinate::new(column, self.heights[column]);
            let index = self.index_from_coord(coord);

            let player_index = match self.player_to_move {
                Player::Red => 0,
                Player::Blue => 1
            };
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
    use super::*;

    #[test]
    fn new_position_is_empty() {
        let pos = Position::new();

        assert_eq!(pos.player_to_move, Player::Red);

        for column in 0..Position::WIDTH {
            assert_eq!(pos.heights[column], 0);
        }
    }

    #[test]
    fn player_to_move_starts_with_red() {
        let pos = Position::new();
        assert_eq!(pos.player_to_move(), Player::Red);
    }

    #[test]
    fn player_other_switches_players() {
        let red = Player::Red;
        let blue = Player::Blue;

        assert_eq!(red.other(), Player::Blue);
        assert_eq!(blue.other(), Player::Red);
        assert_eq!(red.other().other(), Player::Red);
    }

    #[test]
    fn can_play_valid_column() {
        let pos = Position::new();

        for col in 0..Position::WIDTH {
            assert!(pos.can_play(col), "Column {col} should be playable");
        }
    }

    #[test]
    fn can_play_invalid_column_out_of_bounds() {
        let pos = Position::new();

        assert!(!pos.can_play(Position::WIDTH));
        assert!(!pos.can_play(Position::WIDTH + 1));
        assert!(!pos.can_play(100));
    }

    #[test]
    fn can_play_full_column() {
        let mut pos = Position::new();

        // Fill column 0 to the top
        for _ in 0..Position::HEIGHT {
            pos.play(0);
        }

        // Column 0 should now be full
        assert!(!pos.can_play(0));
    }

    #[test]
    fn play_updates_height() {
        let mut pos = Position::new();

        assert_eq!(pos.heights[3], 0);
        pos.play(3);
        assert_eq!(pos.heights[3], 1);
        pos.play(3);
        assert_eq!(pos.heights[3], 2);
    }

    #[test]
    fn play_switches_player() {
        let mut pos = Position::new();

        assert_eq!(pos.player_to_move(), Player::Red);
        pos.play(0);
        assert_eq!(pos.player_to_move(), Player::Blue);
        pos.play(0);
        assert_eq!(pos.player_to_move(), Player::Red);
    }

    #[test]
    fn play_invalid_column_does_not_change_state() {
        let mut pos = Position::new();
        let initial_player = pos.player_to_move();

        pos.play(Position::WIDTH); // Invalid column

        // State should remain unchanged
        assert_eq!(pos.player_to_move(), initial_player);
        for col in 0..Position::WIDTH {
            assert_eq!(pos.heights[col], 0);
        }
    }

    #[test]
    fn play_full_column_does_not_change_state() {
        let mut pos = Position::new();

        // Fill column 0
        for _ in 0..Position::HEIGHT {
            pos.play(0);
        }

        let heights_before = pos.heights;
        let player_before = pos.player_to_move();

        // Try to play in full column
        pos.play(0);

        // Heights should not change
        assert_eq!(pos.heights, heights_before);
        // Player should not switch
        assert_eq!(pos.player_to_move(), player_before);
    }

    #[test]
    fn index_from_coord_bottom_left() {
        let pos = Position::new();
        let coord = CoOrdinate::new(0, 0);
        let index = pos.index_from_coord(coord);
        assert_eq!(index, 1);
    }

    #[test]
    fn index_from_coord_various_positions() {
        let pos = Position::new();

        // Test case from comments: (x: 4, y: 2) -> index 11
        let coord1 = CoOrdinate::new(4, 2);
        let index1 = pos.index_from_coord(coord1);
        assert_eq!(index1, 11, "Position (4, 2) should map to index 11");

        // Test case from comments: (x: 6, y: 3) -> index 16
        let coord2 = CoOrdinate::new(6, 3);
        let index2 = pos.index_from_coord(coord2);
        assert_eq!(index2, 16, "Position (6, 3) should map to index 16");
    }

    #[test]
    fn index_from_coord_top_right() {
        let pos = Position::new();
        let coord = CoOrdinate::new(Position::WIDTH - 1, Position::HEIGHT - 1);
        let index = pos.index_from_coord(coord);
        // Top right corner: (6, 5) -> 5*7 - 6 + 1 = 35 - 6 + 1 = 30
        assert_eq!(index, 30);
    }

    #[test]
    fn get_bitboard_returns_red_bitboard_initially() {
        let pos = Position::new();
        let bitboard = pos.get_bitboard();

        // Initially, the red player's bitboard should be empty
        assert_eq!(bitboard.to_u64(), 0);
    }

    #[test]
    fn multiple_plays_different_columns() {
        let mut pos = Position::new();

        // Red plays column 0
        pos.play(0);
        assert_eq!(pos.heights[0], 1);
        assert_eq!(pos.heights[1], 0);

        // Blue plays column 1
        pos.play(1);
        assert_eq!(pos.heights[0], 1);
        assert_eq!(pos.heights[1], 1);

        // Red plays column 2
        pos.play(2);
        assert_eq!(pos.heights[2], 1);
    }

    #[test]
    fn play_sequence_fills_column() {
        let mut pos = Position::new();
        let col = 3;

        for i in 0..Position::HEIGHT {
            assert!(pos.can_play(col));
            pos.play(col);
            assert_eq!(pos.heights[col], i + 1);
        }

        // Column should now be full
        assert!(!pos.can_play(col));
    }

    #[test]
    fn coordinate_new_creates_coordinate() {
        let coord = CoOrdinate::new(3, 4);
        assert_eq!(coord.x, 3);
        assert_eq!(coord.y, 4);
    }

    #[test]
    fn position_constants_are_correct() {
        assert_eq!(Position::WIDTH, 7);
        assert_eq!(Position::HEIGHT, 6);
        assert_eq!(Position::MAX_MOVES, 42);
    }

    #[test]
    fn play_all_columns_once() {
        let mut pos = Position::new();

        for col in 0..Position::WIDTH {
            pos.play(col);
            assert_eq!(pos.heights[col], 1);
        }
    }

    #[test]
    fn boundary_column_zero() {
        let mut pos = Position::new();
        assert!(pos.can_play(0));
        pos.play(0);
        assert_eq!(pos.heights[0], 1);
    }

    #[test]
    fn boundary_column_max() {
        let mut pos = Position::new();
        let max_col = Position::WIDTH - 1;
        assert!(pos.can_play(max_col));
        pos.play(max_col);
        assert_eq!(pos.heights[max_col], 1);
    }

    #[test]
    fn player_enum_equality() {
        assert_eq!(Player::Red, Player::Red);
        assert_eq!(Player::Blue, Player::Blue);
        assert_ne!(Player::Red, Player::Blue);
    }

    #[test]
    fn player_enum_copy_clone() {
        let red = Player::Red;
        let red_copy = red;
        let red_clone = red.clone();

        assert_eq!(red, red_copy);
        assert_eq!(red, red_clone);
    }

    #[test]
    fn index_from_coord_first_row() {
        let pos = Position::new();

        // First row (y=0) should map to indices 1-7
        for x in 0..Position::WIDTH {
            let coord = CoOrdinate::new(x, 0);
            let index = pos.index_from_coord(coord);
            assert_eq!(index as usize, 1 + (Position::WIDTH - 1 - x));
        }
    }

    #[test]
    fn play_does_not_affect_other_columns() {
        let mut pos = Position::new();

        pos.play(3);

        for col in 0..Position::WIDTH {
            if col == 3 {
                assert_eq!(pos.heights[col], 1);
            } else {
                assert_eq!(pos.heights[col], 0);
            }
        }
    }

    #[test]
    fn play_updates_bitboard() {
        let mut pos = Position::new();

        // Red player plays column 0
        pos.play(0);

        // Check that Red's bitboard (bitboards[0]) has been updated
        let red_bitboard = pos.bitboards[0];
        assert!(red_bitboard.count() > 0, "Red's bitboard should have at least one bit set");

        // Blue's bitboard should still be empty
        let blue_bitboard = pos.bitboards[1];
        assert_eq!(blue_bitboard.count(), 0, "Blue's bitboard should be empty");
    }

    #[test]
    fn play_sequence_updates_both_bitboards() {
        let mut pos = Position::new();

        // Red plays column 0
        pos.play(0);
        // Blue plays column 1
        pos.play(1);
        // Red plays column 2
        pos.play(2);

        // Both bitboards should have moves recorded
        assert_eq!(pos.bitboards[0].count(), 2, "Red should have 2 moves");
        assert_eq!(pos.bitboards[1].count(), 1, "Blue should have 1 move");
    }

    #[test]
    fn bitboard_reflects_column_position() {
        let mut pos = Position::new();

        pos.play(0);

        let red_bitboard = pos.bitboards[0];
        let coord = CoOrdinate::new(0, 0);
        let expected_index = pos.index_from_coord(coord);

        assert!(red_bitboard.is_set(expected_index),
                "Bitboard should have bit set at index {expected_index} for position (0,0)");
    }

    #[test]
    fn multiple_plays_same_column_stack() {
        let mut pos = Position::new();

        // Red plays column 3
        pos.play(3);
        // Blue plays column 3
        pos.play(3);
        // Red plays column 3
        pos.play(3);

        // Check that both players have moves in their bitboards
        assert!(pos.bitboards[0].count() >= 2, "Red should have at least 2 moves");
        assert!(pos.bitboards[1].count() >= 1, "Blue should have at least 1 move");

        // Column height should be 3
        assert_eq!(pos.heights[3], 3);
    }

    #[test]
    fn get_bitboard_returns_correct_player_board() {
        let mut pos = Position::new();

        // Initially Red's turn, bitboard should be empty
        let red_board = pos.get_bitboard();
        assert_eq!(red_board.to_u64(), 0);

        // Red plays
        pos.play(0);

        // Now it's Blue's turn, their bitboard should be empty
        let blue_board = pos.get_bitboard();
        assert_eq!(blue_board.to_u64(), 0);

        // Blue plays
        pos.play(1);

        // Back to Red's turn, their bitboard should have one move
        let red_board_after = pos.get_bitboard();
        assert!(red_board_after.count() > 0);
    }
}