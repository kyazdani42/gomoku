use std::cmp::{max, min};
use std::i32::{MAX, MIN};
use std::time::Instant;

use super::get_heuristics;
use crate::game::GameState;
use crate::game::{get_all_playable_indexes, set_free_threes, switch_player};

pub fn compute(state: &GameState) -> usize {
    if state.placed.is_empty() {
        let board_size = state.board_size;
        return (board_size * board_size) / 2;
    }

    let depth = 2;

    let mut clone_state = state.clone();

    let time = Instant::now();
    let (_heuristic, best_index) = minimax(
        &mut clone_state,
        depth,
        true,
        state.player,
        state.last_played,
    );
    println!("{}ms\n", time.elapsed().as_millis());

    // println!("H: {}\n", best_value);

    best_index
}

fn minimax(
    state: &mut GameState,
    depth: u8,
    maximizing_player: bool,
    player: u8,
    last_played: usize,
) -> (i32, usize) {
    if depth == 0 {
        // node is a terminal node || heuristic == 0 {
        let heuristic = get_heuristics(&state.placed, last_played, state.board_size, player);

        print!("\x1b[32m{number:>width$}\x1b[0m", number=heuristic, width=3);
        (heuristic, state.last_played)
    } else if maximizing_player {
        let mut max_value = MIN;
        let mut best_index = 0;

        let indexes = get_all_playable_indexes(&state.placed, state.board_size);
        for index in indexes {
            state.placed.insert(index, state.player);

            print!("o -> ");
            let value = minimax(state, depth - 1, false, switch_player(player), index).0;
            if max_value < value {
                max_value = value;
                best_index = index;
            }

            state.placed.remove(&index);
        }

        print!("Max of childs: \x1b[33m{}\x1b[0m\n", max_value);
        (max_value, best_index)
    } else {
        let mut min_value = MAX;
        let indexes = get_all_playable_indexes(&state.placed, state.board_size);
        for index in indexes {
            state.placed.insert(index, state.player);

            min_value = min(
                min_value,
                minimax(state, depth - 1, true, switch_player(player), index).0,
            );

            state.placed.remove(&index);
        }

        print!(" -> \x1b[35m{}\x1b[0m\n", min_value);
        (min_value, 0)
    }
}
