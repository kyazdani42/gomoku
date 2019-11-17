use super::moves::top_right;
use super::moves::bot_left;
use super::super::BOARD_LENGTH;

fn check_board_case(board: [i32; ::BOARD_LENGTH], index: usize, player: i32) -> bool {
	if index > ::BOARD_LENGTH || index < 0 {
		return false;
	}
	return board[index] == player;
}

pub fn bot_left_top_right(board: [i32; ::BOARD_LENGTH], index: usize, player: i32) -> i32 {
	let mut nb_aligned = 0;
	let mut new_index = index;

	while check_board_case(board, new_index, player) {
		nb_aligned += 1;
		new_index = top_right(new_index);
	}

	new_index = bot_left(new_index);

	while check_board_case(board, new_index, player) {
		nb_aligned += 1;
		new_index = bot_left(new_index);
	}

	return nb_aligned;
}