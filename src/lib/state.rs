use serde::{Deserialize, Serialize};

use super::analyze::analyze_index;
use super::game::Game;
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
    best_index: i32,
    ia: u8,
    time: u128,
    game: Game,
    winner: u8,
    forbidden: Vec<i32>,
}

impl State {
    pub fn new() -> State {
        State {
            ia: 0,
            time: 0,
            game: Game::new(19),
            winner: 0,
            forbidden: vec![],
            best_index: 0,
        }
    }

    pub fn initialize(&mut self, board_size: u8, ia: u8) {
        *self = State {
            ia,
            time: 0,
            game: Game::new(board_size as i32),
            winner: 0,
            forbidden: vec![],
            best_index: board_size as i32 * board_size as i32 / 2,
        };
    }

    pub fn run_ia(&mut self) {
        if self.should_run_ia() {
            self.run(self.best_index);
        }
    }

    pub fn run(&mut self, index: i32) {
        if self.game.player1.contains(index)
            || self.game.player2.contains(index)
            || self.forbidden.contains(&index)
            || !self.game.valid_index(index)
        {
            return;
        }

        let index_data = analyze_index(
            index,
            self.game.board_size,
            &self.game.get_player(),
            &self.game.get_opponent(),
            &self.game.oponent_alignments,
        );

        self.game.place_stone(index);
        self.game.update_oponent_alignments(&index_data.alignments);
        self.game.update_captures(&index_data.captured);
        self.game.update_empty_neighbours(index);

        if index_data.win {
            self.winner = if self.game.current_player == 1 { 1 } else { 2 };
        } else if index_data.oponent_win {
            self.winner = if self.game.current_player == 1 { 2 } else { 1 };
        } else {
            self.game.switch_player(index);

            let mut best_indexes = ia::run(&self.game);
            self.best_index = best_indexes.pop().unwrap();

            self.update_forbidden();
        }
    }

    fn should_run_ia(&self) -> bool {
        self.ia == self.game.current_player
    }

    fn update_forbidden(&mut self) {
        self.forbidden.clear();
        for neighbour in &self.game.empty_neighbours {
            let data = analyze_index(
                *neighbour,
                self.game.board_size,
                &self.game.get_player(),
                &self.game.get_opponent(),
                &vec![],
            );
            if data.double_free_three {
                self.forbidden.push(*neighbour);
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
        let mut board = vec![];

        for idx in 0..self.game.get_total_size() {
            if self.game.player1.contains(idx) {
                board.push(1);
            } else if self.game.player2.contains(idx) {
                board.push(2);
            } else if self.forbidden.contains(&idx) {
                board.push(3);
            } else {
                board.push(0);
            }
        }

        board
    }
}
