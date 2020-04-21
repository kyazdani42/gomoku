use std::collections::HashSet;
use std::u64::MAX;

use super::analyze::{analyze_index, AnalyzedTile};
use super::create_board::{create_tiles_directions, create_tiles_neighbours};
use super::player::Player;
use super::r#move::Moves;

pub type Tile = (i32, i32);

#[derive(Clone)]
pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub current_player: u8,
    pub opponent_player: u8,
    pub board_size: i32,
    pub current_tiles: HashSet<Tile>,
    pub board: Vec<u64>,
    pub tiles_neighbours: Vec<Vec<Vec<Tile>>>,
    pub tiles_directions: Vec<Vec<Vec<Vec<Vec<Tile>>>>>,
    pub neighbours: HashSet<Tile>,
    pub opponent_alignments: Vec<Vec<Tile>>,
}

impl Game {
    pub fn new(board_size: i32) -> Self {
        let moves = Moves::new(board_size);
        Self {
            current_player: 1,
            opponent_player: 2,
            player1: Player::new(),
            player2: Player::new(),
            current_tiles: HashSet::new(),
            neighbours: HashSet::new(),
            opponent_alignments: vec![],
            tiles_directions: create_tiles_directions(board_size, &moves.straight_moves),
            tiles_neighbours: create_tiles_neighbours(board_size, &moves.all_moves),
            board: (0..board_size).map(|_| 0).collect(),
            board_size,
        }
    }

    pub fn update_game(&mut self, tile: Tile, alignments: &[Vec<Tile>], captured: &[Tile]) {
        self.insert_tile(tile);
        self.update_opponent_alignments(alignments);
        self.update_captures(captured);
        self.update_neighbours(tile);
        self.switch_player();
    }

    pub fn reset_game(
        &mut self,
        tile: Tile,
        alignments: &[Vec<Tile>],
        captured: &[Tile],
        neighbours: &HashSet<Tile>,
    ) {
        self.reset_switch_player();
        self.remove_tile(tile);
        self.reset_captures(captured);
        self.opponent_alignments = alignments.to_owned();
        self.neighbours = neighbours.to_owned();
    }

    pub fn analyze(&self, tile: Tile) -> AnalyzedTile {
        analyze_index(tile, self)
    }

    pub fn insert_tile(&mut self, tile: Tile) {
        self.board[tile.0 as usize] |= (self.current_player as u64) << (tile.1 * 2);
        self.current_tiles.insert(tile);
    }

    pub fn remove_tile(&mut self, tile: Tile) {
        self.board[tile.0 as usize] &= MAX - (0x3 << (tile.1 * 2));
        self.current_tiles.remove(&tile);
    }

    pub fn get_tile_value(&self, tile: Tile) -> u8 {
        ((self.board[tile.0 as usize] >> (tile.1 * 2)) & 0x3) as u8
    }

    pub fn validate_tile(&self, tile: Tile) -> bool {
        -1 < tile.0 && tile.0 < self.board_size && -1 < tile.1 && tile.1 < self.board_size
    }

    pub fn update_captures(&mut self, captured: &[Tile]) {
        self.get_player_mut().captured += captured.len() as u8;
        for tile in captured {
            self.remove_tile(*tile);
        }
    }

    fn reset_captures(&mut self, captured: &[Tile]) {
        self.get_player_mut().captured -= captured.len() as u8;
        for tile in captured {
            self.insert_tile(*tile);
        }
    }

    pub fn update_neighbours(&mut self, tile: Tile) {
        for tile in &self.tiles_neighbours[tile.0 as usize][tile.1 as usize] {
            self.neighbours.insert(*tile);
        }

        self.neighbours.remove(&tile);
    }

    pub fn switch_player(&mut self) {
        let tmp_player = self.current_player;
        self.current_player = self.opponent_player;
        self.opponent_player = tmp_player;
    }

    fn reset_switch_player(&mut self) {
        let tmp_player = self.current_player;
        self.current_player = self.opponent_player;
        self.opponent_player = tmp_player;
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

    pub fn get_player_mut(&mut self) -> &mut Player {
        if self.current_player == 1 {
            &mut self.player1
        } else {
            &mut self.player2
        }
    }

    pub fn update_opponent_alignments(&mut self, alignments: &[Vec<Tile>]) {
        self.opponent_alignments = alignments.to_owned();
    }
}
