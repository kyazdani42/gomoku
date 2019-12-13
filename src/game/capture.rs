use super::{get_value, move_stone, switch_player, GameState, Stones, ACTIONS};

pub fn capture_all(state: &mut GameState) {
    let other_player = switch_player(state.player);
    ACTIONS.iter().for_each(|action| {
        if let Some([i, j]) = get_captured_indexes(
            &state.placed,
            state.last_played,
            state.board_size,
            action,
            state.player,
            other_player,
        ) {
            state.placed.remove(&i);
            state.placed.remove(&j);
            if state.player == 1 {
                state.p1_captured += 2;
            } else {
                state.p2_captured += 2;
            }
        }
    });
}

fn get_captured_indexes(
    placed: &Stones,
    index: usize,
    board_size: usize,
    action: &str,
    player: u8,
    other_player: u8,
) -> Option<[usize; 2]> {
    let board_size = board_size;
    let i = match move_stone(index, board_size, action) {
        Some(i) if get_value(placed, i) == other_player => i,
        _ => return None,
    };
    let j = match move_stone(i, board_size, action) {
        Some(j) if get_value(placed, j) == other_player => j,
        _ => return None,
    };

    if let Some(index) = move_stone(j, board_size, action) {
        if get_value(placed, index) == player {
            return Some([i, j]);
        }
    };

    None
}

// Check if an index matches a capture pattern that can be filled
// by the oponent
pub fn is_capturable(
    placed: &Stones,
    index: usize,
    board_size: usize,
    player: u8,
    player2: u8,
    actions: &Vec<&str>,
) -> bool {
    for action in actions {
        let split = action.split('|').collect::<Vec<&str>>();
        let dir1 = split[0];
        let dir2 = split[1];
        let (i_v, i_i) = match move_stone(index, board_size, dir1) {
            Some(i) => (get_value(placed, i), i),
            _ => continue,
        };

        let (j_v, j_i) = match move_stone(index, board_size, dir2) {
            Some(i) => (get_value(placed, i), i),
            _ => continue,
        };

        let (prev_index, side_value, dir) = if i_v == player {
            (i_i, j_v, dir1)
        } else if j_v == player {
            (j_i, i_v, dir2)
        } else {
            continue;
        };

        if side_value == player2 || side_value == 0 {
            if let Some(i) = move_stone(prev_index, board_size, dir) {
                let value = get_value(placed, i);
                if (side_value == player2 && value == 0) || (side_value == 0 && value == player2) {
                    return true;
                }
            }
        }
    }

    false
}

pub fn can_oponent_capture(placed: &Stones, board_size: usize, player: u8) -> bool {
    let other_player = switch_player(player);
    for (index, value) in placed {
        if *value == player {
            for action in ACTIONS.iter() {
                if get_potential_captured_indexes(placed, *index, board_size, action, other_player)
                {
                    return true;
                }
            }
        }
    }
    false
}

fn get_potential_captured_indexes(
    placed: &Stones,
    index: usize,
    board_size: usize,
    action: &str,
    other_player: u8,
) -> bool {
    let board_size = board_size;
    let i = match move_stone(index, board_size, action) {
        Some(i) if get_value(placed, i) == other_player => i,
        _ => return false,
    };
    let j = match move_stone(i, board_size, action) {
        Some(j) if get_value(placed, j) == other_player => j,
        _ => return false,
    };

    if let Some(index) = move_stone(j, board_size, action) {
        if get_value(placed, index) == 0 {
            return true;
        }
    };

    false
}


#[cfg(test)]
mod tests {
    use super::*;

    mod can_oponent_capture {
        use super::can_oponent_capture;
        use std::collections::HashMap;

        #[test]
        fn return_true() {
            let mut placed = HashMap::new();
            placed.insert(1, 1);
            placed.insert(2, 2);
            placed.insert(3, 2);
            placed.insert(4, 0);

            let board_size = 5;
            let player = 1;

            assert_eq!(can_oponent_capture(&placed, board_size, player), true);
        }

        #[test]
        fn return_false() {
            let mut placed = HashMap::new();
            placed.insert(1, 1);
            placed.insert(2, 2);
            placed.insert(3, 2);
            placed.insert(4, 0);

            let board_size = 5;
            let player = 2;
            assert_eq!(can_oponent_capture(&placed, board_size, player), false);
        }
    }

    mod get_captured_indexes {
        use super::get_captured_indexes;
        use std::collections::HashMap;
        #[test]
        fn capture_top() {
            let board_size = 4;
            let player = 1;
            let other_player = 2;
            let mut placed = HashMap::new();
            placed.insert(0, 1);
            placed.insert(4, 2);
            placed.insert(8, 2);
            placed.insert(12, 1);
            let at = 12;
            let indexes =
                get_captured_indexes(&placed, at, board_size, "top", player, other_player);
            assert_eq!(indexes.is_some(), true);
        }

        #[test]
        fn capture_bot() {
            let board_size = 4;
            let player = 1;
            let other_player = 2;
            let mut placed = HashMap::new();
            placed.insert(0, 1);
            placed.insert(4, 2);
            placed.insert(8, 2);
            placed.insert(12, 1);
            let at = 0;
            let indexes =
                get_captured_indexes(&placed, at, board_size, "bot", player, other_player);
            assert_eq!(indexes.is_some(), true);
        }

        #[test]
        fn capture_left() {
            let board_size = 4;
            let player = 1;
            let other_player = 2;
            let mut placed = HashMap::new();
            placed.insert(0, 1);
            placed.insert(1, 2);
            placed.insert(2, 2);
            placed.insert(3, 1);
            let at = 3;
            let indexes =
                get_captured_indexes(&placed, at, board_size, "left", player, other_player);
            assert_eq!(indexes.is_some(), true);
        }

        #[test]
        fn capture_right() {
            let board_size = 4;
            let player = 1;
            let other_player = 2;
            let mut placed = HashMap::new();
            placed.insert(0, 1);
            placed.insert(1, 2);
            placed.insert(2, 2);
            placed.insert(3, 1);
            let at = 0;
            let indexes =
                get_captured_indexes(&placed, at, board_size, "right", player, other_player);
            assert_eq!(indexes.is_some(), true);
        }
    }
}
