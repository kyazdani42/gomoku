// extern crate rand;
// use rand::{Rng,thread_rng};

use super::heuristic::heuristic;
use crate::lib::game::{Game, Tile};

use std::i32::{MAX, MIN};

static mut analyzer_time: u128 = 0;
static mut analyzer_num: i32 = 0;
static mut update_time: u128 = 0;
static mut reset_time: u128 = 0;
static mut heuristic_time: u128 = 0;
static mut heuristic_num: i32 = 0;
use std::time::Instant;

pub fn run(game: Game) -> Vec<Tile> {
    unsafe {
        analyzer_time = 0;
        analyzer_num = 0;
        update_time = 0;
        reset_time = 0;
        heuristic_time = 0;
        heuristic_num = 0;
    }

    let mut best_hits = vec![];
    let empty_neighbours = game.empty_neighbours.clone();

    let depth = 2;
    let mut alpha = MIN;
    for tile in empty_neighbours {
        unsafe {
            analyzer_num += 1;
        }
        let now = Instant::now();
        let data = game.analyze(tile);
        unsafe {
            analyzer_time += now.elapsed().as_nanos();
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
            let value = alphabeta(&mut game, depth - 1, alpha, MAX, false);
            if value > alpha {
                alpha = value;
            }
            best_hits.push((tile, value));
        }
    }

    unsafe {
        println!("analyzed called {} times", analyzer_num);
        println!("analyzed lasted {}ms", analyzer_time / 1_000_000);
        println!("heuristic called {} times", heuristic_num);
        println!("heuristic lasted {}ms", heuristic_time / 1_000_000);
        println!("updates lasted {}ms", update_time / 1_000_000);
        println!("reset lasted {}ms\n", reset_time / 1_000_000);
    }

    best_hits.sort_by(|a, b| b.1.cmp(&a.1));
    for v in &best_hits {
        println!("{} {}, h: {}", (v.0).0, (v.0).1, v.1);
    }
    best_hits.iter().map(|v| v.0).collect()
}

fn alphabeta(
    game: &mut Game,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: bool,
) -> i32 {
    if depth == 0 {
        let now = Instant::now();
        let h = heuristic(game, maximizing_player);
        // let h = thread_rng().gen();
        unsafe {
            heuristic_time += now.elapsed().as_nanos();
            heuristic_num += 1;
        }
        return h;
    }

    let empty_neighbours = game.empty_neighbours.clone();
    let old_alignment = game.opponent_alignments.clone();
    let mut value = if maximizing_player { MIN } else { MAX };

    for tile in &empty_neighbours {
        let tile = *tile;
        let now = Instant::now();
        let data = game.analyze(tile);
        unsafe {
            analyzer_time += now.elapsed().as_nanos();
            analyzer_num += 1;
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
            game.update_game(tile, &data.alignments, &data.captured);
            unsafe {
                update_time += now.elapsed().as_nanos();
            }
            if maximizing_player {
                value = i32::max(
                    alphabeta(game, depth - 1, alpha, beta, false),
                    value,
                );
                let now = Instant::now();
                game.reset_game(tile, &old_alignment, &data.captured, &empty_neighbours);
                unsafe {
                    reset_time += now.elapsed().as_nanos();
                }

                alpha = i32::max(alpha, value);
            } else {
                value = i32::min(alphabeta(game, depth - 1, alpha, beta, true), value);
                let now = Instant::now();
                game.reset_game(tile, &old_alignment, &data.captured, &empty_neighbours);
                unsafe {
                    reset_time += now.elapsed().as_nanos();
                }
                beta = i32::min(beta, value);
            }
            if alpha >= beta {
                break;
            }
        }
    }
    value
}
