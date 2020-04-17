use crate::lib::game::Game;

pub fn heuristic(game: &Game) -> i32 {
    let player = game.current_player;
    let opponent = game.opponent_player;

    let cur_player = game.get_player();
    let op_player = game.get_opponent();

    let mut possible_capture = vec![0, 0];

    let board_lines = &game.board_lines;
    for line in board_lines {
        let mut i = 0;
        let mut value = 4;
        let mut prev_value = 4;
        let line_len = line.len();
        // let mut num_empty_tiles = 0;
        let mut aligned: Vec<i32> = vec![0, 0, 0];

        while i < line_len - 1 {
            prev_value = value;
            value = game.get_tile_value(line[i]);
            i += 1;
            aligned[value as usize] = 1;

            while i < line_len && game.get_tile_value(line[i]) == value {
                i += 1;
                aligned[value as usize] += 1;
            }

            if value != 0 && aligned[value as usize] == 2 && i < line_len && prev_value != 4 {
                let next_value = game.get_tile_value(line[i]);
                if (next_value == 0 && prev_value != 0) || (next_value != 0 && prev_value == 0) {
                    possible_capture[value as usize - 1] += 2;
                }
            }
        }
    }

    let mut cur_player_capture = cur_player.captured as i32 * 10;
    cur_player_capture *= cur_player_capture;

    let mut player_possible = possible_capture[player as usize - 1];
    player_possible *= player_possible;

    let mut op_capture = op_player.captured as i32 * 10;
    op_capture *= op_capture;

    let mut op_possible = possible_capture[opponent as usize - 1];
    op_possible *= op_possible;

    (cur_player_capture + player_possible) - (op_capture + op_possible)
}
