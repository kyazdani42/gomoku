use super::super::BOARD_LENGTH;
use super::aligned::bot_left_top_right;

pub fn place_piece(board: &mut [i32; ::BOARD_LENGTH], index: usize, player: i32) -> bool {
    let mut win = false;
    board[index] = player;
    win = check_victory(*board, index, player);

    eprintln!("place index : {}", index);
    return win;
}

fn check_victory(board: [i32; ::BOARD_LENGTH], index: usize, player: i32) -> bool {
    let nb_aligned = bot_left_top_right(board, index, player);
    return nb_aligned > 4;
}

