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
    /// Check if we can move from `index` to a direction `depth` times
    pub fn can_move_to(self, board_size: i32, index: i32, depth: i32) -> bool {
        match self {
            Move::Left => index - depth > -1 && (index - depth) / board_size == index / board_size,
            Move::Right => (index + depth) / board_size == index / board_size,
            Move::Top => index - depth * board_size > -1,
            Move::Bottom => index + depth * board_size < board_size * board_size,
            Move::TopLeft => {
                Move::Left.can_move_to(board_size, index, depth)
                    && Move::Top.can_move_to(board_size, index, depth)
            }
            Move::TopRight => {
                Move::Right.can_move_to(board_size, index, depth)
                    && Move::Top.can_move_to(board_size, index, depth)
            }
            Move::BottomLeft => {
                Move::Left.can_move_to(board_size, index, depth)
                    && Move::Bottom.can_move_to(board_size, index, depth)
            }
            Move::BottomRight => {
                Move::Right.can_move_to(board_size, index, depth)
                    && Move::Bottom.can_move_to(board_size, index, depth)
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
    fn can_move_to_left_true() {
        let index = 5;
        let board_size = 10;
        assert_eq!(Move::Left.can_move_to(board_size, index, 5), true);
    }

    #[test]
    fn can_move_to_left_false() {
        let index = 4;
        let board_size = 10;
        assert_eq!(Move::Left.can_move_to(board_size, index, 5), false);
    }

    #[test]
    fn can_move_to_right_true() {
        let index = 5;
        let board_size = 10;
        assert_eq!(Move::Right.can_move_to(board_size, index, 4), true);
    }

    #[test]
    fn can_move_to_right_false() {
        let index = 5;
        let board_size = 10;
        assert_eq!(Move::Right.can_move_to(board_size, index, 5), false);
    }

    #[test]
    fn can_move_to_right_end_false() {
        let index = 99;
        let board_size = 10;
        assert_eq!(Move::Right.can_move_to(board_size, index, 1), false);
    }

    #[test]
    fn can_move_to_top_true() {
        let index = 24;
        let board_size = 5;
        assert_eq!(Move::Top.can_move_to(board_size, index, 4), true);
    }

    #[test]
    fn can_move_to_top_false() {
        let index = 19;
        let board_size = 5;
        assert_eq!(Move::Top.can_move_to(board_size, index, 4), false);
    }

    #[test]
    fn can_move_to_bottom_true() {
        let index = 0;
        let board_size = 5;
        assert_eq!(Move::Bottom.can_move_to(board_size, index, 4), true);
    }

    #[test]
    fn can_move_to_bottom_false() {
        let index = 0;
        let board_size = 5;
        assert_eq!(Move::Bottom.can_move_to(board_size, index, 5), false);
    }
}