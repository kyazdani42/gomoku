use super::game::Tile;
use super::r#move::Move;

pub fn create_tiles_neighbours(moves: &[Box<dyn Move>]) -> Vec<Vec<Tile>> {
    let mut all_tiles_neighbours = vec![];

    for y in 0..19 {
        for x in 0..19 {
            let mut zone = vec![];
            for mov in moves {
                for i in 1..3 {
                    let next_tile = mov.get_tile_mult((y, x), i);
                    if mov.is_ok(next_tile) {
                        zone.push(next_tile.0 * 19 + next_tile.1);
                    }
                }
            }
            all_tiles_neighbours.push(zone);
        }
    }

    all_tiles_neighbours
}

pub fn create_tiles_directions(straight_moves: &[Vec<Box<dyn Move>>]) -> Vec<Vec<Vec<Vec<Tile>>>> {
    let mut all_directions_tiles = vec![];

    for y in 0..19 {
        for x in 0..19 {
            let mut zone = vec![];
            for moves in straight_moves {
                let mut dir_tiles = vec![];
                for mov in moves {
                    let mut mov_tiles = vec![];
                    for i in 1..5 {
                        let next_tile = mov.get_tile_mult((y, x), i);
                        if mov.is_ok(next_tile) {
                            mov_tiles.push(next_tile.0 * 19 + next_tile.1);
                        }
                    }
                    dir_tiles.push(mov_tiles);
                }
                zone.push(dir_tiles);
            }
            all_directions_tiles.push(zone);
        }
    }

    all_directions_tiles
}
