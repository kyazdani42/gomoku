use super::{GameState, Player, Stones, ACTIONS};

pub fn place_stone(state: &mut GameState, index: usize) -> Option<()> {
    let board_size = state.board_size;
    let line = index / board_size;
    let col = index % board_size;
    if line >= board_size || col >= board_size || state.placed.get(&index).is_some() {
        None
    } else {
        state.placed.insert(index, state.player);
        state.last_played = index;
        Some(())
    }
}

pub fn switch_player(player: u8) -> u8 {
    if player == 1 {
        2
    } else {
        1
    }
}

pub fn get_value(placed: &Stones, index: usize) -> Player {
    if let Some(i) = placed.get(&index) {
        *i
    } else {
        0
    }
}

pub fn get_all_playable_indexes(placed: &Stones, board_size: usize) -> Vec<usize> {
    let mut indexes = vec![];
    for (i, v) in placed {
        if *v == 1 || *v == 2 {
            for neighbour in get_empty_neighbours(placed, *i, board_size) {
                indexes.push(neighbour);
            }
        }
    }

    indexes
}

pub fn get_empty_neighbours(placed: &Stones, index: usize, board_size: usize) -> Vec<usize> {
    ACTIONS.iter().fold(vec![], |mut neighbours, action| {
        if let Some(neighbour) = move_stone(index, board_size, action) {
            if get_value(placed, neighbour) == 0 {
                neighbours.push(neighbour);
            }
        }

        neighbours
    })
}

pub fn move_stone(index: usize, board_size: usize, dir: &str) -> Option<usize> {
    match dir {
        "left" if left(index, board_size) => Some(index - 1),
        "right" if right(index, board_size) => Some(index + 1),
        "top" if top(index, board_size) => Some(index - board_size),
        "bot" if bot(index, board_size) => Some(index + board_size),
        "bot_right" if bot_right(index, board_size) => Some(index + board_size + 1),
        "top_right" if top_right(index, board_size) => Some(index - (board_size - 1)),
        "bot_left" if bot_left(index, board_size) => Some(index + (board_size - 1)),
        "top_left" if top_left(index, board_size) => Some(index - (board_size + 1)),
        _ => None,
    }
}

fn bot_right(index: usize, board_size: usize) -> bool {
    bot(index, board_size) && right(index, board_size)
}

fn bot_left(index: usize, board_size: usize) -> bool {
    bot(index, board_size) && left(index, board_size)
}

fn top_right(index: usize, board_size: usize) -> bool {
    top(index, board_size) && right(index, board_size)
}

fn top_left(index: usize, board_size: usize) -> bool {
    top(index, board_size) && left(index, board_size)
}

fn left(index: usize, board_size: usize) -> bool {
    0 < index % board_size
}

fn right(index: usize, board_size: usize) -> bool {
    index % board_size < board_size - 1
}

fn top(index: usize, board_size: usize) -> bool {
    board_size <= index
}

fn bot(index: usize, board_size: usize) -> bool {
    index / board_size < board_size - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    mod move_stone {
        use super::move_stone;

        #[test]
        fn left_false() {
            let index = 0;
            let board_size = 4;
            let dir = "left";
            assert_eq!(move_stone(index, board_size, dir).is_some(), false);
        }

        #[test]
        fn left_true() {
            let index = 1;
            let board_size = 4;
            let dir = "left";
            assert_eq!(move_stone(index, board_size, dir).is_some(), true);
        }

        #[test]
        fn right_false() {
            let index = 3;
            let board_size = 4;
            let dir = "right";
            assert_eq!(move_stone(index, board_size, dir).is_some(), false);
        }

        #[test]
        fn right_true() {
            let index = 2;
            let board_size = 4;
            let dir = "right";
            assert_eq!(move_stone(index, board_size, dir).is_some(), true);
        }

        #[test]
        fn top_false() {
            let index = 2;
            let board_size = 4;
            let dir = "top";
            assert_eq!(move_stone(index, board_size, dir).is_some(), false);
        }

        #[test]
        fn top_true() {
            let index = 4;
            let board_size = 4;
            let dir = "top";
            assert_eq!(move_stone(index, board_size, dir).is_some(), true);
        }

        #[test]
        fn bot_false() {
            let index = 12;
            let board_size = 4;
            let dir = "bot";
            assert_eq!(move_stone(index, board_size, dir).is_some(), false);
        }

        #[test]
        fn bot_true() {
            let index = 11;
            let board_size = 4;
            let dir = "bot";
            assert_eq!(move_stone(index, board_size, dir).is_some(), true);
        }
    }
}
