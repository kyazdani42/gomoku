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
    level: u8,
    best_hits: Vec<Tile>,
    forbidden: Vec<Tile>,
}

impl State {
    pub fn new() -> State {
        State {
            ia: 0,
            level: 1,
            time: 0,
            winner: 0,
            game: Game::new(),
            best_hits: vec![(0, 0)],
            forbidden: vec![],
        }
    }

    pub fn initialize(&mut self, ia: u8, level: u8) {
        *self = State {
            ia,
            level,
            time: 0,
            winner: 0,
            game: Game::new(),
            best_hits: vec![(9, 9)],
            forbidden: vec![],
        };
    }

    pub fn run_ia(&mut self) {
        if self.should_run_ia() && !self.best_hits.is_empty() {
            self.run(self.best_hits[0]);
        }
    }

    pub fn run(&mut self, tile: Tile) {
        if !self.game.validate_tile(tile) || self.game.get_tile_value(tile) != 0 {
            return;
        }

        let index_data = self.game.analyze(tile);
        self.game.insert_tile(tile);
        self.game.update_opponent_alignments(&index_data.alignments);
        self.game.update_captures(&index_data.captured);
        self.game.get_player_mut().push_hit(tile);
        self.game.update_neighbours(tile);

        if index_data.win {
            self.winner = self.game.current_player;
            self.game.switch_player();
        } else if index_data.oponent_win {
            self.winner = self.game.opponent_player;
            self.game.switch_player();
        } else {
            self.game.switch_player();
            self.best_hits = ia::run(self.game.clone(), self.level);
        }

        self.update_forbidden();
    }

    fn should_run_ia(&self) -> bool {
        self.ia == self.game.current_player
    }

    fn reset_forbidden(&mut self) {
        self.forbidden.clear();
    }

    fn update_forbidden(&mut self) {
        self.reset_forbidden();
        let neighbours = self.game.neighbours.clone();
        for neighbour in neighbours {
            let data = self.game.analyze(neighbour);
            if data.double_free_three {
                self.forbidden.push(neighbour);
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
        let mut cloned_board = vec![];
        for line in &self.game.board {
            let mut line_values = vec![];
            for offset in 0..19 {
                line_values.push((line >> (offset as u64 * 2) & 0x3) as u8)
            }
            cloned_board.push(line_values);
        }
        for i in 0..3 {
            if i < self.best_hits.len() {
                let tile = self.best_hits[i];
                cloned_board[tile.0 as usize][tile.1 as usize] = 5 + i as u8;
            }
        }
        for tile in &self.forbidden {
            cloned_board[tile.0 as usize][tile.1 as usize] = 3;
        }
        cloned_board.concat()
    }
}
