use super::{get_value, move_stone, switch_player, GameState, Stones, ACTIONS};

pub fn capture_all(state: &mut GameState) {
    let other_player = switch_player(state.player);
    ACTIONS.iter().for_each(|action| {
        let indexes = get_captured_indexes(
            &state.placed,
            state.last_played,
            state.board_size,
            action,
            state.player,
            other_player,
        );

        if let Some([i, j]) = indexes {
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

pub fn get_captured_indexes(
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

pub fn capturable(placed: &Stones, index: usize, board_size: usize, player: u8) -> bool {
    let other_player = switch_player(player);
    for (i, value) in placed {
        if *value == other_player {
            for action in ACTIONS.iter() {
                if let Some([x, y]) =
                    get_potential_captured_indexes(placed, *i, board_size, action, player)
                {
                    if x == index || y == index {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn check_all_captures(placed: &Stones, board_size: usize, player: u8) -> bool {
    let other_player = switch_player(player);
    for (index, value) in placed {
        if *value == player {
            for action in ACTIONS.iter() {
                if let Some(_) =
                    get_potential_captured_indexes(placed, *index, board_size, action, other_player)
                {
                    return true;
                }
            }
        }
    }
    false
}

pub fn get_potential_captured_indexes(
    placed: &Stones,
    index: usize,
    board_size: usize,
    action: &str,
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
        if get_value(placed, index) == 0 {
            return Some([i, j]);
        }
    };

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    mod check_all_captures {
        use super::check_all_captures;
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

            assert_eq!(check_all_captures(&placed, board_size, player), true);
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
            assert_eq!(check_all_captures(&placed, board_size, player), false);
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
