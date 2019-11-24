use crate::game::board;

pub struct Stone(pub usize, pub usize);

pub struct GameState {
    pub board: Vec<Vec<u8>>,
    pub player: u8,
    pub winner: u8,
    pub player_one_captured: u8,
    pub player_two_captured: u8,
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
            player: 0,
        }
    }

    pub fn init(&mut self, board_size: usize, player: u8) {
        *self = GameState {
            board: vec![vec![0; board_size]; board_size],
            winner: 0,
            player_one_captured: 0,
            player_two_captured: 0,
            stone: Stone(0, 0),
            board_size,
            player,
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
    }

    pub fn place_stone(&mut self, line: usize, col: usize) -> Option<()> {
        if line >= self.board_size || col >= self.board_size || self.board[line][col] != 0 {
            None
        } else {
            self.board[line][col] = self.player;
            self.stone = Stone(line, col);
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
        (self.player == 1 && self.player_one_captured == 10)
            || (self.player == 2 && self.player_two_captured == 10)
            || [
                ["bot_left", "top_right"],
                ["left", "right"],
                ["top", "bot"],
                ["top_left", "bot_right"],
            ]
            .iter()
            .any(|actions| {
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
        [
            "bot_left",
            "top_right",
            "left",
            "right",
            "top",
            "bot",
            "top_left",
            "bot_right",
        ]
        .iter()
        .for_each(|action| self.capture(action, &other_player));
    }

    fn capture(&mut self, action: &str, other_player: &u8) {
        let stone_one: Stone = match board::move_stone(&self.stone, self.board_size, action) {
            Some(stone) => {
                if self.get_player(&stone) == *other_player {
                    stone
                } else {
                    return;
                }
            }
            None => return,
        };
        let stone_two: Stone = match board::move_stone(&stone_one, self.board_size, action) {
            Some(stone) => {
                if self.get_player(&stone) == *other_player {
                    stone
                } else {
                    return;
                }
            }
            None => return,
        };

        if let Some(stone) = board::move_stone(&stone_two, self.board_size, action) {
            if self.get_player(&stone) == self.player {
                self.board[stone_one.0][stone_one.1] = 0;
                self.board[stone_two.0][stone_two.1] = 0;
                if self.player == 1 {
                    self.player_one_captured += 2;
                } else {
                    self.player_two_captured += 2;
                }
            }
        };
    }

    pub fn get_player(&self, stone: &Stone) -> u8 {
        self.board[stone.0][stone.1]
    }
}
