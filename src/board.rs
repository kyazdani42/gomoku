pub fn check_horizontal_alignment(
    board: &Vec<Vec<u8>>,
    line: usize,
    col: usize,
    player: u8,
    board_size: usize,
) -> bool {
    let mut stones = 1;

    let mut index = col as i32 - 1;
    while -1 < index && board[line][index as usize] == player {
        index -= 1;
        stones += 1;
    }

    index = col as i32 + 1;

    while index < board_size as i32 && board[line][index as usize] == player {
        index += 1;
        stones += 1;
    }

    4 < stones
}

pub fn check_vertical_alignment(
    board: &Vec<Vec<u8>>,
    line: usize,
    col: usize,
    player: u8,
    board_size: usize,
) -> bool {
    let mut stones = 1;

    let mut index = line as i32 - 1;
    while -1 < index && board[index as usize][col] == player {
        index -= 1;
        stones += 1;
    }

    index = line as i32 + 1;

    while index < board_size as i32 && board[index as usize][col] == player {
        index += 1;
        stones += 1;
    }

    4 < stones
}

pub fn check_diagonal_left_alignment(
    board: &Vec<Vec<u8>>,
    line: usize,
    col: usize,
    player: u8,
    board_size: usize,
) -> bool {
    let mut stones = 1;
    let mut i_col = col as i32 - 1;
    let mut i_line = line as i32 - 1;

    while -1 < i_col && -1 < i_line && board[i_line as usize][i_col as usize] == player {
        i_col -= 1;
        i_line -= 1;
        stones += 1;
    }

    i_col = col as i32 + 1;
    i_line = line as i32 + 1;

    let size = board_size as i32;
    while i_col < size && i_line < size && board[i_line as usize][i_col as usize] == player {
        i_col += 1;
        i_line += 1;
        stones += 1;
    }

    4 < stones
}

pub fn check_diagonal_right_alignment(
    board: &Vec<Vec<u8>>,
    line: usize,
    col: usize,
    player: u8,
    board_size: usize,
) -> bool {
    let mut stones = 1;
    let mut i_col = col as i32 - 1;
    let mut i_line = line as i32 + 1;

    let size = board_size as i32;

    while -1 < i_col && i_line < size && board[i_line as usize][i_col as usize] == player {
        i_col -= 1;
        i_line += 1;
        stones += 1;
    }

    i_col = col as i32 + 1;
    i_line = line as i32 - 1;

    while i_col < size && -1 < i_line && board[i_line as usize][i_col as usize] == player {
        i_col += 1;
        i_line -= 1;
        stones += 1;
    }

    4 < stones
}

#[cfg(test)]
mod test {
    use super::*;

    mod test_horizontal_alignment {
        use super::check_horizontal_alignment;
        #[test]
        fn test_false() {
            let vec = vec![
                vec![0, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
            ];
            assert_eq!(check_horizontal_alignment(&vec, 1, 2, 1, 4), false);
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
            assert_eq!(check_horizontal_alignment(&vec, 1, 2, 1, 5), true);
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
            assert_eq!(check_horizontal_alignment(&vec, 1, 0, 1, 5), true);
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
            assert_eq!(check_horizontal_alignment(&vec, 1, 4, 1, 5), true);
        }
    }

    mod test_vertical_alignment {
        use super::check_vertical_alignment;
        #[test]
        fn test_false() {
            let vec = vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 1],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
            ];
            assert_eq!(check_vertical_alignment(&vec, 1, 1, 1, 4), false);
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
            assert_eq!(check_vertical_alignment(&vec, 1, 1, 1, 5), true);
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
            assert_eq!(check_vertical_alignment(&vec, 0, 1, 1, 5), true);
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
            assert_eq!(check_vertical_alignment(&vec, 4, 1, 1, 5), true);
        }
    }

    mod test_diagonal_left_alignment {
        use super::check_diagonal_left_alignment;
        #[test]
        fn test_false() {
            let vec = vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 1],
                vec![0, 1, 0, 1],
                vec![0, 1, 0, 0],
            ];
            assert_eq!(check_diagonal_left_alignment(&vec, 1, 2, 1, 4), false);
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
            assert_eq!(check_diagonal_left_alignment(&vec, 2, 2, 1, 5), true);
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
            assert_eq!(check_diagonal_left_alignment(&vec, 0, 0, 1, 5), true);
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
            assert_eq!(check_diagonal_left_alignment(&vec, 4, 4, 1, 5), true);
        }
    }

    mod test_diagonal_right_alignment {
        use super::check_diagonal_right_alignment;
        #[test]
        fn test_false() {
            let vec = vec![
                vec![0, 1, 0, 1],
                vec![1, 1, 1, 1],
                vec![0, 1, 0, 1],
                vec![1, 1, 0, 0],
            ];
            assert_eq!(check_diagonal_right_alignment(&vec, 1, 2, 1, 4), false);
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
            assert_eq!(check_diagonal_right_alignment(&vec, 2, 2, 1, 5), true);
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
            assert_eq!(check_diagonal_right_alignment(&vec, 4, 0, 1, 5), true);
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
            assert_eq!(check_diagonal_right_alignment(&vec, 0, 4, 1, 5), true);
        }
    }
}

