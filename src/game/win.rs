use super::{
    can_oponent_capture, get_value, is_capturable, move_stone, switch_player, GameState, Stones,
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

    if alignments.len() == 0 {
        return;
    }

    state.winner = state.player;
    let alignment = alignments[0].clone();

    let not_done = alignment.brokable
        || (get_oponent_captures(state) == 8
            && can_oponent_capture(&state.placed, state.board_size, switch_player(state.player)));

    if not_done {
        state.alignment = Some(alignment.indexes);
        state.winner = 0;
    }
}

fn get_oponent_captures(state: &GameState) -> u8 {
    if state.player == 1 {
        state.p2_captured
    } else {
        state.p1_captured
    }
}

#[derive(Clone)]
struct Alignment {
    indexes: Vec<usize>,
    brokable: bool,
}

fn get_alignments(state: &GameState) -> Vec<Alignment> {
    let mut return_values: Vec<Alignment> = vec![];
    for actions in JOINED_ACTIONS.iter() {
        let mut aligned = vec![];
        for (j, action) in actions.split('|').into_iter().enumerate() {
            let stones = get_aligned_stones(
                &state.placed,
                state.last_played,
                state.board_size,
                state.player,
                action,
            );
            if j == 0 {
                aligned = stones;
            } else {
                aligned.insert(0, state.last_played);
                for v in stones.into_iter() {
                    aligned.insert(0, v);
                }
            }
        }

        if 4 < aligned.len() {
            let indexes = get_indexes(&aligned);
            let brokable = is_brokable(
                &state.placed,
                &indexes,
                state.player,
                state.board_size,
                *actions,
            );
            return_values.push(Alignment { indexes, brokable });
        }
    }

    return_values
}

fn is_brokable(
    placed: &Stones,
    alignment: &Vec<usize>,
    player: u8,
    board_size: usize,
    directions: &str,
) -> bool {
    let actions = JOINED_ACTIONS
        .iter()
        .filter(|x| *x != &directions)
        .map(|x| *x)
        .collect::<Vec<&str>>();
    for index in alignment {
        if is_capturable(
            placed,
            *index,
            board_size,
            player,
            switch_player(player),
            &actions,
        ) {
            return true;
        }
    }

    false
}

fn get_indexes(alignment: &Vec<usize>) -> Vec<usize> {
    let extremities = get_extremities(alignment);
    alignment
        .clone()
        .iter()
        .filter(|x| !extremities.contains(*x))
        .map(|x| *x)
        .collect()
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
