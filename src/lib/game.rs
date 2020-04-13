use super::analyze::{analyze_index, AnalyzedTile};
use super::player::Player;
use super::r#move::Moves;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::sync::{Arc,Mutex};

pub type Tile = (i32, i32);

#[derive(Clone)]
pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub current_player: u8,
    pub opponent_player: u8,
    pub board_size: i32,
    pub board: Vec<Vec<u8>>,
    pub empty_neighbours: HashSet<Tile>,
    pub opponent_alignments: Vec<Vec<Tile>>,
    pub moves: Arc<Mutex<Moves>>,
}

impl Game {
    pub fn new(board_size: i32) -> Self {
        Self {
            current_player: 1,
            opponent_player: 2,
            player1: Player::new(),
            player2: Player::new(),
            empty_neighbours: HashSet::new(),
            opponent_alignments: vec![],
            moves: Arc::new(Mutex::new(Moves::new(board_size))),
            board: (0..board_size)
                .map(|_| (0..board_size).map(|_| 0).collect::<Vec<u8>>())
                .collect(),
            board_size,
        }
    }

    pub fn update_game(&mut self, tile: &Tile, alignments: &Vec<Vec<Tile>>, captured: &Vec<Tile>) {
        self.insert_tile(tile);
        self.update_opponent_alignments(alignments);
        self.update_captures(captured);
        self.update_empty_neighbours(tile);
        self.switch_player(tile);
    }

    pub fn reset_game(
        &mut self,
        tile: &Tile,
        alignments: &Vec<Vec<Tile>>,
        captured: &Vec<Tile>,
        empty_neighbours: &HashSet<Tile>,
    ) {
        self.reset_switch_player();
        self.remove_tile(tile);
        self.reset_captures(captured);
        self.opponent_alignments = alignments.to_owned();
        self.empty_neighbours = empty_neighbours.to_owned();
    }

    pub fn analyze(&self, tile: &Tile) -> AnalyzedTile {
        analyze_index(tile, self)
    }

    pub fn insert_tile(&mut self, tile: &Tile) {
        self.board[tile.0 as usize][tile.1 as usize] = self.current_player;
    }

    pub fn insert_forbidden(&mut self, tile: &Tile) {
        self.board[tile.0 as usize][tile.1 as usize] = 3;
    }

    pub fn remove_tile(&mut self, tile: &Tile) {
        self.board[tile.0 as usize][tile.1 as usize] = 0;
    }

    pub fn get_tile_value(&self, tile: &Tile) -> u8 {
        self.board[tile.0 as usize][tile.1 as usize]
    }

    pub fn validate_tile(&self, tile: &Tile) -> bool {
        -1 < tile.0 && tile.0 < self.board_size && -1 < tile.1 && tile.1 < self.board_size
    }

    pub fn update_captures(&mut self, captured: &Vec<Tile>) {
        self.get_player_mut().captured += captured.len() as u8;
        for tile in captured {
            self.remove_tile(tile);
        }
    }

    fn reset_captures(&mut self, captured: &Vec<Tile>) {
        self.get_player_mut().captured -= captured.len() as u8;
        for tile in captured {
            self.insert_tile(tile);
        }
    }

    pub fn update_empty_neighbours(&mut self, tile: &Tile) {
        for i in max(tile.0 - 2, 0)..=min(tile.0 + 2, self.board_size - 1) {
            for j in max(tile.1 - 2, 0)..=min(tile.1 + 2, self.board_size - 1) {
                let t = (i, j);
                if self.get_tile_value(&t) == 0 {
                    self.empty_neighbours.insert(t);
                }
            }
        }
        self.empty_neighbours.remove(&tile);
    }

    pub fn switch_player(&mut self, tile: &Tile) {
        self.get_player_mut().push_hit(*tile);
        let tmp_player = self.current_player;
        self.current_player = self.opponent_player;
        self.opponent_player = tmp_player;
    }

    fn reset_switch_player(&mut self) {
        let tmp_player = self.current_player;
        self.current_player = self.opponent_player;
        self.opponent_player = tmp_player;
        self.get_player_mut().remove_hit();
    }

    pub fn get_player(&self) -> &Player {
        if self.current_player == 1 {
            &self.player1
        } else {
            &self.player2
        }
    }

    pub fn get_opponent(&self) -> &Player {
        if self.current_player == 1 {
            &self.player2
        } else {
            &self.player1
        }
    }

    fn get_player_mut(&mut self) -> &mut Player {
        if self.current_player == 1 {
            &mut self.player1
        } else {
            &mut self.player2
        }
    }

    pub fn update_opponent_alignments(&mut self, alignments: &Vec<Vec<Tile>>) {
        self.opponent_alignments = alignments.to_owned();
    }
}
