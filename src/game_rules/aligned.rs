<<<<<<< Updated upstream
use super::super::BOARD_LENGTH;
use super::moves::bot_left;
use super::moves::top_right;

pub fn bot_left_top_right(board: [i32; ::BOARD_LENGTH], index: usize, player: i32) -> i32 {
    let mut nb_aligned = 0;
    let mut new_index = index;

    while check_board_case(board, new_index, player) {
        nb_aligned += 1;
        new_index = top_right(new_index);
    }

    new_index = bot_left(new_index);
=======
use super::moves::top_right;
use super::moves::bot_left;
use super::moves::top_left;
use super::moves::bot_right;
use super::moves::bot;
use super::moves::top;
use super::moves::right;
use super::moves::left;
use super::rules::check_board_case;
const BOARD_LENGTH: usize = 19 * 19;


pub fn bot_left_top_right(board: [i32; BOARD_LENGTH], index: usize, player: i32) -> i32 {
	let mut nb_aligned = 0;
	let mut new_index: i32 = index as i32;

    while check_board_case(board, new_index, player) {
        nb_aligned += 1;
        new_index = bot_left(new_index);
    }

	new_index = top_right(index as i32);

	while check_board_case(board, new_index, player) {
		nb_aligned += 1;
		new_index = top_right(new_index);
	}

	return nb_aligned;
}

pub fn top_left_bot_right(board: [i32; BOARD_LENGTH], index: usize, player: i32) -> i32 {
	let mut nb_aligned = 0;
	let mut new_index: i32 = index as i32;

	while check_board_case(board, new_index, player) {
		nb_aligned += 1;
		new_index = bot_right(new_index);
	}

	new_index = top_left(index as i32);

	while check_board_case(board, new_index, player) {
		nb_aligned += 1;
		new_index = top_left(new_index);
	}

	return nb_aligned;
}


pub fn left_right(board: [i32; BOARD_LENGTH], index: usize, player: i32) -> i32 {
	let mut nb_aligned = 0;
	let mut new_index: i32 = index as i32;

	while check_board_case(board, new_index, player) {
		nb_aligned += 1;
		new_index = right(new_index);
	}

	new_index = left(index as i32);

	while check_board_case(board, new_index, player) {
		nb_aligned += 1;
		new_index = left(new_index);
	}

	return nb_aligned;
}


pub fn top_bot(board: [i32; BOARD_LENGTH], index: usize, player: i32) -> i32 {
	let mut nb_aligned = 0;
	let mut new_index: i32 = index as i32;

	while check_board_case(board, new_index, player) {
		nb_aligned += 1;
		new_index = top(new_index);
	}

	new_index = bot(index as i32);

	while check_board_case(board, new_index, player) {
		nb_aligned += 1;
		new_index = bot(new_index);
	}

	return nb_aligned;
}
