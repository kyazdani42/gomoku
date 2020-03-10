use serde::{Deserialize, Serialize};

use super::game::Game;

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
}

impl State {
    pub fn new() -> State {
        State {
            ia: 0,
            time: 0,
            game: Game::new(0, 0),
            winner: 0,
        }
    }

    pub fn initialize(&mut self, board_size: u8, player: u8, ia: u8) {
        self.ia = ia;
        self.game.current_player = player;
        self.game.board_size = board_size;
    }

    pub fn run(&mut self, index: i32) {
        if self.game.player1.contains(index)
            || self.game.player2.contains(index)
            || self.game.forbidden.contains(&index)
            || !self.game.valid_index(index)
        {
            return;
        }

        self.game.place_stone(index);
        self.game.update_captures();
        self.winner = self.game.get_winner(index);

        self.game.switch_player(index);
        self.game.refresh_free_three();
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
            } else if self.game.forbidden.contains(&idx) {
                board.push(3);
            } else {
                board.push(0);
            }
        }

        board
    }
}
