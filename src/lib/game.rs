use super::player::Player;
use super::r#move::Move;
use std::collections::HashSet;

const MOVES: [Move; 8] = [
    Move::Left,
    Move::Right,
    Move::Top,
    Move::Bottom,
    Move::TopLeft,
    Move::TopRight,
    Move::BottomLeft,
    Move::BottomRight,
];

#[derive(Clone)]
pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub current_player: u8,
    pub board_size: i32,
    pub empty_neighbours: HashSet<i32>,
    pub oponent_alignments: Vec<Vec<i32>>,
}

impl Game {
    pub fn new(board_size: i32) -> Game {
        Game {
            player1: Player::new(),
            player2: Player::new(),
            empty_neighbours: HashSet::new(),
            oponent_alignments: vec![],
            current_player: 1,
            board_size,
        }
    }

    pub fn update_game(&mut self, index: i32, alignments: Vec<Vec<i32>>, captured: &Vec<i32>) {
        self.place_stone(index);
        self.update_oponent_alignments(alignments);
        self.update_captures(captured);
        self.update_empty_neighbours(index);
        self.switch_player(index);
    }

    pub fn reset_game(
        &mut self,
        index: i32,
        alignments: Vec<Vec<i32>>,
        captured: &Vec<i32>,
        empty_neighbours: &HashSet<i32>,
    ) {
        self.switch_player(index);
        self.remove_stone(index);
        self.update_oponent_alignments(alignments);
        self.reset_captures(captured);
        self.empty_neighbours = empty_neighbours.clone();
    }

    pub fn place_stone(&mut self, index: i32) {
        self.get_player_mut().insert(index);
    }

    pub fn remove_stone(&mut self, index: i32) {
        self.get_player_mut().remove(index);
    }

    pub fn update_captures(&mut self, captured: &Vec<i32>) {
        self.get_player_mut().captured += captured.len() as u8;
        self.get_opponent_mut().remove_mult(captured);
    }

    fn reset_captures(&mut self, captured: &Vec<i32>) {
        self.get_player_mut().captured -= captured.len() as u8;
        self.get_opponent_mut().insert_mult(captured);
    }

    fn is_empty_neighbour(&self, index: i32) -> bool {
        !self.empty_neighbours.contains(&index)
            && !self.get_player().contains(index)
            && !self.get_opponent().contains(index)
    }

    pub fn update_empty_neighbours(&mut self, index: i32) {
        for direction in MOVES.iter() {
            if direction.can_move_to(self.board_size, index, 1) {
                let neighbour = direction.get_next_index(self.board_size, index);
                if self.is_empty_neighbour(neighbour) {
                    self.empty_neighbours.insert(neighbour);
                }
                if direction.can_move_to(self.board_size, index, 2) {
                    let neighbour = direction.get_index_mult(self.board_size, index, 2);
                    if self.is_empty_neighbour(neighbour) {
                        self.empty_neighbours.insert(neighbour);
                    }
                }
            }
        }

        self.empty_neighbours.remove(&index);
    }

    pub fn get_total_size(&self) -> i32 {
        self.board_size * self.board_size
    }

    pub fn valid_index(&self, index: i32) -> bool {
        -1 < index && index < self.get_total_size()
    }

    pub fn switch_player(&mut self, index: i32) {
        // add hit history instead of last_played
        self.get_player_mut().last_played = index;
        self.current_player = if self.current_player == 1 { 2 } else { 1 };
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

    pub fn get_opponent_mut(&mut self) -> &mut Player {
        if self.current_player == 1 {
            &mut self.player2
        } else {
            &mut self.player1
        }
    }

    pub fn get_player_mut(&mut self) -> &mut Player {
        if self.current_player == 1 {
            &mut self.player1
        } else {
            &mut self.player2
        }
    }

    pub fn update_oponent_alignments(&mut self, alignments: Vec<Vec<i32>>) {
        self.oponent_alignments = alignments;
    }
}
