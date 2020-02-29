pub enum Move {
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Move {
    pub fn can_move(self, board_size: i32, index: i32) -> bool {
        match self {
            Left => 0 < index % board_size,
            Right => index % board_size < board_size - 1,
            Top => board_size <= index,
            Bottom => index / board_size < board_size - 1,
            // TODO: there might be faster way to do those
            TopLeft => {
                Move::Top.can_move(board_size, index) && Move::Left.can_move(board_size, index)
            }
            TopRight => {
                Move::Top.can_move(board_size, index) && Move::Right.can_move(board_size, index)
            }
            BottomLeft => {
                Move::Bottom.can_move(board_size, index) && Move::Left.can_move(board_size, index)
            }
            BottomRight => {
                Move::Bottom.can_move(board_size, index) && Move::Right.can_move(board_size, index)
            }
        }
    }

    pub fn get_index(self, board_size: i32, index: i32) -> i32 {
        match self {
            Left => index - 1,
            Right => index + 1,
            Top => index - board_size,
            Bottom => index + board_size,
            TopLeft => index - (board_size + 1),
            TopRight => index - (board_size - 1),
            BottomLeft => index + (board_size - 1),
            BottomRight => index + (board_size + 1),
        }
    }
}
