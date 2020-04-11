use super::game::{Game, Tile};
use super::r#move::Move;
use std::cmp::min;
use std::collections::HashMap;

pub struct AnalyzedTile {
    pub captured: Vec<Tile>,
    pub alignments: Vec<Vec<Tile>>,
    pub double_free_three: bool,
    pub win: bool,
    pub oponent_win: bool,
}

const STRAIGHT_MOVES: [[Move; 2]; 4] = [
    [Move::Left, Move::Right],
    [Move::Top, Move::Bottom],
    [Move::TopLeft, Move::BottomRight],
    [Move::TopRight, Move::BottomLeft],
];

pub fn analyze_index(tile: Tile, game: &Game) -> AnalyzedTile {
    let mut free_threes: u8 = 0;
    let mut data = AnalyzedTile {
        captured: vec![],
        alignments: vec![],
        double_free_three: false,
        win: false,
        oponent_win: false,
    };

    let mut counter = 0;
    for moves in &STRAIGHT_MOVES {
        let num_moves = min(4, moves[0].num_move_to(game.board_size, tile));
        let num_moves_2 = min(4, moves[1].num_move_to(game.board_size, tile));

        let mut aligned = vec![tile];

        for move_index in 0..2 {
            if free_threes == 2 {
                return AnalyzedTile {
                    captured: vec![],
                    alignments: vec![],
                    double_free_three: true,
                    win: false,
                    oponent_win: false,
                };
            }

            let mut align_ok = true;

            let direction = &moves[move_index];
            let other_direction = if move_index == 0 {
                &moves[1]
            } else {
                &moves[0]
            };

            let mut i = tile;

            let mut capture_ok = true;
            let mut captured = vec![];

            let mut check_ft_sequence = if num_moves == 0 || num_moves_2 == 0 {
                false
            } else {
                true
            };

            let mut nb_stones = 0;
            let mut check_ft = 0;

            for count in 1..=num_moves {
                i = direction.get_next_tile(game.board_size, i);

                let tile_value = game.get_tile_value(i);
                if tile_value == game.current_player {
                    if count == 1 || count == 2 {
                        capture_ok = false
                    } else if count == 3 && capture_ok {
                        for capture in &captured {
                            data.captured.push(*capture);
                        }
                    }

                    if align_ok {
                        if count == 1 {
                            aligned.insert(0, i);
                        } else {
                            aligned.push(i);
                        }
                    }
                    if check_ft_sequence && check_ft == 0 {
                        nb_stones += 1;
                    }
                } else if tile_value == game.opponent_player {
                    if (count == 1 || count == 2) && capture_ok {
                        captured.push(i);
                    }

                    align_ok = false;
                    if check_ft == 0 && check_ft_sequence {
                        check_ft_sequence = false;
                    }
                } else {
                    if count < 3 {
                        capture_ok = false
                    }
                    if check_ft_sequence {
                        match nb_stones {
                            0 if count == 1 => {}
                            1 => check_ft = 2,
                            2 => check_ft = 1,
                            _ => check_ft_sequence = false,
                        }
                    }
                    align_ok = false;
                }
            }

            if check_ft != 0 && (move_index == 0 || check_ft == 1) {
                let first_op_tile = other_direction.get_next_tile(game.board_size, tile);
                let first_op = game.get_tile_value(first_op_tile);
                if check_ft == 2 {
                    let second_op_tile =
                        other_direction.get_next_tile(game.board_size, first_op_tile);
                    let second_op = game.get_tile_value(second_op_tile);
                    if second_op == 0 && first_op == 1 {
                        free_threes += 1;
                    }
                } else {
                    if first_op == 0 {
                        free_threes += 1;
                    }
                }
            }
        }

        if aligned.len() > 4 {
            let mut all_moves = STRAIGHT_MOVES.to_vec();
            all_moves.remove(counter);

            let capturable = get_capturable_indexes(
                &get_indexes_from_alignment(&aligned),
                game,
                &all_moves,
            );
            data.alignments.push(capturable);
        }

        counter += 1;
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
            let catchers = get_catcher_indexes(game);

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
    all_moves: &Vec<[Move; 2]>,
) -> Vec<Tile> {
    let mut capturable = vec![];

    for tile in aligned {
        for moves in all_moves {
            let first_move = &moves[0];
            let second_move = &moves[1];
            let tile = *tile;
            if !first_move.can_move_to(game.board_size, tile, 1) || !second_move.can_move_to(game.board_size, tile, 1) {
                continue;
            }

            let first_value = game.get_tile_value(first_move.get_next_tile(game.board_size, tile));
            let second_value = game.get_tile_value(second_move.get_next_tile(game.board_size, tile));

            if first_value == 1 && second_value != 1 {
                if !first_move.can_move_to(game.board_size, tile, 2) {
                    continue;
                }

                let edge_value = game.get_tile_value(first_move.get_tile_mult(game.board_size, tile, 2));
                if edge_value == 1 {
                    continue;
                }

                if edge_value != second_value {
                    capturable.push(tile);
                }
            } else if second_value == 1 && first_value != 1 {
                if !second_move.can_move_to(game.board_size, tile, 2) {
                    continue;
                }

                let edge_value =
                    game.get_tile_value(second_move.get_tile_mult(game.board_size, tile, 2));
                if edge_value == 1 {
                    continue;
                }

                if edge_value != first_value {
                    capturable.push(tile);
                }
            }
        }
    }

    capturable
}

fn get_catcher_indexes(game: &Game) -> HashMap<Tile, i32> {
    let mut catchers = HashMap::new();

    for tile in &game.get_player().last_hits {
        for moves in &STRAIGHT_MOVES {
            let first_move = &moves[0];
            let second_move = &moves[1];
            let tile = *tile;
            if !first_move.can_move_to(game.board_size, tile, 1) || !second_move.can_move_to(game.board_size, tile, 1) {
                continue;
            }

            let first_move_tile = first_move.get_next_tile(game.board_size, tile);
            let second_move_tile = second_move.get_next_tile(game.board_size, tile);

            let first_value = game.get_tile_value(first_move_tile);
            let second_value = game.get_tile_value(second_move_tile);

            if first_value == 1 && second_value != 1 {
                if !first_move.can_move_to(game.board_size, tile, 2) {
                    continue;
                }

                let edge_value_tile = first_move.get_tile_mult(game.board_size, tile, 2);

                let edge_value = game.get_tile_value(edge_value_tile);
                if edge_value == 1 {
                    continue;
                }

                if edge_value != second_value {
                    let value_index = if edge_value == 0 {
                        edge_value_tile
                    } else {
                        second_move_tile
                    };
                    let value = if let Some(value) = catchers.get(&value_index) {
                        *value + 1
                    } else {
                        1
                    };
                    catchers.insert(value_index, value);
                }
            } else if second_value == 1 && first_value != 1 {
                if !second_move.can_move_to(game.board_size, tile, 2) {
                    continue;
                }

                let edge_value_tile = second_move.get_tile_mult(game.board_size, tile, 2);
                let edge_value = game.get_tile_value(edge_value_tile);
                if edge_value == 1 {
                    continue;
                }

                if edge_value != first_value {
                    let value_index = if edge_value == 0 {
                        edge_value_tile
                    } else {
                        first_move_tile
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
