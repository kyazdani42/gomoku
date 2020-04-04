use super::player::Player;
use super::r#move::Move;
use std::cmp::min;

pub struct AnalyzedIndex {
    pub captured: Vec<i32>,
    pub alignments: Vec<Vec<i32>>,
    pub double_free_three: bool,
}

const STRAIGHT_MOVES: [[Move; 2]; 4] = [
    [Move::Left, Move::Right],
    [Move::Top, Move::Bottom],
    [Move::TopLeft, Move::BottomRight],
    [Move::TopRight, Move::BottomLeft],
];

pub fn analyze_index(index: i32, player: &Player, oponent: &Player) -> AnalyzedIndex {
    let mut free_threes: u8 = 0;
    let mut data = AnalyzedIndex {
        captured: vec![],
        alignments: vec![],
        double_free_three: false,
    };

    for moves in &STRAIGHT_MOVES {
        let num_moves = min(4, moves[0].num_move_to(19, index));
        let num_moves_2 = min(4, moves[1].num_move_to(19, index));

        let mut aligned = vec![];
        let mut align_ok = true;

        for move_index in 0..2 {
            if free_threes == 2 {
                return AnalyzedIndex {
                    captured: vec![],
                    alignments: vec![],
                    double_free_three: true,
                };
            }

            let direction = &moves[move_index];
            let other_direction = if move_index == 0 { &moves[1] } else { &moves[0] };

            let mut i = index;

            let mut capture_ok = true;
            let mut captured = vec![];

            let mut check_ft_sequence = if num_moves == 0 || num_moves_2 == 0 {
                false
            } else {
                true
            };

            let mut check_ft = 0;
            let mut nb_stones = 0;

            for count in 1..=num_moves {
                i = direction.get_next_index(19, i);

                if player.contains(i) {
                    if count == 1 || count == 2 {
                        capture_ok = false
                    }
                    if align_ok == true {
                        aligned.push(i);
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
                    if count == 2 || count == 3 {
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

            if check_ft != 0 {
                let first_op_index = other_direction.get_next_index(19, index);
                let first_op = if player.contains(first_op_index) {
                    1
                } else if oponent.contains(first_op_index) {
                    2
                } else {
                    0
                };
                if check_ft == 2 {
                    let second_op_index = other_direction.get_next_index(19, first_op_index);
                    let second_op = if player.contains(second_op_index) {
                        1
                    } else if oponent.contains(second_op_index) {
                        2
                    } else {
                        0
                    };

                    if first_op == 1 && second_op == 0 {
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
            data.alignments.push(aligned);
        }
    }

    data
}
