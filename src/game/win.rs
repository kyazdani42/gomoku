use super::{
    capturable, check_all_captures, get_value, move_stone, switch_player, GameState, Stones,
    JOINED_ACTIONS,
};

pub fn win_by_capture(state: &GameState) -> bool {
    if state.player == 1 {
        state.p1_captured == 10
    } else {
        state.p2_captured == 10
    }
}

pub fn win_by_alignment(state: &mut GameState) {
    let alignments = get_alignments(state);
    if alignments.len() > 0 {
        state.winner = state.player;
        let alignment = alignments[0].clone();
        if get_oponent_captures(state) == 8 {
            if check_all_captures(&state.placed, state.board_size, switch_player(state.player)) {
                state.alignment = Some(alignment);
                state.winner = 0;
            }
        } else {
            if let Some(alignment) =
                brokable(alignment, &state.placed, state.board_size, state.player)
            {
                state.alignment = Some(alignment);
                state.winner = 0;
            }
        }
    }
}

fn get_oponent_captures(state: &GameState) -> u8 {
    if state.player == 1 {
        state.p2_captured
    } else {
        state.p1_captured
    }
}

fn get_alignments(state: &GameState) -> Vec<Vec<usize>> {
    let mut aligned = vec![];
    for (i, actions) in JOINED_ACTIONS.iter().enumerate() {
        aligned.push(vec![]);
        for (j, action) in actions.split('|').into_iter().enumerate() {
            let stones = get_aligned_stones(
                &state.placed,
                state.last_played,
                state.board_size,
                state.player,
                action,
            );
            if j == 0 {
                for v in stones.into_iter() {
                    aligned[i].push(v);
                }
            } else {
                aligned[i].insert(0, state.last_played);
                for v in stones.into_iter() {
                    aligned[i].insert(0, v);
                }
            }
        }
    }

    let mut return_values: Vec<Vec<usize>> = vec![];
    for value in aligned.into_iter() {
        if 4 < value.len() {
            return_values.push(value.clone());
        }
    }
    return_values
}

fn get_aligned_stones(
    placed: &Stones,
    index: usize,
    board_size: usize,
    player: u8,
    action: &str,
) -> Vec<usize> {
    let mut stones = vec![];

    let mut maybe_stone = move_stone(index, board_size, action);
    while let Some(next_i) = maybe_stone {
        if get_value(placed, next_i) != player {
            break;
        }
        stones.push(next_i);
        maybe_stone = move_stone(next_i, board_size, action);
    }
    stones
}

fn brokable(
    alignment: Vec<usize>,
    placed: &Stones,
    board_size: usize,
    player: u8,
) -> Option<Vec<usize>> {
    let extremities = get_extremities(&alignment);
    for (i, index) in alignment.clone().iter().enumerate() {
        if extremities.contains(&i) == false {
            if capturable(placed, *index, board_size, player) {
                return Some(
                    alignment
                        .iter()
                        .filter(|v| !extremities.contains(*v))
                        .map(|x| *x)
                        .collect(),
                );
            }
        }
    }

    None
}

fn get_extremities(alignment: &Vec<usize>) -> Vec<usize> {
    match alignment.len() {
        6 => vec![0, 5],
        7 => vec![0, 1, 5, 6],
        8 => vec![0, 1, 2, 5, 6, 7],
        9 => vec![0, 1, 2, 3, 5, 6, 7, 8],
        _ => vec![],
    }
}

pub fn check_alignment_validity(placed: &Stones, alignment: &Vec<usize>) -> bool {
    for index in alignment.iter() {
        if placed.contains_key(index) == false {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    mod check_alignment_validity {
        use super::check_alignment_validity;
        use std::collections::HashMap;

        #[test]
        fn assert_true() {
            let alignment = vec![1, 2, 3, 4, 5];
            let mut placed = HashMap::new();
            placed.insert(1, 1);
            placed.insert(2, 1);
            placed.insert(3, 1);
            placed.insert(4, 1);
            placed.insert(5, 1);

            assert_eq!(check_alignment_validity(&placed, &alignment), true);
        }

        #[test]
        fn assert_false() {
            let alignment = vec![2, 3, 4, 5, 6];
            let mut placed = HashMap::new();
            placed.insert(2, 1);
            placed.insert(3, 1);
            placed.insert(4, 1);
            placed.insert(5, 1);

            assert_eq!(check_alignment_validity(&placed, &alignment), false);
        }
    }

    mod win_by_alignment {
        use super::{win_by_alignment, GameState, Stones};
        use std::collections::HashMap;

        fn get_state(
            placed: Stones,
            alignment: Option<Vec<usize>>,
            captures: u8,
            last_played: usize,
        ) -> GameState {
            let mut state = GameState::new();

            state.player = 1;
            state.p2_captured = captures;
            state.alignment = alignment;
            state.placed = placed;
            state.board_size = 5;
            state.last_played = last_played;

            state
        }

        #[test]
        fn test_no_alignment() {
            let mut placed = HashMap::new();
            placed.insert(0, 1);
            placed.insert(1, 1);
            placed.insert(2, 1);
            placed.insert(3, 1);
            placed.insert(8, 1);
            let mut state = get_state(placed, None, 0, 0);
            win_by_alignment(&mut state);
            assert_eq!(state.winner, 0);
            assert_eq!(state.alignment, None);
        }

        #[test]
        fn test_simple_alignment() {
            let mut placed = HashMap::new();
            placed.insert(0, 1);
            placed.insert(1, 1);
            placed.insert(2, 1);
            placed.insert(3, 1);
            placed.insert(4, 1);
            let mut state = get_state(placed, None, 0, 0);
            win_by_alignment(&mut state);
            assert_eq!(state.winner, 1);
            assert_eq!(state.alignment, None);
        }

        #[test]
        fn test_stopped_alignment() {
            let mut placed = HashMap::new();
            placed.insert(0, 2);
            placed.insert(5, 1);
            placed.insert(6, 1);
            placed.insert(7, 1);
            placed.insert(8, 1);
            placed.insert(9, 1);
            placed.insert(10, 1);
            let mut state = get_state(placed, None, 0, 5);

            win_by_alignment(&mut state);
            assert_eq!(state.winner, 0);

            let alignment = vec![5, 6, 7, 8, 9];
            for x in state.alignment.unwrap().iter() {
                assert_eq!(alignment.contains(x), true);
            }
        }

        #[test]
        fn test_oponent_4_captures_can_capture() {
            let mut placed = HashMap::new();
            placed.insert(0, 2);
            placed.insert(5, 1);
            placed.insert(6, 1);
            placed.insert(7, 1);
            placed.insert(8, 1);
            placed.insert(9, 1);
            placed.insert(10, 1);
            let mut state = get_state(placed, None, 8, 5);

            win_by_alignment(&mut state);
            assert_eq!(state.winner, 0);

            let alignment = vec![5, 6, 7, 8, 9];
            for x in state.alignment.unwrap().iter() {
                assert_eq!(alignment.contains(x), true);
            }
        }

        #[test]
        fn test_oponent_4_captures_can_not_capture() {
            let mut placed = HashMap::new();
            placed.insert(0, 1);
            placed.insert(1, 1);
            placed.insert(2, 1);
            placed.insert(3, 1);
            placed.insert(4, 1);
            placed.insert(5, 2);
            let mut state = get_state(placed, None, 8, 0);
            win_by_alignment(&mut state);
            assert_eq!(state.winner, 1);
            assert_eq!(state.alignment, None);
        }
    }
}
