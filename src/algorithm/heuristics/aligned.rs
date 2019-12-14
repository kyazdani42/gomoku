use crate::game::{JOINED_ACTIONS, Stones, move_stone, get_value};

pub fn get_aligned_h(
    placed: &Stones,
    index: usize,
    board_size: usize,
    player: u8,
) -> i32{
    let mut h = 0;
    for actions in JOINED_ACTIONS.iter() {
        let mut aligned = 0;
        let mut space = 0;
        let mut not_aligned = 0;
        let mut maybe_next;
        let mut maybe_previous;
        for (j, action) in actions.split('|').into_iter().enumerate() {
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
    h
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
