use super::aligned::bot_left_top_right;
use super::aligned::top_left_bot_right;
use super::aligned::top_bot;
use super::aligned::left_right;
const BOARD_LENGTH: i32 = 19 * 19;

pub fn switch_player(player: i32) -> i32 {
    if player == 1 { 2 } else { 1 }
}

pub fn check_board_case(board: [i32; BOARD_LENGTH as usize], index: i32, player: i32) -> bool {
    if index > BOARD_LENGTH - 1 || index < 0 {
        return false;
    }
    return board[index as usize] == player;
}

fn check_victory(board: [i32; BOARD_LENGTH as usize], index: usize, player: i32) -> bool {
    let nb_aligned = bot_left_top_right(board, index, player);
    if nb_aligned > 4 {
        return true;
    }
    let nb_aligned = top_left_bot_right(board, index, player);
    if nb_aligned > 4 {
        return true;
    }
    let nb_aligned = top_bot(board, index, player);
    if nb_aligned > 4 {
        return true;
    }
    let nb_aligned = left_right(board, index, player);
    if nb_aligned > 4 {
        return true;
    }

    return false;
}

fn captures(board: &mut [i32; BOARD_LENGTH as usize], index: usize, player: i32) {

}

pub fn place_piece(board: &mut [i32; BOARD_LENGTH as usize], index: usize, player: i32) -> bool {
    board[index] = player;
    if check_victory(*board, index, player) {
        return true;
    }
    captures(board, index, player);
    return false;
}

