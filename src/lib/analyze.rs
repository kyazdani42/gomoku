use super::game::{Game, Tile};
use std::collections::HashMap;

pub struct AnalyzedTile {
    pub captured: Vec<Tile>,
    pub alignments: Vec<Vec<Tile>>,
    pub double_free_three: bool,
    pub win: bool,
    pub oponent_win: bool,
    pub catchers: Vec<Tile>,
    pub oponent_can_win: bool,
}

pub fn analyze_index(tile: Tile, game: &Game) -> AnalyzedTile {
    let mut free_threes: u8 = 0;
    let mut data = AnalyzedTile {
        captured: vec![],
        alignments: vec![],
        double_free_three: false,
        win: false,
        oponent_win: false,
        catchers: vec![],
        oponent_can_win: false,
    };

    for directions in &game.tiles_directions[tile as usize] {
        let mut counters = [1, 1];
        let mut tile_values = [4, 4];
        let mut aligned = [vec![], vec![]];

        for move_index in 0..2 {
            let direction = &directions[move_index];
            if direction.is_empty() {
                continue;
            }

            while counters[move_index] <= direction.len() {
                let t = direction[counters[move_index] - 1];
                tile_values[move_index] = game.get_tile_value(t);
                if tile_values[move_index] != game.current_player {
                    break;
                }
                aligned[move_index].push(t);
                counters[move_index] += 1;
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
            let direction = &directions[idx];
            let len = direction.len();

            if counter == 1 && tile_value == game.opponent_player {
                if len < 3 {
                    continue;
                }
                let t = direction[0];
                let t2 = direction[1];
                let t3 = direction[2];
                if game.get_tile_value(t2) == game.opponent_player
                    && game.get_tile_value(t3) == game.current_player
                {
                    data.captured.push(t);
                    data.captured.push(t2);
                }
            } else if counter < 4
                && tile_value_reverse == 0
                && counter_reverse < 3
                && tile_value == 0
            {
                if counter == 1 {
                    if len < 3 {
                        continue;
                    }

                    let value2 = game.get_tile_value(direction[1]);
                    if value2 != game.current_player {
                        continue;
                    }

                    let value3 = game.get_tile_value(direction[2]);
                    if counter_reverse == 1 && value3 == game.current_player && len > 3 {
                        let value4 = game.get_tile_value(direction[3]);
                        if value4 == 0 {
                            free_threes += 1;
                        }
                    } else if value3 == 0 && counter_reverse == 2 {
                        free_threes += 1;
                    }
                } else if counter == 2 {
                    if counter_reverse == 1 {
                        if len < 4 {
                            continue;
                        }

                        if game.get_tile_value(direction[2]) == game.current_player
                            && game.get_tile_value(direction[3]) == 0
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
                catchers: vec![],
                oponent_can_win: false,
            };
        }

        if aligned[0].len() + aligned[1].len() > 3 {
            aligned[1].reverse();
            let aligned = [&aligned[0][..], &[tile], &aligned[1][..]].concat();

            let idxs = get_indexes_from_alignment(&aligned);
            let capturable = get_capturable_indexes(&idxs, game);
            data.alignments.push(capturable);
        }
    }

    if game.get_player().captured + data.captured.len() as u8 > 9 {
        data.win = true
    } else if !game.opponent_alignments.is_empty() {
        for al in &game.opponent_alignments {
            if al.is_empty() {
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
        if num_alignments > 1 || (num_alignments > 0 && data.alignments[0].is_empty()) {
            let catchers = get_catcher_indexes(game);

            let max_captures = catchers.iter().fold(0, |max_c, catcher| {
                data.catchers.push(*catcher.0);

                if max_c > *catcher.1 {
                    max_c
                } else {
                    *catcher.1
                }
            });

            if game.get_opponent().captured as i32 + max_captures < 10 {
                data.win = true
            } else {
                data.oponent_can_win = true
            }
        }
    }

    data
}

fn get_indexes_from_alignment(alignment: &[Tile]) -> Vec<Tile> {
    match alignment.len() {
        6 => vec![alignment[1], alignment[2], alignment[3], alignment[4]],
        7 => vec![alignment[2], alignment[3], alignment[4]],
        8 => vec![alignment[3], alignment[4]],
        9 => vec![alignment[4]],
        _ => alignment.to_vec(),
    }
}

fn get_capturable_indexes(aligned: &[Tile], game: &Game) -> Vec<Tile> {
    let mut capturable = vec![];

    let cur_player = game.current_player;
    for tile in aligned {
        for directions in &game.tiles_directions[*tile as usize] {
            let dir1 = &directions[0];
            let dir2 = &directions[1];

            if dir1.is_empty() || dir2.is_empty() {
                continue;
            }

            let first_value = game.get_tile_value(dir1[0]);
            let second_value = game.get_tile_value(dir2[0]);


            if first_value == cur_player && second_value != cur_player {
                if dir1.len() < 2 {
                    continue;
                }

                let edge_value = game.get_tile_value(dir1[1]);
                if edge_value == cur_player {
                    continue;
                }

                if edge_value != second_value {
                    capturable.push(*tile);
                }
            } else if second_value == cur_player && first_value != cur_player {
                if dir2.len() < 2 {
                    continue;
                }

                let edge_value = game.get_tile_value(dir2[1]);
                if edge_value == cur_player {
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

fn get_catcher_indexes(game: &Game) -> HashMap<Tile, i32> {
    let mut catchers = HashMap::new();

    let cur_player = game.current_player;
    for tile in &game.get_player().last_hits {
        for directions in &game.tiles_directions[*tile as usize] {
            let dir1 = &directions[0];
            let dir2 = &directions[1];

            if dir1.is_empty() || dir2.is_empty() {
                continue;
            }

            let first_value = game.get_tile_value(dir1[0]);
            let second_value = game.get_tile_value(dir2[0]);

            if first_value == cur_player && second_value != cur_player {
                if dir1.len() < 2 {
                    continue;
                }

                let edge_value = game.get_tile_value(dir1[1]);
                if edge_value == cur_player {
                    continue;
                }

                if edge_value != second_value {
                    let value_index = if edge_value == 0 { dir1[1] } else { dir2[0] };
                    let value = if let Some(value) = catchers.get(&value_index) {
                        *value + 1
                    } else {
                        1
                    };
                    catchers.insert(value_index, value);
                }
            } else if second_value == cur_player && first_value != cur_player {
                if dir2.len() < 2 {
                    continue;
                }

                let edge_value = game.get_tile_value(dir2[1]);
                if edge_value == cur_player {
                    continue;
                }

                if edge_value != first_value {
                    let value_index = if edge_value == 0 { dir2[1] } else { dir1[0] };
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
