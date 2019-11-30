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

pub fn check_double_free_threes(
    board: &Vec<Vec<u8>>,
    stone: &Stone,
    player: u8,
    board_size: usize,
) -> bool {

    let mut free_threes = 0;

    let actions = [
        ["bot_left", "top_right"],
        ["left", "right"],
        ["top", "bot"],
        ["top_left", "bot_right"],
        ["bot_right", "top_left"],
        ["bot", "top"],
        ["right", "left"],
        ["top_right", "bot_left"],
    ];
    let mut i = 0;
    while i < 8 {
        if is_free_threes(
            board,
            stone,
            player,
            board_size,
            actions[i][0],
            actions[i][1],
                i > 3
        ) {

            free_threes += 1;
        }
        println!("{}", free_threes);
        if free_threes == 2 {
            return true;
        }
        i += 1;
    }
    free_threes > 1
}

// cas :
// avant vide :
    // suivant vide :
        //suivant joueur
            // suivant joueur
                //suivant vide
                    //FREE
    // suivant joueur:
        // suivant joueur
            // suivant vide
                // suivant joueur
                    // suivant vide
                        //FREE
            // suivant joueur
                // suivant vide
                    // FREE
// avant joueur
    // avant vide
        // suivant joueur
            // suivant vide
                //FREE
        //suivant vide
            //suivant joueur
                //suivant vide
                    //FREE


//previous vide
    //next joueur
        //next vide
            //next joueur
                //next vide
                    //FREE
        //next joueur
            //next vide
                //FREE


    //next vide
        //next joueur
            //next joueur
                //next vide
                    //FREE



pub fn is_free_threes(
    board: &Vec<Vec<u8>>,
    stone: &Stone,
    player: u8,
    board_size: usize,
    action_one: &str,
    action_two: &str,
    second_check: bool,
) -> bool {
    let mut previous = move_stone(&stone, board_size, action_one);
    let mut next = move_stone(&stone, board_size, action_two);
    match previous {
        Some(stone) => {
            if board[stone.0][stone.1] == 0 {
                match next {
                    Some(stone) => {
                        if board[stone.0][stone.1] == 0 {
                            next = move_stone(&Stone(stone.0, stone.1), board_size, action_two);
                            match next {
                                Some(stone) => {
                                    if board[stone.0][stone.1] == player {
                                        next = move_stone(&Stone(stone.0, stone.1), board_size, action_two);
                                        match next {
                                            Some(stone) => {
                                                if board[stone.0][stone.1] == player {
                                                    next = move_stone(&Stone(stone.0, stone.1), board_size, action_two);
                                                    match next {
                                                        Some(stone) => {
                                                            if board[stone.0][stone.1] == 0 {
                                                                return true;
                                                            }
                                                        }
                                                        None => return false
                                                    }
                                                }
                                            }
                                            None => return false
                                        }
                                    }
                                }
                                None => return false
                            }
                        } else if board[stone.0][stone.1] == player {
                            next = move_stone(&Stone(stone.0, stone.1), board_size, action_two);
                            match next {
                                Some(stone) => {
                                    if board[stone.0][stone.1] == player {
                                        next = move_stone(&Stone(stone.0, stone.1), board_size, action_two);
                                        match next {
                                            Some(stone) => {
                                                if board[stone.0][stone.1] == 0 {
                                                    return true;
                                                }
                                            }
                                            None => return false
                                        }
                                    } else if board[stone.0][stone.1] == 0 {
                                        next = move_stone(&Stone(stone.0, stone.1), board_size, action_two);
                                        match next {
                                            Some(stone) => {
                                                if board[stone.0][stone.1] == player {
                                                    next = move_stone(&Stone(stone.0, stone.1), board_size, action_two);
                                                    match next {
                                                        Some(stone) => {
                                                            if board[stone.0][stone.1] == 0 {
                                                                return true;
                                                            }
                                                        }
                                                        None => return false
                                                    }
                                                }
                                            }
                                            None => return false
                                        }
                                    }
                                }
                                None => return false
                            }
                        }
                    }
                    None => return false,
                };
            } else if board[stone.0][stone.1] == player {
                previous = move_stone(&Stone(stone.0, stone.1), board_size, action_one);
                match previous {
                    Some(stone) => {
                        if board[stone.0][stone.1] == 0 {
                            match next {
                                Some(stone) => {
                                    if board[stone.0][stone.1] == player {
                                        next = move_stone(&Stone(stone.0, stone.1), board_size, action_two);
                                        match next {
                                            Some(stone) => {
                                                if board[stone.0][stone.1] == 0 {
                                                    return !second_check && true;
                                                }
                                            }
                                            None => return false
                                        }
                                    } else if board[stone.0][stone.1] == 0 {
                                        next = move_stone(&Stone(stone.0, stone.1), board_size, action_two);
                                        match next {
                                            Some(stone) => {
                                                if board[stone.0][stone.1] == player {
                                                    next = move_stone(&Stone(stone.0, stone.1), board_size, action_two);
                                                    match next {
                                                        Some(stone) => {
                                                            if board[stone.0][stone.1] == 0 {
                                                                return true;
                                                            }
                                                        }
                                                        None => return false
                                                    }
                                                }
                                            }
                                            None => return false
                                        }
                                    }
                                }
                                None => return false
                            }
                        }
                    }
                    None => return false
                }
            }
        }
        None => return false,
    };
    false
}

