use crate::lib::game::Game;

pub fn heuristic(game: &Game) -> i32 {
    let player = game.current_player;
    let opponent = game.opponent_player;

    let cur_player = game.get_player();
    let op_player = game.get_player();

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
            if value == 3 { value = 0 }; // fix that later it should not happen
            // also i noticed the heuristic and updates can get a little slow when called too many times
            i += 1;
            aligned[value as usize] = 1;

            while i < line_len && game.get_tile_value(line[i]) == value {
                i += 1;
                aligned[value as usize] += 1;
            }

            if value != 0 && aligned[value as usize] == 2 && i < line_len && prev_value != 4 {
                let next_value = game.get_tile_value(line[i]);
                if (next_value == 0 && prev_value != 0) || (next_value != 0 && prev_value == 0) {
                    possible_capture[value as usize - 1] = 2;
                }
            }
        }
    }

    (cur_player.captured as i32 + possible_capture[player as usize - 1])
        - (op_player.captured as i32 + possible_capture[opponent as usize - 1])
}
