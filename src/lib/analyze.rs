use super::player::Player;
use super::r#move::Move;
use std::cmp::min;

type Capturable = Vec<i32>;

pub struct AnalyzedIndex {
    pub captured: Vec<i32>,
    pub alignments: Vec<Capturable>,
    pub double_free_three: bool,
    pub win: bool,
    pub oponent_win: bool
}

const STRAIGHT_MOVES: [[Move; 2]; 4] = [
    [Move::Left, Move::Right],
    [Move::Top, Move::Bottom],
    [Move::TopLeft, Move::BottomRight],
    [Move::TopRight, Move::BottomLeft],
];

pub fn analyze_index(
    index: i32,
    board_size: i32,
    player: &Player,
    oponent: &Player,
    oponent_alignments: &Vec<Vec<i32>>
) -> AnalyzedIndex {
    let mut free_threes: u8 = 0;
    let mut data = AnalyzedIndex {
        captured: vec![],
        alignments: vec![],
        double_free_three: false,
        win: false,
        oponent_win: false
    };

    let mut counter = 0;
    for moves in &STRAIGHT_MOVES {
        let num_moves = min(4, moves[0].num_move_to(board_size, index));
        let num_moves_2 = min(4, moves[1].num_move_to(board_size, index));

        let mut aligned = vec![index];

        for move_index in 0..2 {
            if free_threes == 2 {
                return AnalyzedIndex {
                    captured: vec![],
                    alignments: vec![],
                    double_free_three: true,
                    win: false,
                    oponent_win: false
                };
            }

            let mut align_ok = true;

            let direction = &moves[move_index];
            let other_direction = if move_index == 0 {
                &moves[1]
            } else {
                &moves[0]
            };

            let mut i = index;

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
                i = direction.get_next_index(board_size, i);

                if player.contains(i) {
                    if count == 1 || count == 2 {
                        capture_ok = false
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
                } else if oponent.contains(i) {
                    if (count == 1 || count == 2) && capture_ok {
                        captured.push(i);
                    } else if count == 3 {
                        capture_ok = false;
                    }
                    align_ok = false;
                    if check_ft == 0 && check_ft_sequence {
                        check_ft_sequence = false;
                    }
                } else {
                    if count < 4 {
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
                let first_op_index = other_direction.get_next_index(board_size, index);
                let first_op = if player.contains(first_op_index) {
                    1
                } else if oponent.contains(first_op_index) {
                    2
                } else {
                    0
                };
                if check_ft == 2 {
                    let second_op_index =
                        other_direction.get_next_index(board_size, first_op_index);
                    if !player.contains(second_op_index)
                        && !oponent.contains(second_op_index)
                        && first_op == 1
                    {
                        free_threes += 1;
                    }
                } else {
                    if first_op == 0 {
                        free_threes += 1;
                    }
                }
            }

            if capture_ok {
                for capture in captured {
                    data.captured.push(capture);
                }
            }
        }

        if aligned.len() > 4 {
            let mut all_moves = STRAIGHT_MOVES.to_vec();
            all_moves.remove(counter);

            let capturable = get_capturable_indexes(
                &get_indexes_from_alignment(&aligned),
                player,
                oponent,
                &all_moves,
            );
            data.alignments.push(capturable);
        }

        counter += 1;
    }

    if player.captured + data.captured.len() as u8 > 9 {
        data.win = true
    } else if oponent_alignments.len() > 0 {
        for al in oponent_alignments {
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
            // TODO: checker toutes les pieces du joueur afin de determiner s'il peut gagner par capture
            data.win = true; // todo <
        }
    }

    data
}

fn get_indexes_from_alignment(alignment: &Vec<i32>) -> Vec<i32> {
    match alignment.len() {
        6 => vec![alignment[1], alignment[2], alignment[3], alignment[4]],
        7 => vec![alignment[2], alignment[3], alignment[4]],
        8 => vec![alignment[3], alignment[4]],
        9 => vec![alignment[4]],
        _ => alignment.clone(),
    }
}

fn get_capturable_indexes(
    aligned: &Vec<i32>,
    player: &Player,
    oponent: &Player,
    all_moves: &Vec<[Move; 2]>,
) -> Vec<i32> {
    let mut capturable = vec![];

    for tile in aligned {
        for moves in all_moves {
            let first_move = &moves[0];
            let second_move = &moves[1];
            let tile = *tile;
            if !first_move.can_move_to(19, tile, 1) || !second_move.can_move_to(19, tile, 1) {
                continue;
            }

            let first_value = get_value(player, oponent, first_move.get_next_index(19, tile));
            let second_value = get_value(player, oponent, second_move.get_next_index(19, tile));

            if first_value == 1 && second_value != 1 {
                if !first_move.can_move_to(19, tile, 2) {
                    continue;
                }

                let edge_value = get_value(player, oponent, first_move.get_index_mult(19, tile, 2));
                if edge_value == 1 {
                    continue;
                }

                if edge_value != second_value {
                    capturable.push(tile);
                }
            } else if second_value == 1 && first_value != 1 {
                if !second_move.can_move_to(19, tile, 2) {
                    continue;
                }

                let edge_value =
                    get_value(player, oponent, second_move.get_index_mult(19, tile, 2));
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

fn get_value(player: &Player, oponent: &Player, index: i32) -> u8 {
    if player.contains(index) {
        1
    } else if oponent.contains(index) {
        2
    } else {
        0
    }
}
