pub use super::game_state::Stone;

fn nb_aligned(
    board: &Vec<Vec<u8>>,
    stone: &Stone,
    player: u8,
    board_size: usize,
    action: &str,
) -> i32 {
    let mut new_stone = move_stone(&stone, board_size, action);
    let mut stones = 0;

    while let Some(_) = new_stone {
        let next_stone = new_stone.unwrap();
        let Stone(line, col) = next_stone;
        if board[line][col] == player {
            stones += 1;
        } else {
            break;
        }
        new_stone = move_stone(&next_stone, board_size, action);
    }
    stones
}

pub fn check_alignment(
    board: &Vec<Vec<u8>>,
    stone: &Stone,
    player: u8,
    board_size: usize,
    action_one: &str,
    action_two: &str,
) -> bool {
    4 < 1
        + nb_aligned(board, stone, player, board_size, action_one)
        + nb_aligned(board, stone, player, board_size, action_two)
}

pub fn move_stone(stone: &Stone, board_size: usize, dir: &str) -> Option<Stone> {
    match dir {
        "left" => {
            if stone.1 == 0 {
                None
            } else {
                Some(Stone(stone.0, stone.1 - 1))
            }
        }
        "right" => {
            if stone.1 == board_size - 1 {
                None
            } else {
                Some(Stone(stone.0, stone.1 + 1))
            }
        }
        "top" => {
            if stone.0 == 0 {
                None
            } else {
                Some(Stone(stone.0 - 1, stone.1))
            }
        }
        "bot" => {
            if stone.0 == board_size - 1 {
                None
            } else {
                Some(Stone(stone.0 + 1, stone.1))
            }
        }
        "bot_right" => {
            if stone.0 == board_size - 1 || stone.1 == board_size - 1 {
                None
            } else {
                Some(Stone(stone.0 + 1, stone.1 + 1))
            }
        }
        "top_right" => {
            if stone.0 == 0 || stone.1 == board_size - 1 {
                None
            } else {
                Some(Stone(stone.0 - 1, stone.1 + 1))
            }
        }
        "bot_left" => {
            if stone.0 == board_size - 1 || stone.1 == 0 {
                None
            } else {
                Some(Stone(stone.0 + 1, stone.1 - 1))
            }
        }
        "top_left" => {
            if stone.0 == 0 || stone.1 == 0 {
                None
            } else {
                Some(Stone(stone.0 - 1, stone.1 - 1))
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod test_horizontal_alignment {
        use super::{check_alignment, Stone};
        #[test]
        fn test_false() {
            let vec = vec![
                vec![0, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(1, 2), 1, 4, "left", "right"),
                false
            );
        }

        #[test]
        fn test_true() {
            let vec = vec![
                vec![0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(1, 2), 1, 5, "left", "right"),
                true
            );
        }

        #[test]
        fn test_true_from_0() {
            let vec = vec![
                vec![0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(1, 0), 1, 5, "left", "right"),
                true
            );
        }

        #[test]
        fn test_true_from_len() {
            let vec = vec![
                vec![0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(1, 4), 1, 5, "left", "right"),
                true
            );
        }
    }

    mod test_vertical_alignment {
        use super::{check_alignment, Stone};

        #[test]
        fn test_false() {
            let vec = vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 1],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(1, 1), 1, 4, "top", "bot"),
                false
            );
        }

        #[test]
        fn test_true() {
            let vec = vec![
                vec![0, 1, 0, 0, 0],
                vec![1, 1, 0, 1, 1],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(1, 1), 1, 5, "top", "bot"),
                true
            );
        }

        #[test]
        fn test_true_from_0() {
            let vec = vec![
                vec![0, 1, 0, 0, 0],
                vec![1, 1, 0, 1, 1],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(0, 1), 1, 5, "top", "bot"),
                true
            );
        }

        #[test]
        fn test_true_from_len() {
            let vec = vec![
                vec![0, 1, 0, 0, 0],
                vec![1, 1, 0, 1, 1],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(4, 1), 1, 5, "top", "bot"),
                true
            );
        }
    }

    mod test_diagonal_left_alignment {
        use super::{check_alignment, Stone};

        #[test]
        fn test_false() {
            let vec = vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 1],
                vec![0, 1, 0, 1],
                vec![0, 1, 0, 0],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(1, 2), 1, 4, "top_left", "bot_right"),
                false
            );
        }

        #[test]
        fn test_true() {
            let vec = vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 0, 1, 1],
                vec![0, 1, 1, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 1, 0, 0, 1],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(2, 2), 1, 5, "top_left", "bot_right"),
                true
            );
        }

        #[test]
        fn test_true_from_0() {
            let vec = vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 0, 1, 1],
                vec![0, 1, 1, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 1, 0, 0, 1],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(0, 0), 1, 5, "top_left", "bot_right"),
                true
            );
        }

        #[test]
        fn test_true_from_len() {
            let vec = vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 0, 0, 1],
                vec![0, 1, 1, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 1, 0, 0, 1],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(4, 4), 1, 5, "top_left", "bot_right"),
                true
            );
        }
    }

    mod test_diagonal_right_alignment {
        use super::{check_alignment, Stone};

        #[test]
        fn test_false() {
            let vec = vec![
                vec![0, 1, 0, 1],
                vec![1, 1, 1, 1],
                vec![0, 1, 0, 1],
                vec![1, 1, 0, 0],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(1, 2), 1, 4, "bot_left", "top_right"),
                false
            );
        }

        #[test]
        fn test_true() {
            let vec = vec![
                vec![1, 1, 0, 0, 1],
                vec![0, 0, 0, 1, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(2, 2), 1, 5, "bot_left", "top_right"),
                true
            );
        }

        #[test]
        fn test_true_from_0() {
            let vec = vec![
                vec![0, 1, 0, 0, 1],
                vec![1, 0, 0, 1, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![1, 1, 0, 0, 1],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(4, 0), 1, 5, "bot_left", "top_right"),
                true
            );
        }

        #[test]
        fn test_true_from_len() {
            let vec = vec![
                vec![0, 1, 0, 0, 1],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![1, 0, 0, 0, 0],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(0, 4), 1, 5, "bot_left", "top_right"),
                true
            );
        }
    }
}
