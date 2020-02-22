use super::{get_value, move_stone, GameState, Stones, ACTIONS};

pub fn set_free_threes(state: &mut GameState) {
    let mut index_to_check = Vec::new();

    state.placed.retain(|_, v| *v != 3);

    for (key, value) in &state.placed {
        if *value == state.player {
            for action in ACTIONS.iter() {
                let mut next = move_stone(*key, state.board_size, action);
                if let Some(stone) = next {
                    if get_value(&state.placed, stone) == 0 {
                        index_to_check.push(stone);
                        next = move_stone(stone, state.board_size, action);
                        if let Some(stone) = next {
                            if get_value(&state.placed, stone) == 0 {
                                index_to_check.push(stone);
                            }
                        }
                    }
                }
            }
        }
    }

    for value in &index_to_check {
        let is_double_free_three =
            check_double_free_threes(&state.placed, *value, state.board_size, state.player);
        if is_double_free_three {
            state.placed.insert(*value, 3);
        }
    }
}

pub fn check_double_free_threes(
    board: &Stones,
    index: usize,
    board_size: usize,
    player: u8,
) -> bool {
    let mut free_threes = 0;

    let actions = [
        ["bot_left", "top_right"],
        ["left", "right"],
        ["top", "bot"],
        ["top_left", "bot_right"],
        ["bot_right", "top_left"],
        ["bot", "top"],
        ["right", "left"],
        ["top_right", "bot_left"],
    ];
    for (i, action) in actions.iter().enumerate() {
        if is_free_threes(
            board,
            index,
            board_size,
            player,
            action[0],
            action[1],
            i > 3,
        ) {
            free_threes += 1;
        }
        if free_threes == 2 {
            return true;
        }
    }
    false
}

pub fn is_free_threes(
    board: &Stones,
    index: usize,
    board_size: usize,
    player: u8,
    action_one: &str,
    action_two: &str,
    second_check: bool,
) -> bool {
    let mut previous = move_stone(index, board_size, action_one);
    let mut next = move_stone(index, board_size, action_two);
    if previous.is_none() {
        return false;
    }

    let stone = previous.unwrap();
    if get_value(board, stone) == 0 {
        if let Some(stone) = next {
            if get_value(board, stone) == 0 {
                next = move_stone(stone, board_size, action_two);
                return check_free_pattern(
                    board,
                    next,
                    vec![player, player, 0],
                    action_two,
                    board_size,
                );
            } else if get_value(board, stone) == player {
                next = move_stone(stone, board_size, action_two);
                if let Some(stone) = next {
                    if get_value(board, stone) == player {
                        next = move_stone(stone, board_size, action_two);
                        if let Some(stone) = next {
                            if get_value(board, stone) == 0 {
                                return true;
                            }
                        }
                    } else if get_value(board, stone) == 0 {
                        next = move_stone(stone, board_size, action_two);
                        return check_free_pattern(
                            board,
                            next,
                            vec![player, 0],
                            action_two,
                            board_size,
                        );
                    }
                }
            }
        };
    } else if get_value(board, stone) == player {
        previous = move_stone(stone, board_size, action_one);
        if let Some(stone) = previous {
            if get_value(board, stone) == 0 {
                if let Some(stone) = next {
                    if get_value(board, stone) == player {
                        next = move_stone(stone, board_size, action_two);
                        if let Some(stone) = next {
                            if get_value(board, stone) == 0 {
                                return !second_check;
                            }
                        }
                    } else if get_value(board, stone) == 0 {
                        next = move_stone(stone, board_size, action_two);
                        return check_free_pattern(
                            board,
                            next,
                            vec![player, 0],
                            action_two,
                            board_size,
                        );
                    }
                }
            }
        }
    }
    false
}

fn check_free_pattern(
    placed: &Stones,
    index: Option<usize>,
    pattern: Vec<u8>,
    action: &str,
    board_size: usize,
) -> bool {
    let mut next = index;
    for (i, v) in pattern.iter().enumerate() {
        if let None = next {
            break;
        }
        let stone = next.unwrap();
        if *v != get_value(placed, stone) {
            break;
        }
        if i == pattern.len() - 1 {
            return true;
        }
        next = move_stone(stone, board_size, action);
    }
    false
}
