use std::time::Instant;

use super::{board, ACTIONS, JOINED_ACTIONS};
use crate::algorithm;

pub struct Stone(pub usize, pub usize);
pub type Board = Vec<Vec<u8>>;

pub struct GameState {
    pub board: Board,
    pub player: u8,
    pub winner: u8,
    pub player_one_captured: u8,
    pub player_two_captured: u8,
    pub ia: u8,
    pub time: u128,
    board_size: usize,
    stone: Stone,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            board: vec![],
            winner: 0,
            player_one_captured: 0,
            player_two_captured: 0,
            stone: Stone(0, 0),
            board_size: 0,
            time: 0,
            player: 0,
            ia: 0,
        }
    }

    pub fn init(&mut self, board_size: usize, player: u8, ia: u8) {
        *self = GameState {
            board: vec![vec![0; board_size]; board_size],
            winner: 0,
            player_one_captured: 0,
            player_two_captured: 0,
            stone: Stone(0, 0),
            board_size,
            time: 0,
            player,
            ia,
        };
    }

    pub fn play(&mut self, line: usize, col: usize) {
        if self.winner != 0 {
            return;
        }

        if let None = self.place_stone(line, col) {
            return;
        }

        self.capture_all();

        if self.check_winner() {
            self.winner = self.player;
        }

        self.player = self.switch_player();

        self.set_free_threes();
    }

    pub fn play_ia(&mut self) {
        if self.winner != 0 || self.ia == 0 {
            return;
        }

        let time = Instant::now();
        let (line, col) = algorithm::compute(&self.board, &self.player, self.board_size);
        self.time = time.elapsed().as_nanos();
        self.play(line, col);
    }

    fn set_free_threes(&mut self) {
        // we need to do this because we mutate the board later on
        // and we cannot mutate the board when it's borrowed as immutable
        let board = self.board.clone();
        for (i_line, line) in board.iter().enumerate() {
            for (i_col, value) in line.iter().enumerate() {
                // if its empty or a free three
                if *value == 0 || *value == 3 {
                    let stone = Stone(i_line, i_col);
                    let is_double_free_three = board::check_double_free_threes(
                        &self.board,
                        &stone,
                        self.player,
                        self.board_size,
                    );

                    // TODO: if the player can capture, do not set the value
                    if is_double_free_three {
                        self.set_value(&stone, 3);
                    } else if *value == 3 {
                        self.set_value(&stone, 0);
                    }
                }
            }
        }
    }

    pub fn place_stone(&mut self, line: usize, col: usize) -> Option<()> {
        let stone = Stone(line, col);
        if line >= self.board_size || col >= self.board_size || self.get_value(&stone) != 0 {
            None
        } else {
            self.set_value(&stone, self.player);
            self.stone = stone;
            Some(())
        }
    }

    fn switch_player(&self) -> u8 {
        if self.player == 1 {
            2
        } else {
            1
        }
    }

    pub fn check_winner(&self) -> bool {
        self.win_by_capture() || self.win_by_alignment()
    }

    fn win_by_capture(&self) -> bool {
        if self.player == 1 {
            self.player_one_captured == 10
        } else {
            self.player_two_captured == 10
        }
    }

    fn win_by_alignment(&self) -> bool {
        JOINED_ACTIONS.iter().any(|actions| {
            board::check_alignment(
                &self.board,
                &self.stone,
                self.player,
                self.board_size,
                *actions,
            ) == true
        })
    }

    pub fn capture_all(&mut self) {
        let other_player = self.switch_player();
        ACTIONS
            .iter()
            .for_each(|action| self.capture(action, &other_player));
    }

    fn capture(&mut self, action: &str, other_player: &u8) {
        let stone_one: Stone = match board::move_stone(&self.stone, self.board_size, action) {
            Some(stone) if self.get_value(&stone) == *other_player => stone,
            _ => return,
        };
        let stone_two: Stone = match board::move_stone(&stone_one, self.board_size, action) {
            Some(stone) if self.get_value(&stone) == *other_player => stone,
            _ => return,
        };

        if let Some(stone) = board::move_stone(&stone_two, self.board_size, action) {
            if self.get_value(&stone) == self.player {
                self.set_value(&stone_one, 0);
                self.set_value(&stone_two, 0);
                if self.player == 1 {
                    self.player_one_captured += 2;
                } else {
                    self.player_two_captured += 2;
                }
            }
        };
    }

    fn set_value(&mut self, stone: &Stone, player: u8) {
        self.board[stone.0][stone.1] = player;
    }

    fn get_value(&self, stone: &Stone) -> u8 {
        self.board[stone.0][stone.1]
    }
}
