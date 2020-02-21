use crate::game::{JOINED_ACTIONS, Stones, move_stone, get_value, is_capturable};


// capturable : 01x2 0x12 21x0 2x10
// aligned 0x111...
// other_player_aligned x2222...


pub fn get_aligned_h(
    placed: &Stones,
    index: usize,
    board_size: usize,
    player: u8,
) -> i32{
    let mut h = 0;
    let other_player = if player == 1 {2} else {1};
    let mut can_capture = 0;
    let mut can_be_captured = false;

    let joined_action_vec: Vec<&str> = JOINED_ACTIONS.iter()
        .map(|&x| x)
        .collect();

    let other_player_can_capture = is_capturable(
        placed,
        index,
        board_size,
        player,
        other_player,
        &joined_action_vec,
    );

    if other_player_can_capture {
        println!("can be captured\n");
        h -= 10000000;
    }

    for actions in JOINED_ACTIONS.iter() {
        let mut aligned = 0;
        let mut space = 0;
        let mut not_aligned = 0;
        let mut maybe_next;
        let mut maybe_previous;




        for (j, action) in actions.split('|').into_iter().enumerate() {

            // check other player aligned // and capture enemy
            let mut other_player_aligned = 0;
            let mut other_player_index = move_stone(index, board_size, action);
            for x in 0..5 {
                if let Some(check_other_player) = other_player_index {
                    let value = get_value(placed, check_other_player);
                    if value == other_player {
                        other_player_aligned += 1;
                    } else {
                        if value == 0 {
                            other_player_aligned += 1;
                        } else if x == 2 {
                            can_capture += 1;
                        }
                        break;
                    }
                    aligned += 1;
                    other_player_index = move_stone(check_other_player, board_size, action);
                }
            }
            if other_player_aligned > 3 {
                h += 1000000000;
            } else if other_player_aligned > 2 {
                h += other_player_aligned * other_player_aligned;
            }

            // check this player aligned
            if j == 0 {
                maybe_previous = move_stone(index, board_size, action);
                while let Some(previous_i) = maybe_previous {
                    if get_value(placed, previous_i) != player {
                        break;
                    }
                    aligned += 1;
                    maybe_previous = move_stone(previous_i, board_size, action);
                }
                while let Some(previous_i) = maybe_previous {
                    let value = get_value(placed, previous_i);
                    if value == player {
                        not_aligned += 1;
                    } else if value == 0 {
                        space += 1;
                    } else {
                        break;
                    }
                    maybe_previous = move_stone(previous_i, board_size, action);
                }
            } else {
                maybe_next = move_stone(index, board_size, action);
                while let Some(next_i) = maybe_next {
                    if get_value(placed, next_i) != player {
                        break;
                    }
                    aligned += 1;
                    maybe_next = move_stone(next_i, board_size, action);
                }
                while let Some(next_i) = maybe_next {
                    let value = get_value(placed, next_i);
                    if value == player {
                        not_aligned += 1;
                    } else if value == 0 {
                        space += 1;
                    } else {
                        break;
                    }
                    maybe_next = move_stone(next_i, board_size, action);
                }
            }

        }
        if aligned + space > 4 {
            h += aligned * aligned;
        }
    }
    h + (can_capture * 100)
}

//fn get_aligned_length(
//    placed: &Stones,
//    index: usize,
//    board_size: usize,
//    player: u8,
//    action: &str,
//) -> i32 {
//    let mut stones = vec![];
//
//    let mut maybe_stone = move_stone(index, board_size, action);
//    while let Some(next_i) = maybe_stone {
//        let value = get_value(placed, next_i);
//        if value != player && value != 0 {
//            break;
//        }
//        stones.push(next_i);
//        maybe_stone = move_stone(next_i, board_size, action);
//    }
//    stones
//}
