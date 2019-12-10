use super::{
    check_alignment, check_double_free_threes, move_stone, Board, GameState, Stone, ACTIONS,
    JOINED_ACTIONS,
};

pub fn place_stone(state: &mut GameState, line: usize, col: usize) -> Option<()> {
    let stone = Stone(line, col);
    if line >= state.board_size || col >= state.board_size || get_value(&state.board, &stone) != 0 {
        None
    } else {
        set_value(state, &stone, state.player);
        state.stone = stone;
        Some(())
    }
}

pub fn capture_all(state: &mut GameState) {
    let other_player = switch_player(state.player);
    ACTIONS
        .iter()
        .for_each(|action| capture(state, action, &other_player));
}

fn capture(state: &mut GameState, action: &str, other_player: &u8) {
    let stone_one: Stone = match move_stone(&state.stone, state.board_size, action) {
        Some(stone) if get_value(&state.board, &stone) == *other_player => stone,
        _ => return,
    };
    let stone_two: Stone = match move_stone(&stone_one, state.board_size, action) {
        Some(stone) if get_value(&state.board, &stone) == *other_player => stone,
        _ => return,
    };

    if let Some(stone) = move_stone(&stone_two, state.board_size, action) {
        if get_value(&state.board, &stone) == state.player {
            set_value(state, &stone_one, 0);
            set_value(state, &stone_two, 0);
            if state.player == 1 {
                state.player_one_captured += 2;
            } else {
                state.player_two_captured += 2;
            }
        }
    };
}

pub fn check_winner(state: &GameState) -> bool {
    win_by_capture(state) || win_by_alignment(state)
}

fn win_by_capture(state: &GameState) -> bool {
    if state.player == 1 {
        state.player_one_captured == 10
    } else {
        state.player_two_captured == 10
    }
}

fn win_by_alignment(state: &GameState) -> bool {
    JOINED_ACTIONS.iter().any(|actions| {
        check_alignment(
            &state.board,
            &state.stone,
            state.player,
            state.board_size,
            *actions,
        ) == true
    })
}

pub fn set_free_threes(state: &mut GameState) {
    // we need to do this because we mutate the board later on
    // and we cannot mutate the board when it's borrowed as immutable
    let board = state.board.clone();
    let player = state.player;
    let board_size = state.board_size;
    for (i_line, line) in board.iter().enumerate() {
        for (i_col, value) in line.iter().enumerate() {
            // if its empty or a free three
            if *value == 0 || *value == 3 {
                let stone = Stone(i_line, i_col);
                let is_double_free_three =
                    check_double_free_threes(&state.board, &stone, player, board_size);

                // TODO: if the player can capture, do not set the value
                if is_double_free_three {
                    set_value(state, &stone, 3);
                } else if *value == 3 {
                    set_value(state, &stone, 0);
                }
            }
        }
    }
}

pub fn has_neighbour(board: &Board, board_size: usize, stone: &Stone) -> bool {
    ACTIONS.iter().any(|action| {
        if let Some(neighbour) = move_stone(&stone, board_size, action) {
            let value = get_value(board, &neighbour);
            if value == 1 || value == 2 {
                return true;
            };
        }
        false
    })
}

pub fn switch_player(player: u8) -> u8 {
    if player == 1 {
        2
    } else {
        1
    }
}

pub fn set_value(state: &mut GameState, stone: &Stone, player: u8) {
    let Stone(line, col) = *stone;
    state.board[line][col] = player;
}

pub fn get_value(board: &Board, stone: &Stone) -> u8 {
    board[stone.0][stone.1]
}
