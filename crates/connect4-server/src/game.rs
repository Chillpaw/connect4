use connect4_core::position::{PlayError, Position};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayError {
    InvalidCharacter { position: usize, character: char },
    IllegalMove { position: usize, character: char, reason: PlayError }
}

pub fn replay(moves: &str) -> Result<Position, ReplayError> {
    let mut pos = Position::new();
    for (i, ch) in moves.chars().enumerate() {
        let column = match ch {
            '1'..='7' => ch as usize - '1' as usize,
            _ => return Err(ReplayError::InvalidCharacter { position: i, character: ch })
        };
        println!("parsed column: {column}");
        pos.play(column)
            .map_err(|e| ReplayError::IllegalMove { position: i, character: ch, reason: e})?;
    }
    Ok(pos)
}

#[cfg(test)]
mod tests {
    use connect4_core::board::Bitboard;
    use connect4_core::position::Player;
    use connect4_core::win_detection::is_win;
    use crate::game::*;

    #[test]
    fn empty_position_string() -> Result<(), ReplayError> {
        let pos = replay("")?;
        let empty_pos = Position::new();
        assert_eq!(pos, empty_pos);

        Ok(())
    }

    #[test]
    fn replay_one_position() -> Result<(), ReplayError> {
        let pos = replay("4")?;
        let bb = Bitboard::from_u64(0x8);
        println!("{pos}");
        println!("{bb}");

        assert_eq!(pos.bitboards[0], bb);
        assert_eq!(pos.player_to_move, Player::Blue);

        Ok(())
    }

    #[test]
    fn replay_two_positions() -> Result<(), ReplayError> {
        let pos = replay("44")?;
        let red = Bitboard::from_u64(0x8);
        let blue = Bitboard::from_u64(0x400);

        assert_eq!(pos.bitboards[0], red);
        assert_eq!(pos.bitboards[1], blue);
        assert_eq!(pos.player_to_move, Player::Red);

        Ok(())
    }

    #[test]
    fn replay_terminal_position() -> Result<(), ReplayError> {
        let pos = replay("44444432655555323332267666211123567777711")?;
        println!("{pos}");
        assert!(is_win(pos.bitboards[0]));

        Ok(())
    }

    #[test]
    fn invalid_character_returns_error() {
        assert_eq!(replay("x"), Err(ReplayError::InvalidCharacter { position: 0, character: 'x' }))
    }

    #[test]
    fn invalid_character_returns_error_second_position() {
        assert_eq!(replay("4x4"), Err(ReplayError::InvalidCharacter { position: 1, character: 'x' }))
    }

    #[test]
    fn move_out_of_range_returns_error() {
        assert_eq!(replay("0"), Err(ReplayError::InvalidCharacter {
            position: 0,
            character: '0',
        }));
        assert_eq!(replay("8"), Err(ReplayError::InvalidCharacter{
            position: 0,
            character: '8',
        }))
    }

    #[test]
    fn column_full_returns_error() {
        assert_eq!(replay("1111111"), Err(ReplayError::IllegalMove {
            position: 6,
            character: '1',
            reason: PlayError::ColumnFull
        }))

    }

}