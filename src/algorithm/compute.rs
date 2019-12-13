use std::cmp::{max, min};
use std::i32::{MAX, MIN};
use std::time::Instant;

use crate::game::{get_empty_neighbours, switch_player};
use crate::game::{GameState, Stones};

pub fn compute(state: &GameState) -> usize {

    if state.placed.is_empty() {
        let board_size = state.board_size;
        return (board_size * board_size) / 2;
    }

    let time = Instant::now();
    let parent_node = Node::new(state, 2);
    println!("{}ms", time.elapsed().as_millis());
    println!("children: {}", parent_node.children.len());
    println!(
        "children of first children: {}",
        parent_node.children[0].children.len()
    );

    let (heuristic, index) = minimax(parent_node, 2, true);
    println!("H: {}", heuristic);

    index
}

#[derive(Debug)]
struct Node {
    placed: Stones,
    index: usize,
    children: Vec<Node>,
    heuristic: i32,
}

impl Node {
    pub fn new(state: &GameState, depth: u8) -> Node {
        generate_tree(
            state.placed.clone(),
            state.board_size,
            depth,
            state.player,
            0,
        )
    }
}

fn generate_tree(
    placed: Stones,
    board_size: usize,
    depth: u8,
    player: u8,
    played_index: usize,
) -> Node {
    let children = generate_children(&placed, board_size, depth, player);
    // TODO: compute heuristic
    let heuristic = 1;
    Node {
        index: played_index,
        placed,
        children,
        heuristic,
    }
}

fn generate_children(placed: &Stones, board_size: usize, depth: u8, player: u8) -> Vec<Node> {
    if depth == 0 {
        return vec![];
    }

    let mut children = vec![];

    let next_depth = depth - 1;
    for (index, value) in placed {
        if *value == 1 || *value == 2 {
            for neighbour in get_empty_neighbours(placed, *index, board_size) {
                children.push(create_child_node(
                    placed, board_size, next_depth, player, neighbour,
                ));
            }
        }
    }

    children
}

fn create_child_node(
    placed: &Stones,
    board_size: usize,
    depth: u8,
    player: u8,
    index: usize,
) -> Node {
    let mut new_placed = placed.clone();
    new_placed.insert(index, player);

    generate_tree(new_placed, board_size, depth, switch_player(player), index)
}

fn minimax(node: Node, depth: u8, maximizing_player: bool) -> (i32, usize) {
    if depth == 0 || node.heuristic == 0 {
        (node.heuristic, node.index)
    } else if maximizing_player == true {
        let mut best_index = 0;
        let mut max_value = MIN;

        for child in node.children {
            let (child_h, child_i) = minimax(child, depth - 1, false);
            if max_value < child_h {
                max_value = child_h;
                best_index = child_i;
            }
        }

        (max_value, best_index)
    } else {
        let mut best_index = 0;
        let mut min_value = MAX;

        for child in node.children {
            let (child_h, child_i) = minimax(child, depth - 1, true);
            if child_h < min_value {
                min_value = child_h;
                best_index = child_i;
            }
        }

        (min_value, best_index)
    }
}
