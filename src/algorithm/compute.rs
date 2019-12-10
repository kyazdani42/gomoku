use std::cmp::{max, min};
use std::i32::{MAX, MIN};
use std::time::Instant;

use crate::game::{get_aligned_stones, has_neighbour, set_value, switch_player};
use crate::game::{Board, GameState, Stone, JOINED_ACTIONS};

pub fn compute(state: &GameState) -> (usize, usize) {
    let mut play_line = 0;
    let mut play_col = 0;
    let mut point = 0;
    let GameState {
        player,
        board_size,
        board,
        ..
    } = state;
    let player = *player;
    let board_size = *board_size;
    let other_player = switch_player(player);

    if state.init == true {
        return (board_size / 2, board_size / 2);
    }

    let time = Instant::now();
    let parent_node = Node::new(state, 2);
    println!("{}ms", time.elapsed().as_millis());
    println!("children: {}", parent_node.children.len());

    println!("{}", minimax(&parent_node, 2, true));

    for (i_line, line) in board.iter().enumerate() {
        for (i_col, col) in line.iter().enumerate() {
            if *col == 0 {
                let mut stone_point = get_basic_point(i_line, i_col, board_size);
                stone_point += get_alignement_point(board, i_line, i_col, board_size, player);
                stone_point += get_alignement_point(board, i_line, i_col, board_size, other_player);
                if stone_point > point {
                    point = stone_point;
                    play_line = i_line;
                    play_col = i_col;
                }
            }
        }
    }

    (play_line, play_col)
}

#[derive(Debug)]
struct Node {
    state: GameState,
    children: Vec<Node>,
    heuristic: i32,
}

impl Node {
    pub fn new(state: &GameState, depth: u8) -> Node {
        generate_tree(state.clone(), depth, state.player)
    }
}

fn generate_tree(state: GameState, depth: u8, player: u8) -> Node {
    let children = generate_children(&state, depth, player);
    // TODO: compute heuristic
    let heuristic = 1;
    Node {
        state,
        children,
        heuristic,
    }
}

fn generate_children(state: &GameState, depth: u8, player: u8) -> Vec<Node> {
    let mut children: Vec<Node> = vec![];
    if depth == 0 {
        return children;
    }

    let new_depth = depth - 1;
    let board_size = state.board_size;

    for (il, line) in state.board.iter().enumerate() {
        for (ic, value) in line.iter().enumerate() {
            let stone = Stone(il, ic);
            if *value == 0 && has_neighbour(&state.board, board_size, &stone) == true {
                children.push(create_child(state, new_depth, stone, player))
            }
        }
    }

    children
}

fn create_child(state: &GameState, new_depth: u8, stone: Stone, player: u8) -> Node {
    let mut new_state = state.clone();

    let children_player = switch_player(player);
    new_state.player = player;

    set_value(&mut new_state, &stone, player);
    new_state.stone = stone;

    generate_tree(new_state, new_depth, children_player)
}

fn minimax(node: &Node, depth: u8, maximizing_player: bool) -> i32 {
    if depth == 0 || node.heuristic == 0 {
        node.heuristic
    } else if maximizing_player == true {
        node.children.iter().fold(MIN, |value, child| {
            max(value, minimax(child, depth - 1, true))
        })
    } else {
        node.children.iter().fold(MAX, move |value, child| {
            min(value, minimax(child, depth - 1, false))
        })
    }
}

fn get_alignement_point(
    board: &Board,
    line: usize,
    col: usize,
    board_size: usize,
    player: u8,
) -> i32 {
    JOINED_ACTIONS.iter().fold(0, |mut points, actions| {
        points += fake_heuristic(board, &Stone(line, col), player, board_size, *actions);
        points
    })
}

fn get_basic_point(line: usize, col: usize, board_size: usize) -> i32 {
    let point_line = line < (board_size / 2);
    let point_col = col < (board_size / 2);

    if !point_line && !point_col {
        (board_size - line + board_size - col) as i32
    } else if !point_line {
        (board_size - line + col) as i32
    } else if !point_col {
        (board_size - col + line) as i32
    } else {
        (line + col) as i32
    }
}

fn fake_heuristic(
    board: &Board,
    stone: &Stone,
    player: u8,
    board_size: usize,
    actions: &str,
) -> i32 {
    actions
        .split('|')
        .into_iter()
        .fold(1, |mut stones, action| {
            let new_stones = get_aligned_stones(board, stone, player, board_size, action, actions);
            stones += new_stones * new_stones * new_stones * 100;
            stones
        })
}
