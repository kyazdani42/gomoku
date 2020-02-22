use std::cmp::{max, min};
use std::collections::HashSet;
use std::i32::{MAX, MIN};
use std::time::Instant;

use super::get_heuristics;
use crate::game::{
    add_empty_neighbours, check_double_free_threes, get_all_playable_indexes, switch_player,
};
use crate::game::{GameState, Stones};

static mut TOTAL: u128 = 0;
static mut TOTAL2: u128 = 0;
static mut TOTAL3: u128 = 0;
static mut HITS: u128 = 0;

pub fn compute(state: &GameState) -> usize {
    if state.placed.is_empty() {
        let board_size = state.board_size;
        return (board_size * board_size) / 2;
    }

    let depth = 4;

    unsafe {
        TOTAL = 0;
        TOTAL2 = 0;
        TOTAL3 = 0;
        HITS = 0;
    }

    let time = Instant::now();
    let free_indexes = get_all_playable_indexes(&state.placed, state.board_size);

    let (heuristic, best_index) = alphabeta(
        &state.placed,
        depth,
        true,
        MIN,
        MAX,
        switch_player(state.player),
        state.last_played,
        state.board_size,
        &free_indexes,
    );

    unsafe {
        println!(
            "For heuristic {}, index {} with {} hits:",
            heuristic, best_index, HITS
        );
        println!("total heuristic time: {}ms", TOTAL / 1_000_000);
        println!("total index recuperation time: {}ms", TOTAL2 / 1_000_000);
        println!("total cloning time: {}ms", TOTAL3 / 1_000_000);
        println!("total compute time: {}ms\n", time.elapsed().as_millis());
    }

    best_index
}

fn alphabeta(
    board: &Stones,
    depth: u8,
    maximizing_player: bool,
    alpha: i32,
    beta: i32,
    cur_player: u8,
    cur_index: usize,
    board_size: usize,
    previous_free_indexes: &HashSet<usize>,
) -> (i32, usize) {
    unsafe { HITS += 1 };
    let mut alpha = alpha;
    let mut beta = beta;
    if depth == 0 {
        let time = Instant::now();
        let heuristic = get_heuristics(&board, cur_index, board_size, cur_player);

        unsafe {
            TOTAL += time.elapsed().as_nanos();
        }

        return (heuristic, 0);
    }

    // TODO: check capture and remove from board if capture, if so,
    // update the empty neighbours by adding the captures into them
    let time = Instant::now();
    let mut indexes = previous_free_indexes.clone();
    indexes.remove(&cur_index);
    add_empty_neighbours(&mut indexes, &board, cur_index, board_size);
    unsafe {
        TOTAL2 += time.elapsed().as_nanos();
    }

    let mut best_value = if maximizing_player { MIN } else { MAX };
    let mut best_index = 0;


    let next_player = switch_player(cur_player);

    for index in indexes
        .iter()
        .filter(|x| !check_double_free_threes(&board, **x, board_size, cur_player))
    {
        let time = Instant::now();
        let mut new_board = board.clone();
        new_board.insert(*index, next_player);
        unsafe {
            TOTAL3 += time.elapsed().as_nanos();
        }

        let value = alphabeta(
            &new_board,
            depth - 1,
            !maximizing_player,
            alpha,
            beta,
            next_player,
            *index,
            board_size,
            &indexes,
        )
        .0;

        if maximizing_player {
            if best_value < value {
                best_value = value;
                best_index = *index;
            }
            alpha = max(alpha, value);
            if beta < alpha {
                break;
            }
        } else {
            best_value = min(best_value, value);
            beta = min(beta, best_value);
            if alpha < beta {
                break;
            }
        }
    }

    (best_value, best_index)
}
