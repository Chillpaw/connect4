use crate::board::Bitboard;
use crate::position::{Position, Player};
use crate::win_detection::is_win;

const WINNING_SCORE: f32 = 1.0;
const LOSING_SCORE: f32 = -1.0;
const DRAW_SCORE: f32 = 0.0;
const THREE_IN_A_ROW_PLAYER: f32 = 0.00005;
const TWO_IN_A_ROW: f32 = 0.00001;
const THREE_IN_A_ROW_OPPONENT: f32 = -0.00008;

fn minimax(pos: Position, depth: usize, maximising_player: Player) -> f32 {
    let mut score = 0.0;
    //check if depth is zero or the board is in a winning/draw state
    if is_terminal_state(&pos) || (depth == 0) {
        score = evaluate_board(&pos, maximising_player);
        return score
    }

    score
}

fn is_terminal_state(&pos: &Position) -> bool {
    pos.board_full()
        || is_win(pos.get_bitboard(Player::Red))
        || is_win(pos.get_bitboard(Player::Blue))
}

fn evaluate_board(&pos: &Position, player: Player) -> f32 {
    let mut score = 0.0;

    let max_board = pos.get_bitboard(player);
    let empty_board = !(pos.get_bitboard(player) | pos.get_bitboard(player.other()));



    score
}

/// look for pairs which have open-ended pairs either side
/// for a horizontal case it would look like
/// 0 0 1 1 0 0 0 >> 1 -> 0 0 0 1 0 0 0 after AND with original board
/// 1 1 0 0 1 1 1 empty board after inversion.
/// 1 1 0 0 1 1 1 >> 1 -> 0 1 0 0 0 1 1 after shift left and AND. We can use index -2 and 2 of paired bit to signify empty pairs either side of the player's pair
///
///
///

fn find_pairs(b: Bitboard, e: Bitboard) -> u32 {
    let mut pairs_count = 0;

    pairs_count += count_horizontal_pairs(b, e);
    pairs_count += count_vertical_pairs(b, e);
    pairs_count += count_diag_left_pairs(b, e);
    pairs_count += count_diag_right_pairs(b, e);

    pairs_count
}

fn count_horizontal_pairs(b: Bitboard, empties: Bitboard) -> u32 {
    let mut pairs_count = 0;

    // check horizontal pairs
    let m = b & Bitboard::from_u64(Position::NOT_RIGHT_EDGE);
    let pairs = m & (b >> 1);

    // check for open-ended pairs either side
    pairs_count += ((pairs << 2) & empties & Bitboard::from_u64(Position::NOT_LEFT_EDGE)).count(); // left side
    pairs_count += ((pairs >> 1) & empties).count(); // right side

    pairs_count
}

fn count_vertical_pairs(b: Bitboard, empties: Bitboard) -> u32 {
    let mut pairs_count = 0;

    // check vertical pairs
    let width = Position::WIDTH as u8;
    let pairs = b & (b >> width); // move 'up' the board

    // check for open-ended pairs above the vertical pair
    pairs_count += (pairs & (empties >> (width * 2))).count(); // not required as below will always be blocked and top will always be open

    pairs_count
}

fn count_diag_left_pairs(b: Bitboard, empties: Bitboard) -> u32 {

    // check left diagonal pairs
    let mask = Bitboard::from_u64(Position::NOT_LEFT_EDGE);
    let m = b & mask;
    let offset = Position::WIDTH as u8 - 1;
    let pairs = m & (b >> offset);
    let one_step_ahead = pairs & (empties >> offset * 2);
    let two_steps_ahead = pairs & (empties >> offset * 3);

    (pairs & one_step_ahead & two_steps_ahead).count()
}

fn count_diag_right_pairs(b: Bitboard, empties: Bitboard) -> u32 {

    // Pairs along the \ diagonal use offset (WIDTH + 1); mask like win_detection::diag_right_win.
    let mask = Bitboard::from_u64(Position::NOT_RIGHT_EDGE);
    let m = b & mask;
    let offset = Position::WIDTH as u8 + 1;
    let pairs = m & (b >> offset);
    let one_step_ahead = pairs & (empties >> offset * 2);
    let two_steps_ahead = pairs & (empties >> offset * 3);
    let one_step_behind = pairs & (empties << offset * 2);

    ((one_step_ahead & two_steps_ahead) | (one_step_behind & one_step_ahead)).count()
}

#[cfg(test)]
mod tests {
    use crate::board::Bitboard;
    use crate::minimax::find_pairs;
    use crate::position::Position;

