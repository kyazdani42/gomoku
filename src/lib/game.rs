use super::player::Player;

#[derive(Clone)]
pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub current_player: u8,
    pub board_size: u8,
    pub forbidden: Vec<i32>,
}

impl Game {
    pub fn new(current_player: u8, board_size: u8) -> Game {
        Game {
            player1: Player::new(),
            player2: Player::new(),
            forbidden: vec![],
            current_player,
            board_size,
        }
    }

    pub fn place_stone(&mut self, index: i32) {
        self.get_player_mut().tiles.push(index)
    }

    pub fn update_captures(&mut self) {}

    pub fn get_winner(&self, index: i32) -> u8 {
        if 9 < self.get_player().captured {
            self.current_player
        // TODO: can optimize this by storing in Player
        } else if self.get_opponent().get_alignments(index).is_some() {
            if self.current_player == 1 {
                2
            } else {
                1
            }
        } else {
            let alignments = self.get_player().get_alignments(index);
            // if self.get_player().can_alignments_be_captured() {}
            if alignments.is_none() {
                0
            } else {
                self.current_player
            }
        }
    }

    pub fn refresh_free_three(&mut self) {}

    pub fn get_total_size(&self) -> i32 {
        self.board_size as i32 * self.board_size as i32
    }

    pub fn valid_index(&self, index: i32) -> bool {
        -1 < index && index < self.get_total_size()
    }

    pub fn switch_player(&mut self, index: i32) {
        self.get_player_mut().last_played = index;
        self.current_player = if self.current_player == 1 { 2 } else { 1 };
    }

    fn get_player(&self) -> &Player {
        if self.current_player == 1 {
            &self.player1
        } else {
            &self.player2
        }
    }

    fn get_opponent(&self) -> &Player {
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
}
