use crate::game::board;

pub struct Stone(pub usize, pub usize);

pub struct GameState {
    pub board: Vec<Vec<u8>>,
    pub player: u8,
    pub winner: u8,
    board_size: usize,
    stone: Stone,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            board: vec![],
            winner: 0,
            stone: Stone(0, 0),
            board_size: 0,
            player: 0,
        }
    }

    pub fn init(&mut self, board_size: usize, player: u8) {
        self.board = vec![vec![0; board_size]; board_size];
        self.board_size = board_size;
        self.player = player;
    }

    pub fn place_stone(&mut self, line: usize, col: usize) -> Option<()> {
        if line < 0
            || line >= self.board_size
            || col < 0
            || col >= self.board_size
            || self.board[line][col] != 0
        {
            None
        } else {
            self.board[line][col] = self.player;
            self.stone = Stone(line, col);
            if self.check_winner() == true {
                self.winner = self.player;
            }
            self.switch_player();
            Some(())
        }
    }

    fn switch_player(&mut self) {
        self.player = if self.player == 1 { 2 } else { 1 };
    }

    pub fn check_winner(&self) -> bool {
        [
            ("bot_left", "top_right"),
            ("left", "right"),
            ("top", "bot"),
            ("top_left", "bot_right"),
        ]
        .iter()
        .any(|actions| {
            board::check_alignment(
                &self.board,
                &self.stone,
                self.player,
                self.board_size,
                actions.0,
                actions.1,
            ) == true
        })
    }

    //    pub fn captures_all(&self) -> {
    //        let mut i = 0;
    //        let mut captures: i32 = 0;
    //        self.capture();
    //        let t: Stone = (5, 4);;
    //
    //        return captures;
    //    }
    //
    //    pub fn capture(&self) -> i32 {
    //        let other_player = switch_player(player);
    //        let capture_one: Stone = f(index as i32);
    //        let capture_two: Stone = f(capture_one);
    //        let capture_final: Stone = f(capture_two);
    //        if capture_final == player {
    //            if capture_one == other_player && capture_two == other_player {
    //                return (capture_one, capture_two);
    //            }
    //        }
    //
    //        return (-1, -1);
    //    }
}
