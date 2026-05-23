use connect4_core::position::{PlayError, Player, Position};
use connect4_core::win_detection::is_win;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayError {
    InvalidCharacter { position: usize, character: char },
    IllegalMove { position: usize, character: char, reason: PlayError },
}

#[derive(Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CellState {
    Red,
    Blue,
    Empty,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GameState {
    InProgress,
    RedWins,
    BlueWins,
    Draw,
}

#[derive(Serialize, Debug)]
pub struct GameStateResponse {
    pub moves: String,
    pub board: Vec<Vec<CellState>>,
    pub state: GameState,
    pub next_player: CellState,
}

pub fn position_to_response(moves: String) -> GameStateResponse {
    let position = match replay(&moves) {
        Ok(pos) => pos,
        Err(e) => panic!("{:?}", e),
    };
    let mut board: Vec<Vec<CellState>> = (0..Position::HEIGHT)
        .map(|_| (0..Position::WIDTH)
            .map(|_| CellState::Empty)
            .collect())
        .collect();
    for (row, rows) in board.iter_mut().enumerate() {
        for (col, cell) in rows.iter_mut().enumerate() {
            let cell_state = match position.cell_at(col, row) {
                None => { CellState::Empty }
                Some(player) => {
                    match player {
                        Player::Red => CellState::Red,
                        Player::Blue => CellState::Blue
                    }
                }
            };
            *cell = cell_state
        }
    }

    let mut state = GameState::InProgress;
    if is_win(position.bitboards[1]) {
        state = GameState::BlueWins;
    }
    if is_win(position.bitboards[0]) {
        state = GameState::RedWins;
    }


    let next_player = match position.player_to_move {
        Player::Red => CellState::Red,
        Player::Blue => CellState::Blue,
    };

    GameStateResponse {
        moves,
        board,
        state,
        next_player,
    }
}

fn replay(moves: &str) -> Result<Position, ReplayError> {
    let mut pos = Position::new();
    for (i, ch) in moves.chars().enumerate() {
        let column = match ch {
            '1'..='7' => ch as usize - '1' as usize,
            _ => return Err(ReplayError::InvalidCharacter { position: i, character: ch })
        };
        pos.play(column)
            .map_err(|e| ReplayError::IllegalMove { position: i, character: ch, reason: e })?;
    }
    Ok(pos)
}

#[cfg(test)]
mod tests {
    use crate::game::*;
    use connect4_core::board::Bitboard;
    use connect4_core::position::Player;
    use connect4_core::win_detection::is_win;

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
        assert_eq!(replay("8"), Err(ReplayError::InvalidCharacter {
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

    #[test]
    fn board_correct_dimensions() {
        let resp = position_to_response("".to_string());
        assert_eq!(resp.board.len(), Position::HEIGHT);
        for row in &resp.board { assert_eq!(row.len(), Position::WIDTH) }
    }

    #[test]
    fn empty_board_is_empty_cells() {
        let resp = position_to_response("".to_string());
        assert!(resp.board.iter().flatten().all(|c| *c == CellState::Empty))
    }

    #[test]
    fn initial_next_player_is_red() {
        let resp = position_to_response("".to_string());
        assert_eq!(resp.next_player, CellState::Red);
    }

    #[test]
    fn next_player_alternates_after_one_move() -> Result<(), ReplayError> {
        let resp = position_to_response("4".to_string());
        assert_eq!(resp.next_player, CellState::Blue);
        Ok(())
    }

    #[test]
    fn next_player_alternates_back_to_red_after_two_moves() -> Result<(), ReplayError> {
        let resp = position_to_response("44".to_string());
        assert_eq!(resp.next_player, CellState::Red);
        Ok(())
    }

    #[test]
    fn initial_state_is_in_progress() {
        let resp = position_to_response("".to_string());
        assert_eq!(resp.state, GameState::InProgress);
    }

    #[test]
    fn state_is_in_progress_mid_game() -> Result<(), ReplayError> {
        let resp = position_to_response("44556611".to_string());
        assert_eq!(resp.state, GameState::InProgress);
        Ok(())
    }

    #[test]
    fn state_is_red_wins() -> Result<(), ReplayError> {
        // Reuses the known terminal position from the replay tests
        let moves = "44444432655555323332267666211123567777711";
        let resp = position_to_response(moves.to_string());
        assert_eq!(resp.state, GameState::RedWins);
        Ok(())
    }

    #[test]
    fn json_serialisation() -> Result<(), ReplayError> {
        let moves = "44";
        let resp = position_to_response(moves.to_string());
        let json = serde_json::to_string(&resp).expect("serialisation failed.");
        let value: serde_json::Value = serde_json::from_str(&json).expect("invalid json.");

        assert_eq!(value["moves"], "44");
        assert_eq!(value["state"], "in_progress");
        assert_eq!(value["next_player"], "red");

        Ok(())
    }
}