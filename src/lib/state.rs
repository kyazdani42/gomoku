use serde::{Deserialize, Serialize};

use super::game::{Game, Tile};
use super::ia;

#[derive(Serialize, Deserialize)]
pub struct ResponseData {
    board: Vec<u8>,
    player: u8,
    p1_captured: u8,
    p2_captured: u8,
    winner: u8,
    ia: u8,
    time: u128,
}

pub struct State {
    ia: u8,
    time: u128,
    game: Game,
    winner: u8,
    best_hits: Vec<Tile>,
}

impl State {
    pub fn new() -> State {
        State {
            ia: 0,
            time: 0,
            game: Game::new(19),
            winner: 0,
            best_hits: vec![(0, 0)],
        }
    }

    pub fn initialize(&mut self, board_size: u8, ia: u8) {
        *self = State {
            ia,
            time: 0,
            game: Game::new(board_size as i32),
            winner: 0,
            best_hits: vec![(board_size as i32 / 2, board_size as i32 / 2)],
        };
    }

    pub fn run_ia(&mut self) {
        if self.should_run_ia() && self.best_hits.len() > 0 {
            self.run(self.best_hits[self.best_hits.len() - 1]);
        }
    }

    pub fn get_board_size(&self) -> i32 {
        self.game.board_size
    }

    pub fn run(&mut self, tile: Tile) {
        let tile_ref = &tile;
        if !self.game.validate_tile(tile_ref) || self.game.get_tile_value(tile_ref) != 0 {
            return;
        }

        let index_data = self.game.analyze(&tile);
        self.game.insert_tile(tile_ref);
        self.game.update_opponent_alignments(index_data.alignments);
        self.game.update_captures(&index_data.captured);
        self.game.update_empty_neighbours(tile_ref);

        if index_data.win {
            self.winner = self.game.current_player
        } else if index_data.oponent_win {
            self.winner = self.game.opponent_player
        } else {
            self.game.switch_player(tile_ref);
            self.best_hits = ia::run(&mut self.game);
            self.update_forbidden();
        }
    }

    fn should_run_ia(&self) -> bool {
        self.ia == self.game.current_player
    }

    fn reset_forbidden(&mut self) {
        for line in 0..self.game.board_size {
            for col in 0..self.game.board_size {
                if self.game.get_tile_value(&(line, col)) == 3 {
                    self.game.remove_tile(&(line, col));
                }
            }
        }
    }

    fn update_forbidden(&mut self) {
        self.reset_forbidden();
        let empty_neighbours = self.game.empty_neighbours.clone();
        for neighbour in empty_neighbours {
            let data = self.game.analyze(&neighbour);
            if data.double_free_three {
                self.game.insert_forbidden(&neighbour);
            }
        }
    }

    pub fn get_data(&self) -> ResponseData {
        ResponseData {
            board: self.get_board(),
            player: self.game.current_player,
            p1_captured: self.game.player1.captured,
            p2_captured: self.game.player2.captured,
            winner: self.winner,
            ia: self.ia,
            time: self.time,
        }
    }

    fn get_board(&self) -> Vec<u8> {
        let mut cloned_board = self.game.board.clone();
        let len = self.best_hits.len() as i32 - 1;
        for i in 0..3 {
            if len - i >= 0 {
                let tile = self.best_hits[(len - i) as usize];
                cloned_board[tile.0 as usize][tile.1 as usize] = 5;
            }
        }
        cloned_board.concat()
    }
}
