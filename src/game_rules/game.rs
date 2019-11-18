mod game_rules;
use game_rules::rules::place_piece;
use game_rules::rules::check_board_case;
use game_rules::rules::switch_player;
mod config;
const BOARD_LENGTH: usize = 19 * 19;

pub fn play() {
	let mut board: [i32; BOARD_LENGTH] = [0; BOARD_LENGTH];
	let mut end: bool = false;
	let mut player = 1;
	let mut index: usize = 12;
	while !end {
		if !check_board_case(board, index as i32, 0) {
			eprintln!("index to hot : {}", index);
			break;
		}
		end = place_piece(&mut board, index, 1);
		index += 21;
	}
}