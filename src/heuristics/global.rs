use crate::game::{Stones};

use super::{get_aligned_h};

pub fn get_heuristics(
    placed: &Stones,
    index: usize,
    board_size: usize,
    player: u8,
) -> i32{
    let mut heuristic = 1;
    heuristic += get_aligned_h(placed, index, board_size, player);

    heuristic
}