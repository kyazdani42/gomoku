use std::cmp::{max, min};
use std::collections::HashSet;
use std::i32::{MAX, MIN};
use std::time::Instant;

use super::get_heuristics;
use crate::game::{
    add_empty_neighbours, check_double_free_threes, get_all_playable_indexes, set_free_threes,
    switch_player,
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

    let board = state.placed.clone();

    let time = Instant::now();
    let free_indexes = get_all_playable_indexes(&board, state.board_size);
    let (heuristic, best_index) = alphabeta(
        board,
        depth,
        true,
        MIN,
        MAX,
        state.player,
        state.last_played,
        state.board_size,
        &free_indexes,
    );

    unsafe {
        println!("For heuristic {} with {} hits:", heuristic, HITS);
        println!("total heuristic time: {}ms", TOTAL / 1_000_000);
        println!("total index recuperation time: {}ms", TOTAL2 / 1_000_000);
        println!("total cloning time: {}ms", TOTAL3 / 1_000_000);
        println!("total compute time: {}ms\n", time.elapsed().as_millis());
    }

    best_index
}

fn alphabeta(
    board: Stones,
    depth: u8,
    maximizing_player: bool,
    alpha: i32,
    beta: i32,
    player: u8,
    last_played: usize,
    board_size: usize,
    previous_free_indexes: &HashSet<usize>,
) -> (i32, usize) {
    unsafe { HITS += 1 };
    let mut alpha = alpha;
    let mut beta = beta;
    if depth == 0 {
        let time = Instant::now();
        let heuristic = get_heuristics(&board, last_played, board_size, player); // this take a long time to complete
                                                                                 // let heuristic = 1;
        unsafe {
            TOTAL += time.elapsed().as_nanos();
        }

        return (heuristic, 0);
    }

    let time = Instant::now();
    let mut indexes = previous_free_indexes.clone();
    indexes.remove(&last_played);
    add_empty_neighbours(&mut indexes, &board, last_played, board_size);
    unsafe {
        TOTAL2 += time.elapsed().as_nanos();
    }

    let mut best_value = if maximizing_player { MIN } else { MAX };
    let mut best_index = 0;

    for index in indexes
        .iter()
        .filter(|x| !check_double_free_threes(&board, **x, board_size, player))
    {
        let time = Instant::now();
        let mut new_board = board.clone();
        unsafe {
            TOTAL3 += time.elapsed().as_nanos();
        }
        new_board.insert(*index, player);

        let value = alphabeta(
            new_board,
            depth - 1,
            !maximizing_player,
            alpha,
            beta,
            switch_player(player),
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
