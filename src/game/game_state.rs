use std::time::Instant;

use super::{capture_all, check_winner, place_stone, set_free_threes, switch_player};
use crate::algorithm;

#[derive(Clone, Debug)]
pub struct Stone(pub usize, pub usize);

pub type Board = Vec<Vec<u8>>;

#[derive(Clone, Debug)]
pub struct GameState {
    pub board: Board,
    pub player: u8,
    pub winner: u8,
    pub player_one_captured: u8,
    pub player_two_captured: u8,
    pub ia: u8,
    pub time: u128,
    pub stone: Stone,
    pub init: bool,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            board: vec![],
            winner: 0,
            player_one_captured: 0,
            player_two_captured: 0,
            stone: Stone(0, 0),
            time: 0,
            player: 0,
            ia: 0,
            init: true,
        }
    }

    pub fn init(&mut self, board_size: usize, player: u8, ia: u8) {
        *self = GameState {
            board: vec![vec![0; board_size]; board_size],
            winner: 0,
            player_one_captured: 0,
            player_two_captured: 0,
            stone: Stone(0, 0),
            time: 0,
            init: true,
            player,
            ia,
        };
    }

    pub fn play(&mut self, line: usize, col: usize) {
        if self.winner != 0 {
            return;
        }

        if let None = place_stone(self, line, col) {
            return;
        }

        self.init = false;
        capture_all(self);

        if check_winner(self) {
            self.winner = self.player;
        }

        self.player = switch_player(self.player);

        set_free_threes(self);
    }

    pub fn play_ia(&mut self) {
        if self.winner != 0 || self.ia == 0 {
            return;
        }

        self.init = false;
        let time = Instant::now();
        let (line, col) = algorithm::compute(&self);
        self.time = time.elapsed().as_nanos();
        self.play(line, col);
    }
}