pub fn check_free_threes(
    board: &Vec<Vec<u8>>,
    stone: &Stone,
    player: u8,
    board_size: usize,
    action_one: &str,
    action_two: &str,
) -> bool {
    let mut empty_stone = 2;
    let mut next = move_stone(&stone, board_size, action_two);

    let mut i = 0;

    while i < 4 {
        match next {
            Some(stone) => {
                if board[stone.0][stone.1] == 0  {
                    if empty_stone == 0 || i == 0 {
                        return false;
                    }
                    if i != 3  {
                        empty_stone -= 1;
                    }
                } else if board[stone.0][stone.1] != player{
                    return false;
                }
                next = move_stone(&Stone(stone.0, stone.1), board_size, action_two);
            }
            None => return false,
        };

        i+=1;
    }

    if empty_stone < 1 {
        return false;
    }
    if empty_stone == 1{
        match next {
            Some(stone) => {
                if board[stone.0][stone.1] != 0 {
                    return false;
                }
            }
            None => return false,
        };
    }
    true
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

    mod two_free_threes {
        use super::{check_double_free_threes, Stone};

        #[test]
        fn test_true_one() {
            let vec = vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
            ];
            assert_eq!(
                check_double_free_threes(&vec, &Stone(4, 4), 1, 8),
                true
            );
        }
        #[test]
        fn test_false_tow_protect() {
            let vec = vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 2, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
            ];
            assert_eq!(
                check_double_free_threes(&vec, &Stone(4, 4), 1, 8),
                false
            );
        }
        #[test]
        fn test_true_diagonal() {
            let vec = vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            ];
            assert_eq!(
                check_double_free_threes(&vec, &Stone(4, 4), 1, 9),
                true
            );
        }
        #[test]
        fn test_false_diagonal_two_protect() {
            let vec = vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 2],
            ];
            assert_eq!(
                check_double_free_threes(&vec, &Stone(4, 4), 1, 9),
                false
            );
        }
        #[test]
        fn test_false_diagonal_one() {
            let vec = vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            ];
            assert_eq!(
                check_double_free_threes(&vec, &Stone(4, 4), 1, 9),
                false
            );
        }
        #[test]
        fn test_false_diagonal() {
            let vec = vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 2],
            ];
            assert_eq!(
                check_double_free_threes(&vec, &Stone(4, 4), 1, 9),
                false
            );
        }
        #[test]
        fn test_false_line() {
            let vec = vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 1, 1, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            ];
            assert_eq!(
                check_double_free_threes(&vec, &Stone(4, 4), 1, 9),
                false
            );
        }
        #[test]
        fn test_false_line_one() {
            let vec = vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 1, 1, 0, 1, 2],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            ];
            assert_eq!(
                check_double_free_threes(&vec, &Stone(4, 4), 1, 9),
                false
            );
        }
    }
}
