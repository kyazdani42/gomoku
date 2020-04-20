use crate::lib::game::Game;

pub fn heuristic(game: &Game, _maximizing_player: bool) -> i32 {
    let player = game.current_player;
    let opponent = game.opponent_player;

    let cur_player = game.get_player();
    let op_player = game.get_opponent();

    let mut alignment_values = vec![0, 0];
    let mut possible_capture = vec![0, 0];

    for tile in &game.current_tiles {
        let tile = *tile;
        let p = game.get_tile_value(tile);
        let op = if p == 1 { 2 } else { 1 };
        for directions in &game.tiles_directions[tile.0 as usize][tile.1 as usize] {
            let mut real_aligned = 0;
            let mut empty_tiles = vec![0, 0];
            let mut owned_tiles = vec![0, 0];

            for idx in 0..2 {
                let direction = &directions[idx];
                let len = direction.len();
                if len == 0 {
                    continue;
                }

                let mut i = 0;
                while i < direction.len() {
                    let t = direction[i];
                    let value = game.get_tile_value(t);
                    if value == op {
                        break;
                    } else if value == 0 {
                        empty_tiles[idx] += 1;
                    } else if empty_tiles[idx] == 0 {
                        real_aligned += 1;
                    } else {
                        owned_tiles[idx] += 1;
                    }

                    i += 1;
                }

                if i == 0 && len > 2 {
                    let next_tile = direction[1];
                    if game.get_tile_value(next_tile) == opponent
                        && game.get_tile_value(direction[2]) == 0
                    {
                        let _first_op_tile = direction[0];
                        possible_capture[p as usize - 1] += 2;
                        // captures.push(whatever)
                    }
                }
            }

            let num_empty = empty_tiles[0] + empty_tiles[1];
            let num_owned = owned_tiles[0] + owned_tiles[1];
            if real_aligned + num_owned + num_empty > 5 {
                alignment_values[p as usize - 1] += (real_aligned * real_aligned * real_aligned * real_aligned) + (num_owned * num_owned + num_empty);
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
    let op_align_value = alignment_values[opponent as usize - 1];
    let player_align_value = alignment_values[player as usize - 1];

    (cur_player_capture + player_possible + player_align_value) - (op_capture + op_possible + op_align_value)
}
