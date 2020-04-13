use super::r#move::{Move,Moves};
use super::game::{Game, Tile};
use std::collections::HashMap;

pub struct AnalyzedTile {
    pub captured: Vec<Tile>,
    pub alignments: Vec<Vec<Tile>>,
    pub double_free_three: bool,
    pub win: bool,
    pub oponent_win: bool,
}

pub fn analyze_index(tile: &Tile, game: &Game) -> AnalyzedTile {
    let mut free_threes: u8 = 0;
    let mut data = AnalyzedTile {
        captured: vec![],
        alignments: vec![],
        double_free_three: false,
        win: false,
        oponent_win: false,
    };

    let mut move_counter = 0;
    let game_moves = &game.moves.lock().unwrap();
    for moves in &game_moves.straight_moves {
        let mut counters = [1, 1];
        let mut tile_values = [4, 4];
        let mut aligned = [vec![], vec![]];

        for move_index in 0..2 {
            let direction = &moves[move_index];
            let mut t = direction.get_next_tile(tile);
            while counters[move_index] < 5 && direction.is_ok(&t) {
                tile_values[move_index] = game.get_tile_value(&t);
                if tile_values[move_index] != game.current_player {
                    break;
                }
                aligned[move_index].push(t);
                counters[move_index] += 1;
                t = direction.get_next_tile(&t);
            }
        }

        for idx in 0..2 {
            let tile_value = tile_values[idx];
            let tile_value_reverse = if idx == 0 {
                tile_values[1]
            } else {
                tile_values[0]
            };
            let counter = counters[idx];
            let counter_reverse = if idx == 0 { counters[1] } else { counters[0] };
            let direction = &moves[idx];

            if counter == 1 && tile_value == game.opponent_player {
                let t3 = &direction.get_tile_mult(&tile, 3);
                if !direction.is_ok(t3) {
                    continue;
                }
                let t = direction.get_next_tile(tile);
                let t2 = direction.get_tile_mult(tile, 2);
                if game.get_tile_value(&t2) == game.opponent_player {
                    if game.get_tile_value(t3) == game.current_player
                    {
                        data.captured.push(t);
                        data.captured.push(t2);
                    }
                }
            } else if counter < 4
                && tile_value_reverse == 0
                && counter_reverse < 3
                && tile_value == 0
            {
                if counter == 1 {
                    let t3 = direction.get_tile_mult(&tile, 3);
                    if !direction.is_ok(&t3) {
                        continue;
                    }

                    let value2 = game.get_tile_value(&direction.get_tile_mult(&tile, 2));
                    if value2 != game.current_player {
                        continue;
                    }

                    let value3 = game.get_tile_value(&t3);
                    let t4 = direction.get_next_tile(&t3);
                    if counter_reverse == 1
                        && value3 == game.current_player
                        && direction.is_ok(&t4)
                    {
                        let value4 = game.get_tile_value(&t4);
                        if value4 == 0 {
                            free_threes += 1;
                        }
                    } else if value3 == 0 && counter_reverse == 2 {
                        free_threes += 1;
                    }
                } else if counter == 2 {
                    if counter_reverse == 1 {
                        let t4 = direction.get_tile_mult(&tile, 4);
                        if !direction.is_ok(&t4) {
                            continue;
                        }

                        if game.get_tile_value(&direction.get_tile_mult(&tile, 3))
                            == game.current_player
                            && game.get_tile_value(&t4) == 0
                        {
                            free_threes += 1;
                        }
                    } else if idx == 0 {
                        free_threes += 1;
                    }
                } else if counter_reverse == 1 {
                    free_threes += 1;
                }
            }
        }

        if free_threes > 1 {
            return AnalyzedTile {
                captured: vec![],
                alignments: vec![],
                double_free_three: true,
                win: false,
                oponent_win: false,
            };
        }

        if aligned[0].len() + aligned[1].len() > 3 {
            let mut all_moves = vec![];
            for i in 0..game_moves.straight_moves.len() {
                if i == move_counter {
                    continue;
                }
                all_moves.push(&game_moves.straight_moves[i]);
            }

            aligned[1].reverse();
            let aligned = aligned.to_vec().join(tile);

            let idxs = get_indexes_from_alignment(&aligned);
            let capturable = get_capturable_indexes(&idxs, game, &all_moves);
            data.alignments.push(capturable);
        }

        move_counter += 1;
    }

    if game.get_player().captured + data.captured.len() as u8 > 9 {
        data.win = true
    } else if game.opponent_alignments.len() > 0 {
        for al in &game.opponent_alignments {
            if al.len() == 0 {
                data.oponent_win = true;
                break;
            }
            if al.iter().all(|x| !data.captured.contains(x)) {
                data.oponent_win = true;
                break;
            }
        }
    } else {
        let num_alignments = data.alignments.len();
        if num_alignments > 1 || (num_alignments > 0 && data.alignments[0].len() == 0) {
            let catchers = get_catcher_indexes(game, game_moves);

            let max_captures = catchers.iter().fold(0, |max_c, catcher| {
                if max_c > *catcher.1 {
                    max_c
                } else {
                    *catcher.1
                }
            });

            if game.get_opponent().captured as i32 + max_captures < 10 {
                data.win = true
            }
        }
    }

    data
}

