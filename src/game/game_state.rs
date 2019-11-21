use crate::game::board;

pub struct GameState {
    pub board: Vec<Vec<u8>>,
    pub player: u8,
    pub winner: u8,
    board_size: usize,
    line: usize,
    col: usize,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            board: vec![],
            winner: 0,
            line: 0,
            col: 0,
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
        if self.board[line][col] != 0 {
            None
        } else {
            self.board[line][col] = self.player;
            self.line = line;
            self.col = col;
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
            board::check_horizontal_alignment,
            board::check_vertical_alignment,
            board::check_diagonal_left_alignment,
            board::check_diagonal_right_alignment,
        ]
        .iter()
        .any(|f| {
            f(
                &self.board,
                self.line,
                self.col,
                self.player,
                self.board_size,
            ) == true
        })
    }
}
