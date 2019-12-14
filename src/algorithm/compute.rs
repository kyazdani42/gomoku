use std::cmp::{max, min};
use std::i32::{MAX, MIN};
use std::time::Instant;

use crate::game::GameState;
use crate::game::{get_all_playable_indexes, switch_player};
use crate::heuristics::{get_heuristics};

pub fn compute(state: &GameState) -> usize {
    let board_size = state.board_size;
    if state.placed.is_empty() {
        return (board_size * board_size) / 2;
    }

    let depth = 2;
    let time = Instant::now();

    let mut clone_state = state.clone();
    let mut best_value = MAX;
    let mut best_index = 0;

    for index in get_all_playable_indexes(&state.placed, board_size) {
        clone_state.placed.insert(index, state.player);

        let heuristic = minimax(&mut clone_state, depth - 1, false, switch_player(state.player));
        if heuristic < best_value {
            best_value = heuristic;
            best_index = index;
        }
        clone_state.placed.remove(&index);
    }

    println!("{}ms\n", time.elapsed().as_millis());

    // println!("H: {}\n", best_value);

    best_index
}

fn minimax(state: &mut GameState, depth: u8, maximizing_player: bool, player: u8) -> i32 {
    let heuristic = get_heuristics(&state.placed, state.last_played, state.board_size, player);;
    if depth == 0 || heuristic == 0 {
        heuristic
    } else if maximizing_player {
        let mut max_value = MIN;

        for index in get_all_playable_indexes(&state.placed, state.board_size) {
            state.placed.insert(index, player);

            max_value = max(
                max_value,
                minimax(state, depth - 1, false, switch_player(player)),
            );

            state.placed.remove(&index);
        }

        max_value
    } else {
        let mut min_value = MAX;

        for index in get_all_playable_indexes(&state.placed, state.board_size) {
            state.placed.insert(index, player);

            min_value = min(
                min_value,
                minimax(state, depth - 1, true, switch_player(player)),
            );

            state.placed.remove(&index);
        }

        min_value
    }
}