fn get_indexes_from_alignment(alignment: &Vec<Tile>) -> Vec<Tile> {
    match alignment.len() {
        6 => vec![alignment[1], alignment[2], alignment[3], alignment[4]],
        7 => vec![alignment[2], alignment[3], alignment[4]],
        8 => vec![alignment[3], alignment[4]],
        9 => vec![alignment[4]],
        _ => alignment.clone(),
    }
}

fn get_capturable_indexes(
    aligned: &Vec<Tile>,
    game: &Game,
    all_moves: &Vec<&Vec<Box<dyn Move + Send>>>,
) -> Vec<Tile> {
    let mut capturable = vec![];

    for tile in aligned {
        for moves in all_moves {
            let first_move = &moves[0];
            let second_move = &moves[1];

            let t1 = &first_move.get_next_tile(tile);
            let t1_2 = &second_move.get_next_tile(tile);
            if !first_move.is_ok(t1)
                || !second_move.is_ok(t1_2)
            {
                continue;
            }

            let first_value = game.get_tile_value(t1);
            let second_value = game.get_tile_value(t1_2);

            if first_value == 1 && second_value != 1 {
                let t2 = &first_move.get_tile_mult(tile, 2);
                if !first_move.is_ok(t2) {
                    continue;
                }

                let edge_value = game.get_tile_value(t2);
                if edge_value == 1 {
                    continue;
                }

                if edge_value != second_value {
                    capturable.push(*tile);
                }
            } else if second_value == 1 && first_value != 1 {
                let t2 = &second_move.get_tile_mult(tile, 2);
                if !second_move.is_ok(t2) {
                    continue;
                }

                let edge_value = game.get_tile_value(t2);
                if edge_value == 1 {
                    continue;
                }

                if edge_value != first_value {
                    capturable.push(*tile);
                }
            }
        }
    }

    capturable
}

fn get_catcher_indexes(game: &Game, game_moves: &std::sync::MutexGuard<Moves>) -> HashMap<Tile, i32> {
    let mut catchers = HashMap::new();

    for tile in &game.get_player().last_hits {
        for moves in &game_moves.straight_moves {
            let first_move = &moves[0];
            let second_move = &moves[1];

            let first_move_tile = &first_move.get_next_tile(tile);
            let second_move_tile = &second_move.get_next_tile(tile);
            if !first_move.is_ok(first_move_tile)
                || !second_move.is_ok(second_move_tile)
            {
                continue;
            }

            let first_value = game.get_tile_value(first_move_tile);
            let second_value = game.get_tile_value(second_move_tile);

            if first_value == 1 && second_value != 1 {
                let edge_value_tile = &first_move.get_tile_mult(tile, 2);
                if !first_move.is_ok(edge_value_tile) {
                    continue;
                }

                let edge_value = game.get_tile_value(edge_value_tile);
                if edge_value == 1 {
                    continue;
                }

                if edge_value != second_value {
                    let value_index = if edge_value == 0 {
                        *edge_value_tile
                    } else {
                        *second_move_tile
                    };
                    let value = if let Some(value) = catchers.get(&value_index) {
                        *value + 1
                    } else {
                        1
                    };
                    catchers.insert(value_index, value);
                }
            } else if second_value == 1 && first_value != 1 {
                let edge_value_tile = &second_move.get_tile_mult(tile, 2);
                if !second_move.is_ok(edge_value_tile) {
                    continue;
                }

                let edge_value = game.get_tile_value(edge_value_tile);
                if edge_value == 1 {
                    continue;
                }

                if edge_value != first_value {
                    let value_index = if edge_value == 0 {
                        *edge_value_tile
                    } else {
                        *first_move_tile
                    };
                    let value = if let Some(value) = catchers.get(&value_index) {
                        *value + 1
                    } else {
                        1
                    };
                    catchers.insert(value_index, value);
                }
            }
        }
    }

    catchers
}
