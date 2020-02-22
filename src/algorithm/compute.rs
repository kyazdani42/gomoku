use std::cmp::{max, min};
use std::i32::{MAX, MIN};
use std::time::Instant;

use super::get_heuristics;
use crate::game::{GameState,Stones};
use crate::game::{get_all_playable_indexes, set_free_threes, switch_player};

static mut TOTAL: i32 = 0;
pub fn compute(state: &GameState) -> usize {
    if state.placed.is_empty() {
        let board_size = state.board_size;
        return (board_size * board_size) / 2;
    }

    let depth = 4;

    unsafe { TOTAL = 0; }

    let board = state.placed.clone();

    let time = Instant::now();
    let (_heuristic, best_index) = minimax(
        board,
        depth,
        true,
        MIN,
        MAX,
        state.player,
        state.last_played,
        state.board_size,
    );
    unsafe { println!("{}\n", TOTAL); }
    println!("{}ms\n", time.elapsed().as_millis());

    // println!("H: {}\n", best_value);

    best_index
}


fn minimax(
    board: Stones,
    depth: u8,
    maximizing_player: bool,
    alpha: i32,
    beta: i32,
    player: u8,
    last_played: usize,
    board_size: usize,
) -> (i32, usize) {
    let mut alpha = alpha;
    let mut beta  = beta;
    if depth == 0 {
        // node is a terminal node || heuristic == 0 {
        let heuristic = get_heuristics(&board, last_played, board_size, player);

        unsafe { TOTAL += 1; }
        (heuristic, 0)
    } else if maximizing_player {
        let mut max_value = MIN;
        let mut best_index = 0;

        let indexes = get_all_playable_indexes(&board, board_size);
        for index in indexes {
            let mut new_board = board.clone();
            new_board.insert(index, player);

            let value = minimax(new_board, depth - 1, false, alpha, beta, switch_player(player), index, board_size).0;
            if max_value < value {
                max_value = value;
                best_index = index;
            }
            alpha = max(alpha, value);
            if beta < alpha {
                break;
            }
        }

        (max_value, best_index)
    } else {
        let mut min_value = MAX;
        let indexes = get_all_playable_indexes(&board, board_size);
        for index in indexes {
            let mut new_board = board.clone();
            new_board.insert(index, player);

            min_value = min(
                min_value,
                minimax(new_board, depth - 1, true, alpha, beta, switch_player(player), index, board_size).0,
            );
            beta = min(beta, min_value);
            if alpha < beta {
                break;
            }
        }

        (min_value, 0)
    }
}

