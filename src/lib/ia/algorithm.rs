extern crate rand;
use rand::Rng;

use crate::lib::game::{Game, Tile};

use std::i32::{MAX, MIN};

static mut analyzer_time: u128 = 0;
static mut analyzer_num: i32 = 0;
static mut update_time: u128 = 0;
static mut reset_time: u128 = 0;
use std::time::Instant;

pub fn run(game: Game) -> Vec<Tile> {
    unsafe {
        analyzer_time = 0;
        analyzer_num = 0;
        update_time = 0;
        reset_time = 0;
    }

    let mut best_hits = vec![];
    let empty_neighbours = game.empty_neighbours.clone();

    let mut alpha = MAX;
    let mut beta = MIN;
    for tile in empty_neighbours {
        unsafe {
            analyzer_num += 1;
        }
        let now = Instant::now();
        let data = game.analyze(&tile);
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
            best_hits.push((tile, alphabeta(&mut game, 9, &mut alpha, &mut beta, false)));
        }
    }

    unsafe {
        println!("analyzed called {} times", analyzer_num);
        println!("analyzed lasted {}ms", analyzer_time / 1_000_000);
        println!("updates lasted {}ms", update_time / 1_000_000);
        println!("reset lasted {}ms", reset_time / 1_000_000);
    }

    best_hits.sort_by(|a, b| a.1.cmp(&b.1));
    best_hits.iter().map(|v| v.0).collect()
}

fn alphabeta(
    game: &mut Game,
    depth: i32,
    alpha: &mut i32,
    beta: &mut i32,
    maximizing_player: bool,
) -> i32 {
    if depth == 0 {
        // return 1;
        // return rand::thread_rng().gen();
        return game.get_opponent().captured as i32
    }

    let empty_neighbours = game.empty_neighbours.clone();
    let old_alignment = game.opponent_alignments.clone();
    if maximizing_player {
        let mut value = MIN;

        for tile in &empty_neighbours {
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
                return MAX - depth * 1000;
            } else if data.oponent_win {
                return MIN + depth * 1000;
            } else {
                let now = Instant::now();
                game.update_game(tile, &data.alignments, &data.captured);
                unsafe {
                    update_time += now.elapsed().as_nanos();
                }

                value = i32::max(
                    alphabeta(game, depth - 1, alpha, beta, !maximizing_player),
                    value,
                );

                let now = Instant::now();
                game.reset_game(tile, &old_alignment, &data.captured, &empty_neighbours);
                unsafe {
                    reset_time += now.elapsed().as_nanos();
                }

                *alpha = i32::max(*alpha, value);
                if *alpha >= *beta {
                    break;
                }
            }
        }
        value
    } else {
        let mut value = MAX;

        for tile in &empty_neighbours {
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
                return MIN + depth * 1000;
            } else if data.oponent_win {
                return MAX - depth * 1000;
            } else {
                let now = Instant::now();
                game.update_game(tile, &data.alignments, &data.captured);
                unsafe {
                    update_time += now.elapsed().as_nanos();
                }

                value = i32::min(
                    alphabeta(game, depth - 1, alpha, beta, !maximizing_player),
                    value,
                );

                let now = Instant::now();
                game.reset_game(tile, &old_alignment, &data.captured, &empty_neighbours);
                unsafe {
                    reset_time += now.elapsed().as_nanos();
                }

                *beta = i32::min(*beta, value);
                if *alpha >= *beta {
                    break;
                }
            }
        }
        value
    }
}
