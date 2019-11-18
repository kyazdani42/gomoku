use crate::board;

pub struct GameState {
    board: Vec<Vec<u8>>,
    board_size: usize,
    line: usize,
    col: usize,
    player: u8,
    winner: u8,
}

impl GameState {
    pub fn new(board_size: usize, player: u8) -> GameState {
        GameState {
            board: vec![vec![0; board_size]; board_size],
            winner: 0,
            line: 0,
            col: 0,
            board_size,
            player,
        }
    }

    pub fn place_stone(&mut self, line: usize, col: usize) -> Result<(), ()> {
        if self.board[line][col] != 0 {
            Err(())
        } else {
            self.board[line][col] = self.player;
            self.line = line;
            self.col = col;
            if self.check_winner == true {
                self.winner = self.player;
            }
            self.switch_player();
            Ok(())
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
