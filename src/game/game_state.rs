use std::collections::HashMap;
use std::time::Instant;

use super::{
    capture_all, check_alignment_validity, place_stone, set_free_threes, switch_player,
    win_by_alignment, win_by_capture,
};
use crate::algorithm;

pub type Player = u8;
pub type Stones = HashMap<usize, Player>;

#[derive(Clone, Debug)]
pub struct GameState {
    pub player: Player,
    pub winner: Player,
    pub ia: Player,
    pub p1_captured: u8,
    pub p2_captured: u8,
    pub placed: Stones,
    pub board_size: usize,
    pub last_played: usize,
    pub time: u128,
    pub alignment: Option<Vec<usize>>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player: 0,
            winner: 0,
            ia: 0,
            p1_captured: 0,
            p2_captured: 0,
            board_size: 0,
            last_played: 0,
            placed: HashMap::new(),
            alignment: None,
            time: 0,
        }
    }

    pub fn init(&mut self, board_size: usize, player: u8, ia: u8) {
        *self = GameState {
            player,
            board_size,
            ia,
            winner: 0,
            p1_captured: 0,
            p2_captured: 0,
            last_played: 0,
            alignment: None,
            placed: HashMap::new(),
            time: 0,
        };
    }

    pub fn play(&mut self, index: usize) {
        let time = Instant::now();
        if self.winner != 0 {
            return;
        }

        if let None = place_stone(self, index) {
            return;
        }

        capture_all(self);

        if win_by_capture(self) {
            self.winner = self.player;
            return;
        }

        if let Some(alignment) = &self.alignment {
            if check_alignment_validity(&self.placed, alignment) {
                self.winner = switch_player(self.player);
            } else {
                self.alignment = None;
            }
        }

        if self.winner == 0 {
            win_by_alignment(self);
        }

        self.player = switch_player(self.player);

        set_free_threes(self);
        println!("{}ms", time.elapsed().as_millis());
    }

    pub fn play_ia(&mut self) {
        if self.winner != 0 || self.ia == 0 {
            return;
        }

        let time = Instant::now();
        let index = algorithm::compute(&self);
        self.time = time.elapsed().as_nanos();
        self.play(index);
    }
}
