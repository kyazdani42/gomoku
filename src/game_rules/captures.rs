use super::moves::top_right;
use super::moves::bot_left;
use super::moves::top_left;
use super::moves::bot_right;
use super::moves::bot;
use super::moves::top;
use super::moves::right;
use super::moves::left;
use super::rules::check_board_case;
use super::rules::switch_player;
const BOARD_LENGTH: usize = 19 * 19;

pub fn captures_all(board: [i32; BOARD_LENGTH], index: usize, player: i32) -> &[i32] {
	let mut i = 0;
	let mut captures = [];
	let (capture_top_one, capture_top_two) = capture(board, index, player, &top);
	if capture_top_one > -1 {
		captures[i] = capture_top_one;
		i += 1;
		captures[i] = capture_top_two;
	}
	return &captures;
}

pub fn capture(board: [i32; BOARD_LENGTH], index: usize, player: i32, f: &dyn Fn(i32) -> i32) -> (i32,i32) {
	let other_player = switch_player(player);
	let capture_one: i32 = f(index as i32);
	let capture_two: i32 = f(capture_one);
	let capture_final: i32 = f(capture_two);
	if capture_final == player {
		if capture_one == other_player && capture_two == other_player {
			return (capture_one, capture_two);
		}
	}

	return (-1, -1);
}