    #[test]
    fn pair_found_horizontal() {
        // this test should find two open-ended pairs for the player
        let player_board = Bitboard::from_u64(0x18); // 0 0 1 1 0 0 0
        println!("{}",player_board);
        let empty_board = !player_board;
        println!("{}", empty_board);
        assert_eq!(find_pairs(player_board, empty_board), 2)
    }

    #[test]
    fn pairs_not_found_horizontal() {
        // this test should find two open-ended pairs for the player
        let player_board = Bitboard::from_u64(0x18); // 0 0 1 1 0 0 0
        println!("{}",player_board);
        let empty_board = Bitboard::from_u64(0x5b); // 1 0 1 1 0 1 1
        println!("{}", empty_board);
        assert_eq!(find_pairs(player_board, empty_board), 0)
    }

    #[test]
    fn pair_found_horizontal_left_edge() {
        // this test should find one open-ended pair for the player
        let player_board = Bitboard::from_u64(0x60); // 1 1 0 0 0 0 0
        println!("{}",player_board);
        let empty_board = !player_board;
        println!("{}", empty_board);
        assert_eq!(find_pairs(player_board, empty_board), 1)
    }

    #[test]
    fn pair_found_horizontal_right_edge() {
        // this test should find one open-ended pair for the player
        let player_board = Bitboard::from_u64(0x3); // 0 0 0 0 0 1 1
        println!("{}",player_board);
        let empty_board = !player_board;
        println!("{}", empty_board);
        assert_eq!(find_pairs(player_board, empty_board), 1)
    }

    #[test]
    fn pair_found_vertical_central() {
        // this test should find two open-ended pairs for the player
        let player_board = Bitboard::from_u64(0x408); // 0 0 0 1 0 0 0 | 0 0 0 1 0 0 0
        println!("player board:\n{}",player_board);
        let empty_board = !player_board;
        println!("empty board:\n{}", empty_board);
        assert_eq!(find_pairs(player_board, empty_board), 1)
    }

    #[test]
    fn pair_found_vertical_left_edge() {
        // this test should find two open-ended pairs for the player
        let player_board = Bitboard::from_u64(0x2040); // 1 0 0 0 0 0 0 | 1 0 0 0 0 0 0
        println!("player board:\n{}",player_board);
        let empty_board = !player_board;
        println!("empty board:\n{}", empty_board);
        assert_eq!(find_pairs(player_board, empty_board), 1)
    }

    #[test]
    fn pair_found_vertical_right_edge() {
        // this test should find two open-ended pairs for the player
        let player_board = Bitboard::from_u64(0x081); // 0 0 0 0 0 0 1 | 0 0 0 0 0 0 1
        println!("player board:\n{}",player_board);
        let empty_board = !player_board;
        println!("empty board:\n{}", empty_board);
        assert_eq!(find_pairs(player_board, empty_board), 1)
    }

    #[test]
    fn pair_found_diag_bottom_left() {
        // this test should find two open-ended pairs for the player
        let player_board = Bitboard::from_u64(0x1040); // 0 1.0 0 0 0.0 | 1 0 0.0 0 0 0
        println!("player board:\n{}",player_board);
        let empty_board = !player_board;
        println!("empty board:\n{}", empty_board);
        assert_eq!(find_pairs(player_board, empty_board), 1)
    }

    #[test]
    fn pair_found_diag_bottom_right() {
        // this test should find two open-ended pairs for the player
        let player_board = Bitboard::from_u64(0x101); // 0 0.0 0 0 1.0 | 0 0 0.0 0 0 1
        println!("player board:\n{}",player_board);
        let empty_board = !player_board;
        println!("empty board:\n{}", empty_board);
        assert_eq!(find_pairs(player_board, empty_board), 1)
    }

    #[test]
    fn pair_found_diag_top_left() {
        // this test should find two open-ended pairs for the player
        let player_board = Bitboard::from_u64(0x20200000000);
        println!("player board:\n{}",player_board);
        let empty_board = !player_board;
        println!("empty board:\n{}", empty_board);
        assert_eq!(find_pairs(player_board, empty_board), 1)
    }

    #[test]
    fn pair_found_diag_top_right() {
        // this test should find two open-ended pairs for the player
        let player_board = Bitboard::from_u64(0x820000000);
        println!("player board:\n{}",player_board);
        let empty_board = !player_board;
        println!("empty board:\n{}", empty_board);
        assert_eq!(find_pairs(player_board, empty_board), 1)
    }
}
