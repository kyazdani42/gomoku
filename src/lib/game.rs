use std::collections::HashSet;

use super::analyze::{analyze_index, AnalyzedTile};
use super::create_board::{create_tiles_directions, create_tiles_neighbours};
use super::player::Player;
use super::r#move::Moves;

pub type Tile = isize;

#[derive(Clone)]
pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub current_player: u8,
    pub opponent_player: u8,
    pub current_tiles: Vec<Tile>,
    pub board: Vec<u8>,
    pub tiles_neighbours: Vec<Vec<Tile>>,
    pub tiles_directions: Vec<Vec<Vec<Vec<Tile>>>>,
    pub neighbours: HashSet<Tile>,
    pub opponent_alignments: Vec<Vec<Tile>>,
}

impl Game {
    pub fn new() -> Self {
        let moves = Moves::new();
        Self {
            current_player: 1,
            opponent_player: 2,
            player1: Player::new(),
            player2: Player::new(),
            current_tiles: vec![],
            neighbours: HashSet::new(),
            opponent_alignments: vec![],
            tiles_directions: create_tiles_directions(&moves.straight_moves),
            tiles_neighbours: create_tiles_neighbours(&moves.all_moves),
            board: (0..(19*19)).map(|_| 0).collect(),
        }
    }

    pub fn update_game(&mut self, tile: Tile, alignments: &[Vec<Tile>], captured: &[Tile]) {
        self.insert_tile(tile);
        self.update_opponent_alignments(alignments);
        self.update_captures(captured);
        self.update_neighbours(tile);
        self.switch_player();
    }

    pub fn reset_game(&mut self, tile: Tile, alignments: &[Vec<Tile>], captured: &[Tile]) {
        self.reset_switch_player();
        self.remove_tile(tile);
        self.reset_captures(captured);
        self.opponent_alignments = alignments.to_owned();
    }

    pub fn analyze(&self, tile: Tile) -> AnalyzedTile {
        analyze_index(tile, self)
    }

    pub fn insert_tile(&mut self, tile: Tile) {
        unsafe {
            *self.board.as_mut_ptr().offset(tile as isize) = self.current_player;
        }
        self.current_tiles.push(tile);
    }

    pub fn remove_tile(&mut self, tile: Tile) {
        unsafe {
            *self.board.as_mut_ptr().offset(tile) = 0;
        }
        self.current_tiles.retain(|x| *x != tile);
    }

    pub fn get_tile_value(&self, tile: Tile) -> u8 {
        unsafe {
            *self.board.as_ptr().offset(tile)
        }
    }

    pub fn validate_tile(&self, tile: Tile) -> bool {
        -1 < tile && tile < 19 * 19
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
        for tile in &self.tiles_neighbours[tile as usize] {
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
