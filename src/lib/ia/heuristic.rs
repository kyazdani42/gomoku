use crate::lib::game::Game;

pub fn heuristic(game: &Game) -> i32 {
    let player = game.get_player();
    let opponent_player = game.get_opponent();
    let mut player_value = 0;
    let mut opponent_value = 0;

    let board_size = game.board_size;
    let index_size = board_size - 1;

    for line in 0..board_size {
        for col in 0..board_size {
            let tile = (line, col);
            let tile_value = game.get_tile_value(tile);
            if tile_value == 1 || tile_value == 2 {
                // sur une ligne
            }
            if line == 0 {
                // initialize

                for inline in 0..board_size {
                    let column_tile = (inline, col);

                    let inline = if inline < col { inline } else { inline - col };
                    let tile = (inline, inline);
                    let tile_value = game.get_tile_value(tile);
                    if tile_value == 1 || tile_value == 2 {
                        //
                    }

                    let second_tile = (index_size - inline, inline);
                    let tile_value = game.get_tile_value(second_tile);
                    if tile_value == 1 || tile_value == 2 {
                        //
                    }
                }
            }
        }
    }

    player.captured as i32 - opponent_player.captured as i32
}
