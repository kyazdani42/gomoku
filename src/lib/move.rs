use std::cmp::min;

use super::game::Tile;

#[derive(Clone)]
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
    pub fn can_move_to(&self, board_size: i32, tile: &Tile, depth: i32) -> bool {
        match self {
            Move::Left => tile.1 - depth > -1,
            Move::Right => tile.1 + depth < board_size,
            Move::Top => tile.0 - depth > -1,
            Move::Bottom => tile.0 + depth < board_size,
            Move::TopLeft => tile.0 - depth > -1 && tile.1 - depth > -1,
            Move::TopRight => tile.0 - depth > -1 && tile.1 + depth < board_size,
            Move::BottomLeft => tile.0 + depth < board_size && tile.1 - depth > -1,
            Move::BottomRight => tile.0 + depth < board_size && tile.1 + depth < board_size,
        }
    }

    pub fn num_move_to(&self, board_size: i32, tile: &Tile) -> i32 {
        match self {
            Move::Left => tile.1,
            Move::Right => (board_size - 1) - tile.1,
            Move::Top => tile.0,
            Move::Bottom => (board_size - 1) - tile.0,
            Move::TopLeft => min(tile.0, tile.1),
            Move::TopRight => min(tile.0, (board_size - 1) - tile.1),
            Move::BottomLeft => min((board_size - 1) - tile.0, tile.1),
            Move::BottomRight => min((board_size - 1) - tile.0, (board_size - 1) - tile.1),
        }
    }

    pub fn get_next_tile(&self, tile: &Tile) -> Tile {
        match self {
            Move::Left => (tile.0, tile.1 - 1),
            Move::Right => (tile.0, tile.1 + 1),
            Move::Top => (tile.0 - 1, tile.1),
            Move::Bottom => (tile.0 + 1, tile.1),
            Move::TopLeft => (tile.0 - 1, tile.1 - 1),
            Move::TopRight => (tile.0 - 1, tile.1 + 1),
            Move::BottomLeft => (tile.0 + 1, tile.1 - 1),
            Move::BottomRight => (tile.0 + 1, tile.1 + 1),
        }
    }

    pub fn get_tile_mult(&self, tile: &Tile, depth: i32) -> Tile {
        match self {
            Move::Left => (tile.0, tile.1 - depth),
            Move::Right => (tile.0, tile.1 + depth),
            Move::Top => (tile.0 - depth, tile.1),
            Move::Bottom => (tile.0 + depth, tile.1),
            Move::TopLeft => (tile.0 - depth, tile.1 - depth),
            Move::TopRight => (tile.0 - depth, tile.1 + depth),
            Move::BottomLeft => (tile.0 + depth, tile.1 - depth),
            Move::BottomRight => (tile.0 + depth, tile.1 + depth),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::Move;
//
//     #[test]
//     fn can_move_to_left_true() {
//         let index = 5;
//         let board_size = 10;
//         assert_eq!(Move::Left.can_move_to(board_size, index, 5), true);
//     }
//
//     #[test]
//     fn can_move_to_left_false() {
//         let index = 4;
//         let board_size = 10;
//         assert_eq!(Move::Left.can_move_to(board_size, index, 5), false);
//     }
//
//     #[test]
//     fn can_move_to_right_true() {
//         let index = 5;
//         let board_size = 10;
//         assert_eq!(Move::Right.can_move_to(board_size, index, 4), true);
//     }
//
//     #[test]
//     fn can_move_to_right_false() {
//         let index = 5;
//         let board_size = 10;
//         assert_eq!(Move::Right.can_move_to(board_size, index, 5), false);
//     }
//
//     #[test]
//     fn can_move_to_right_end_false() {
//         let index = 99;
//         let board_size = 10;
//         assert_eq!(Move::Right.can_move_to(board_size, index, 1), false);
//     }
//
//     #[test]
//     fn can_move_to_top_true() {
//         let index = 24;
//         let board_size = 5;
//         assert_eq!(Move::Top.can_move_to(board_size, index, 4), true);
//     }
//
//     #[test]
//     fn can_move_to_top_false() {
//         let index = 19;
//         let board_size = 5;
//         assert_eq!(Move::Top.can_move_to(board_size, index, 4), false);
//     }
//
//     #[test]
//     fn can_move_to_bottom_true() {
//         let index = 0;
//         let board_size = 5;
//         assert_eq!(Move::Bottom.can_move_to(board_size, index, 4), true);
//     }
//
//     #[test]
//     fn can_move_to_bottom_false() {
//         let index = 0;
//         let board_size = 5;
//         assert_eq!(Move::Bottom.can_move_to(board_size, index, 5), false);
//     }
// }
