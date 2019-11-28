use crate::game::game_state::Stone;
use crate::game::JOINED_ACTIONS;

pub fn check_alignment(
    board: &Vec<Vec<u8>>,
    stone: &Stone,
    player: u8,
    board_size: usize,
    actions: &str,
) -> bool {
    4 < actions
        .split('|')
        .into_iter()
        .fold(1, |mut stones, action| {
            stones += get_aligned_stones(board, stone, player, board_size, action, actions);
            stones
        })
}

fn get_aligned_stones(
    board: &Vec<Vec<u8>>,
    stone: &Stone,
    player: u8,
    board_size: usize,
    action: &str,
    actions: &str,
) -> i32 {
    let mut new_stone = move_stone(&stone, board_size, action);
    let mut stones = 0;

    while let Some(next_stone) = new_stone {
        if board[next_stone.0][next_stone.1] != player {
            break;
        }
        if is_capturable(&next_stone, board, player, actions) {
            break;
        }
        stones += 1;
        new_stone = move_stone(&next_stone, board_size, action);
    }
    stones
}

fn get_all_moves(actions: &str) -> Vec<&str> {
    JOINED_ACTIONS
        .iter()
        .map(|x| *x)
        .filter(|x| *x != actions)
        .collect::<Vec<&str>>()
}

fn is_capturable(stone: &Stone, board: &Vec<Vec<u8>>, player: u8, actions: &str) -> bool {
    let other_player = if player == 1 { 2 } else { 1 };
    get_all_moves(actions).iter().any(|actions| {
        let actions = actions.split('|').into_iter().collect::<Vec<&str>>();

        if let Some(stone_side) = move_stone(stone, board.len(), actions[0]) {
            if let Some(stone_oside) = move_stone(stone, board.len(), actions[1]) {
                let value_side = board[stone_side.0][stone_side.1];
                let value_oside = board[stone_oside.0][stone_oside.1];

                if value_side == player
                    && check_place_capture(
                        board,
                        &stone_side,
                        value_oside,
                        actions[0],
                        other_player,
                    )
                {
                    return true;
                }
                if value_oside == player
                    && check_place_capture(
                        board,
                        &stone_oside,
                        value_side,
                        actions[1],
                        other_player,
                    )
                {
                    return true;
                };
            }
        };

        false
    })
}

fn check_place_capture(
    board: &Vec<Vec<u8>>,
    stone: &Stone,
    value: u8,
    action: &str,
    player: u8,
) -> bool {
    if let Some(Stone(x, y)) = move_stone(stone, board.len(), action) {
        let value_2 = board[x][y];
        return (value == 0 && value_2 == player) || (value == player && value_2 == 0);
    }
    false
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
                check_alignment(&vec, &Stone(1, 2), 1, 4, "left|right"),
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
                check_alignment(&vec, &Stone(1, 2), 1, 5, "left|right"),
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
                check_alignment(&vec, &Stone(1, 0), 1, 5, "left|right"),
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
                check_alignment(&vec, &Stone(1, 4), 1, 5, "left|right"),
                true
            );
        }

        #[test]
        fn test_false_from_capture() {
            let vec = vec![
                vec![0, 0, 0, 2, 0],
                vec![1, 1, 1, 1, 1],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(1, 4), 1, 5, "left|right"),
                false
            );
        }

        #[test]
        fn test_capture_occupied() {
            let vec = vec![
                vec![0, 0, 2, 2, 0],
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
            ];
            assert_eq!(
                check_alignment(&vec, &Stone(1, 4), 1, 5, "left|right"),
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
            assert_eq!(check_alignment(&vec, &Stone(1, 1), 1, 4, "top|bot"), false);
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
            assert_eq!(check_alignment(&vec, &Stone(1, 1), 1, 5, "top|bot"), true);
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
            assert_eq!(check_alignment(&vec, &Stone(0, 1), 1, 5, "top|bot"), true);
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
            assert_eq!(check_alignment(&vec, &Stone(4, 1), 1, 5, "top|bot"), true);
        }

        #[test]
        fn test_false_from_capture() {
            let vec = vec![
                vec![2, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 1],
                vec![0, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
            ];
            assert_eq!(check_alignment(&vec, &Stone(4, 1), 1, 5, "top|bot"), false);
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
                check_alignment(&vec, &Stone(1, 2), 1, 4, "top_left|bot_right"),
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
                check_alignment(&vec, &Stone(2, 2), 1, 5, "top_left|bot_right"),
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
                check_alignment(&vec, &Stone(0, 0), 1, 5, "top_left|bot_right"),
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
                check_alignment(&vec, &Stone(4, 4), 1, 5, "top_left|bot_right"),
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
                check_alignment(&vec, &Stone(1, 2), 1, 4, "bot_left|top_right"),
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
                check_alignment(&vec, &Stone(2, 2), 1, 5, "bot_left|top_right"),
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
                check_alignment(&vec, &Stone(4, 0), 1, 5, "bot_left|top_right"),
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
                check_alignment(&vec, &Stone(0, 4), 1, 5, "bot_left|top_right"),
                true
            );
        }
    }
}
