use crate::lib::game::Game;

pub fn heuristic(game: &Game, _maximizing_player: bool) -> i32 {
    let player = game.current_player;
    let opponent = game.opponent_player;

    let cur_player = game.get_player();
    let op_player = game.get_opponent();

    let mut alignment_values = vec![0, 0];
    let mut possible_capture = vec![0, 0];

    let len = game.current_tiles.len();
    let from = if len > 8 {
        len - 8
    } else {
        0
    };
    for i in from..len {
        let tile = game.current_tiles[i];
        let p = game.get_tile_value(tile);
        let op = if p == 1 { 2 } else { 1 };
        for directions in &game.tiles_directions[tile.0 as usize][tile.1 as usize] {
            let mut real_aligned = 0;
            let mut empty_tiles: u8 = 0;
            let mut owned_tiles: u8 = 0;

            for (idx, direction) in directions.iter().enumerate() {
                let len = direction.len();
                if len == 0 {
                    continue;
                }

                let mut i = 0;
                let byte_move = idx * 4;
                while i < direction.len() {
                    let t = direction[i];
                    let value = game.get_tile_value(t);
                    if value == op {
                        break;
                    } else if value == 0 {
                        // 0Xff
                        // we store at 0X10 or 0X01 depending on idx.
                        empty_tiles += 1 << byte_move;
                    // to compare, move idx to the right, so the right side byte is removed when idx is 1,
                    // and then get the trailing_zeros to check if there are no empty tiles
                    // and you get the 'no empty tiles' comparison
                    } else if (empty_tiles >> byte_move).trailing_zeros() >= 4 {
                        real_aligned += 1;
                    } else {
                        owned_tiles += 1 << byte_move;
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

            let num_empty = ((empty_tiles & 0x0f) + (empty_tiles >> 4)) as i32;
            let num_owned = ((owned_tiles & 0x0f) + (owned_tiles >> 4)) as i32;
            if real_aligned + num_owned + num_empty > 5 {
                alignment_values[p as usize - 1] +=
                    (real_aligned * real_aligned * real_aligned * real_aligned)
                        + (num_owned * num_owned + num_empty);
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

    (cur_player_capture + player_possible + player_align_value)
        - (op_capture + op_possible + op_align_value)
}
