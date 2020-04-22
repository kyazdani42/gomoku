// extern crate rand;
// use rand::{Rng,thread_rng};

use super::heuristic::heuristic;
use crate::lib::game::{Game, Tile};

use std::i32::{MAX, MIN};

static mut ANALYZER_TIME: u128 = 0;
static mut ANALYZER_NUM: i32 = 0;

static mut HEURISTIC_TIME: u128 = 0;
static mut HEURISTIC_NUM: i32 = 0;

static mut UPDATE: u128 = 0;
static mut UPDATE_NUM: i32 = 0;
static mut UPDATE_INSERT: u128 = 0;
static mut UPDATE_EMPTY: u128 = 0;
static mut UPDATE_ALIGN: u128 = 0;
static mut UPDATE_CAPTURE: u128 = 0;

static mut RESET_TIME: u128 = 0;

use std::time::Instant;

pub fn run(game: Game, level: u8) -> Vec<Tile> {
    unsafe {
        ANALYZER_TIME = 0;
        ANALYZER_NUM = 0;
        UPDATE = 0;
        UPDATE_NUM = 0;
        RESET_TIME = 0;
        HEURISTIC_TIME = 0;
        HEURISTIC_NUM = 0;

        UPDATE_EMPTY = 0;
        UPDATE_ALIGN = 0;
        UPDATE_INSERT = 0;
        UPDATE_CAPTURE = 0;
    }

    let mut best_hits = vec![];
    let neighbours = game.neighbours.clone();

    let depth = level as i32 * 2;
    let mut alpha = MIN;
    let mut best_tree: Vec<Tile> = vec![(0, 0); depth as usize - 1];
    for tile in neighbours {
        if game.get_tile_value(tile) != 0 {
            continue;
        }
        unsafe {
            ANALYZER_NUM += 1;
        }
        let now = Instant::now();
        let data = game.analyze(tile);
        unsafe {
            ANALYZER_TIME += now.elapsed().as_nanos();
        }

        if data.double_free_three {
            continue;
        }

        if data.win {
            best_hits.push((tile, MAX));
            break;
        } else if data.oponent_win {
            best_hits.push((tile, MIN));
        } else {
            let mut game = game.clone();
            game.update_game(tile, &data.alignments, &data.captured);
            let value = alphabeta(&mut game, depth - 1, alpha, MAX, false, &mut best_tree);
            if value > alpha {
                alpha = value;
            }
            best_hits.push((tile, value));
        }
    }

    unsafe {
        let a = "\x1b[1m";
        let b = "\x1b[0m";
        println!("analyzed {}called{} {} times, {}lasted{} {}ms", a, b, ANALYZER_NUM,a,b, ANALYZER_TIME / 1_000_000);
        println!("heuristic {}called{} {} times, {}lasted{} {}ms", a,b, HEURISTIC_NUM,a,b, HEURISTIC_TIME / 1_000_000);
        println!("{}reset{} lasted {}ms", a,b,RESET_TIME / 1_000_000);
        println!("updates {}called{} {} times, {}lasted{} {}ms", a,b,UPDATE_NUM,a,b, UPDATE / 1_000_000);
        print!("{}insert:{} {}ms, ", a, b, UPDATE_INSERT / 1_000_000);
        print!("{}empty:{} {}ms, ", a, b, UPDATE_EMPTY / 1_000_000);
        print!("{}align:{} {}ms, ", a, b, UPDATE_ALIGN / 1_000_000);
        println!("{}captures:{} {}ms\n", a, b, UPDATE_CAPTURE / 1_000_000);
    }

    for (i, v) in best_tree.iter().enumerate() {
        println!("runner {}: ({}, {})", i, v.0, v.1);
    }

    best_hits.sort_by(|a, b| b.1.cmp(&a.1));
    println!("heuristics:");
    for v in &best_hits {
        print!("| {} ", v.1);
    }
    println!("\n");
    best_hits.iter().map(|v| v.0).collect()
}

fn alphabeta(
    game: &mut Game,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: bool,
    best_tree: &mut Vec<Tile>
) -> i32 {
    if depth < 1 {
        let now = Instant::now();
        let h = heuristic(game, maximizing_player);
        // let h = thread_rng().gen();
        unsafe {
            HEURISTIC_TIME += now.elapsed().as_nanos();
            HEURISTIC_NUM += 1;
        }
        return h;
    }

    let neighbours = game.neighbours.clone();
    let old_alignment = game.opponent_alignments.clone();
    let mut value = if maximizing_player { MIN } else { MAX };

    for tile in &neighbours {
        let tile = *tile;
        if game.get_tile_value(tile) != 0 {
            continue;
        }
        let now = Instant::now();
        let data = game.analyze(tile);
        unsafe {
            ANALYZER_TIME += now.elapsed().as_nanos();
            ANALYZER_NUM += 1;
        }

        if data.double_free_three {
            continue;
        } else if data.win {
            return if maximizing_player {
                4000 - depth
            } else {
                -4000 + depth
            };
        } else if data.oponent_win {
            return if maximizing_player {
                -4000 + depth
            } else {
                4000 - depth
            };
        } else {
            let now = Instant::now();

            let now_2 = Instant::now();
            game.insert_tile(tile);
            unsafe {
                UPDATE_INSERT += now_2.elapsed().as_nanos();
            }

            let now_2 = Instant::now();
            game.update_opponent_alignments(&data.alignments);
            unsafe {
                UPDATE_ALIGN += now_2.elapsed().as_nanos();
            }

            let now_2 = Instant::now();
            game.update_captures(&data.captured);
            unsafe {
                UPDATE_CAPTURE += now_2.elapsed().as_nanos();
            }

            // do not update neighbours when we run the heuristic
            if depth > 1 {
                let now_2 = Instant::now();
                game.update_neighbours(tile);
                unsafe {
                    UPDATE_EMPTY += now_2.elapsed().as_nanos();
                }
            }

            game.switch_player();

            unsafe {
                UPDATE += now.elapsed().as_nanos();
                UPDATE_NUM += 1;
            }

            let alphabeta_value = alphabeta(game, depth - 1, alpha, beta, !maximizing_player, best_tree);
            if maximizing_player {
                if alphabeta_value > value {
                    value = alphabeta_value;
                    alpha = alphabeta_value;
                    best_tree[depth as usize - 1] = tile;
                }
            } else {
                if alphabeta_value < value {
                    value = alphabeta_value;
                    beta = alphabeta_value;
                    best_tree[depth as usize - 1] = tile;
                }
            }

            let now = Instant::now();
            game.reset_game(tile, &old_alignment, &data.captured);
            if depth > 1 {
                game.neighbours = neighbours.to_owned();
            }
            unsafe {
                RESET_TIME += now.elapsed().as_nanos();
            }

            if alpha >= beta {
                break;
            }
        }
    }
    value
}
