use crate::game;

use game::{get_alignment, Stone, JOINED_ACTIONS};

pub fn compute(board: &Vec<Vec<u8>>, player: &u8, board_size: usize) -> (usize, usize) {
    let mut play_line = 0;
    let mut play_col = 0;
    let mut point = 0;
    let other_player = if *player == 1 { 2 } else { 1 };

    for (i_line, line) in board.iter().enumerate() {
        for (i_col, col) in line.iter().enumerate() {
            if *col == 0 {
                let mut stone_point = get_basic_point(i_line, i_col, board_size);
                stone_point += get_alignement_point(board, i_line, i_col, board_size, *player);
                stone_point += get_alignement_point(board, i_line, i_col, board_size, other_player);
                if stone_point > point {
                    point = stone_point;
                    play_line = i_line;
                    play_col = i_col;
                }
            }
        }
    }

    (play_line, play_col)
}

fn get_alignement_point(
    board: &Vec<Vec<u8>>,
    line: usize,
    col: usize,
    board_size: usize,
    player: u8,
) -> i32 {
    JOINED_ACTIONS.iter().fold(0, |mut points, actions| {
        points += get_alignment(board, &Stone(line, col), player, board_size, *actions);
        points
    })
}

fn get_basic_point(line: usize, col: usize, board_size: usize) -> i32 {
    let point_line = line < (board_size / 2);
    let point_col = col < (board_size / 2);

    if !point_line && !point_col {
        (board_size - line + board_size - col) as i32
    } else if !point_line {
        (board_size - line + col) as i32
    } else if !point_col {
        (board_size - col + line) as i32
    } else {
        (line + col) as i32
    }
}
