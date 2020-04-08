extern crate rand;
use rand::Rng;

use std::i32::{MAX, MIN};

use crate::lib::analyze::analyze_index;
use crate::lib::game::Game;

pub fn run(game: &Game) -> Vec<i32> {
    let mut best_hits = vec![];

    for tile in &game.empty_neighbours {
        let index = *tile;
        let data = analyze_index(
            index,
            game.board_size,
            &game.get_player(),
            &game.get_opponent(),
            &game.oponent_alignments,
        );

        if data.double_free_three {
            continue;
        }

        if data.win {
            best_hits.push((tile, MAX));
        } else if data.oponent_win {
            best_hits.push((tile, MIN));
        } else {
            let mut cloned_game = game.clone();
            cloned_game.place_stone(index);
            cloned_game.update_oponent_alignments(&data.alignments);
            cloned_game.update_captures(&data.captured);
            cloned_game.update_empty_neighbours(index);
            cloned_game.switch_player(index);
            // best_hits.push((tile, minimax(&cloned_game, 2, false)));
            best_hits.push((tile, alphabeta(&cloned_game, 2, MIN, MAX, false)));
        }
    }

    best_hits.sort_by(|a, b| a.1.cmp(&b.1));
    best_hits.iter().map(|v| *v.0).collect()
}

fn minimax(game: &Game, depth: u8, maximizing_player: bool) -> i32 {
    if depth == 0 {
        return game.get_opponent().captured as i32;
    }

    let mut value = if maximizing_player { MIN } else { MAX };
    let cmp_func = if maximizing_player {
        i32::max
    } else {
        i32::min
    };

    for tile in &game.empty_neighbours {
        let index = *tile;
        let data = analyze_index(
            index,
            game.board_size,
            &game.get_player(),
            &game.get_opponent(),
            &game.oponent_alignments,
        );

        if data.double_free_three {
            continue;
        }

        if data.win {
            return MAX - depth as i32;
        } else if data.oponent_win {
            return MIN + depth as i32;
        } else {
            let mut cloned_game = game.clone();
            cloned_game.place_stone(index);
            cloned_game.update_oponent_alignments(&data.alignments);
            cloned_game.update_captures(&data.captured);
            cloned_game.update_empty_neighbours(index);
            cloned_game.switch_player(index);
            value = cmp_func(minimax(&cloned_game, depth - 1, !maximizing_player), value);
        }
    }

    value
}

fn alphabeta(game: &Game, depth: u8, alpha: i32, beta: i32, maximizing_player: bool) -> i32 {
    if depth == 0 {
        return rand::thread_rng().gen();
        // return game.get_opponent().captured as i32;
    }

    if maximizing_player {
        let mut value = MIN;
        let mut alpha = alpha;

        for tile in &game.empty_neighbours {
            let index = *tile;
            let data = analyze_index(
                index,
                game.board_size,
                &game.get_player(),
                &game.get_opponent(),
                &game.oponent_alignments,
            );

            if data.double_free_three {
                continue;
            }

            if data.win {
                return MAX - depth as i32 * 1000;
            } else if data.oponent_win {
                return MIN + depth as i32 * 1000;
            } else {
                let mut cloned_game = game.clone();
                cloned_game.place_stone(index);
                cloned_game.update_oponent_alignments(&data.alignments);
                cloned_game.update_captures(&data.captured);
                cloned_game.update_empty_neighbours(index);
                cloned_game.switch_player(index);
                value = i32::max(
                    alphabeta(&cloned_game, depth - 1, alpha, beta, !maximizing_player),
                    value,
                );
                alpha = i32::max(alpha, value);
                if alpha >= beta {
                    break;
                }
            }
        }
        value
    } else {
        let mut value = MAX;
        let mut beta = beta;

        for tile in &game.empty_neighbours {
            let index = *tile;
            let data = analyze_index(
                index,
                game.board_size,
                &game.get_player(),
                &game.get_opponent(),
                &game.oponent_alignments,
            );

            if data.double_free_three {
                continue;
            }

            if data.win {
                return MAX - depth as i32 * 1000;
            } else if data.oponent_win {
                return MIN + depth as i32 * 1000;
            } else {
                let mut cloned_game = game.clone();
                cloned_game.place_stone(index);
                cloned_game.update_oponent_alignments(&data.alignments);
                cloned_game.update_captures(&data.captured);
                cloned_game.update_empty_neighbours(index);
                cloned_game.switch_player(index);
                value = i32::min(
                    alphabeta(&cloned_game, depth - 1, alpha, beta, !maximizing_player),
                    value,
                );
                beta = i32::min(beta, value);
                if alpha >= beta {
                    break;
                }
            }
        }
        value
    }
}
