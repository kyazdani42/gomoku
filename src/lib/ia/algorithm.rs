use std::cell::RefCell;
extern crate rand;
// use rand::Rng;

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

    let alpha = RefCell::new(MIN);
    let beta = RefCell::new(MAX);
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
        } else if data.oponent_win {
            best_hits.push((tile, MIN));
        } else {
            let mut game = game.clone();
            game.update_game(tile, &data.alignments, &data.captured);
            best_hits.push((tile, alphabeta(&mut game, 9, &alpha, &beta, false)));
        }
    }

    unsafe {
        // println!("analyzed called {} times", analyzer_num);
        // println!("analyzed lasted {}ms", analyzer_time / 1_000_000);
        // println!("updates lasted {}ms", update_time / 1_000_000);
        // println!("reset lasted {}ms", reset_time / 1_000_000);
        // println!("heuristic called {} times", heuristic_num);
        // println!("heuristic lasted {}ms", heuristic_time / 1_000_000);
    }

    best_hits.sort_by(|a, b| a.1.cmp(&b.1));
    // for v in &best_hits {
    //     println!("{} {}, h: {}", (v.0).0, (v.0).1, v.1);
    // }
    best_hits.iter().map(|v| v.0).collect()
}

fn alphabeta(
    game: &mut Game,
    depth: i32,
    alpha: &RefCell<i32>,
    beta: &RefCell<i32>,
    maximizing_player: bool,
) -> i32 {
    if depth == 0 {
        let now = Instant::now();
        let heuristic = -heuristic(game);
        unsafe {
            heuristic_time += now.elapsed().as_nanos();
            heuristic_num += 1;
        }
        return heuristic;
        // return rand::thread_rng().gen();
    }

    let empty_neighbours = game.empty_neighbours.clone();
    let old_alignment = game.opponent_alignments.clone();
    let (mut value, alpha_value) = if maximizing_player {
        (MIN, alpha)
    } else {
        (MAX, beta)
    };

    let cmp_fn = if maximizing_player {
        i32::max
    } else {
        i32::min
    };

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
                MAX - depth * 1000
            } else {
                MIN + depth * 1000
            };
        } else if data.oponent_win {
            return if maximizing_player {
                MIN + depth * 1000
            } else {
                MAX - depth * 1000
            };
        } else {
            let now = Instant::now();
            game.update_game(tile, &data.alignments, &data.captured);
            unsafe {
                update_time += now.elapsed().as_nanos();
            }
            value = cmp_fn(
                alphabeta(game, depth - 1, alpha, beta, !maximizing_player),
                value,
            );
            let now = Instant::now();
            game.reset_game(tile, &old_alignment, &data.captured, &empty_neighbours);
            unsafe {
                reset_time += now.elapsed().as_nanos();
            }

            let v = *alpha_value.borrow();
            alpha_value.replace(cmp_fn(v, value));
            if *alpha >= *beta {
                break;
            }
        }
    }
    value
}
