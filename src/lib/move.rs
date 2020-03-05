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
            Move::Left => 0 < index % board_size,
            Move::Right => index % board_size < board_size - 1,
            Move::Top => board_size <= index,
            Move::Bottom => index / board_size < board_size - 1,
            Move::TopLeft => board_size <= index && 0 < index % board_size,
            Move::TopRight => board_size <= index && index % board_size < board_size - 1,
            Move::BottomLeft => index / board_size < board_size - 1 && 0 < index % board_size,
            Move::BottomRight => {
                let edge = board_size - 1;
                index / board_size < edge && index % board_size < edge
            }
        }
    }

    pub fn get_index(self, board_size: i32, index: i32) -> i32 {
        match self {
            Move::Left => index - 1,
            Move::Right => index + 1,
            Move::Top => index - board_size,
            Move::Bottom => index + board_size,
            Move::TopLeft => index - (board_size + 1),
            Move::TopRight => index - (board_size - 1),
            Move::BottomLeft => index + (board_size - 1),
            Move::BottomRight => index + (board_size + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Move;

    #[test]
    fn left_false() {
        let index = 0;
        let board_size = 4;
        assert_eq!(Move::Left.can_move(board_size, index), false);
    }

    #[test]
    fn left_true() {
        let index = 1;
        let board_size = 4;
        assert_eq!(Move::Left.can_move(board_size, index), true);
    }

    #[test]
    fn right_false() {
        let index = 3;
        let board_size = 4;
        assert_eq!(Move::Right.can_move(board_size, index), false);
    }

    #[test]
    fn right_true() {
        let index = 2;
        let board_size = 4;
        assert_eq!(Move::Right.can_move(board_size, index), true);
    }

    #[test]
    fn top_false() {
        let index = 2;
        let board_size = 4;
        assert_eq!(Move::Top.can_move(board_size, index), false);
    }

    #[test]
    fn top_true() {
        let index = 4;
        let board_size = 4;
        assert_eq!(Move::Top.can_move(board_size, index), true);
    }

    #[test]
    fn bot_false() {
        let index = 12;
        let board_size = 4;
        assert_eq!(Move::Bottom.can_move(board_size, index), false);
    }

    #[test]
    fn bot_true() {
        let index = 11;
        let board_size = 4;
        assert_eq!(Move::Bottom.can_move(board_size, index), true);
    }
}
