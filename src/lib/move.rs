use super::game::Tile;

pub trait Move {
    fn is_ok(&self, tile: Tile) -> bool;
    fn get_next_tile(&self, tile: Tile) -> Tile;
    fn get_tile_mult(&self, tile: Tile, depth: i32) -> Tile;
}

struct Left {}
impl Move for Left {
    fn is_ok(&self, tile: Tile) -> bool {
        tile.1 > -1
    }
    fn get_next_tile(&self, tile: Tile) -> Tile {
        (tile.0, tile.1 - 1)
    }

    fn get_tile_mult(&self, tile: Tile, depth: i32) -> Tile {
        (tile.0, tile.1 - depth)
    }
}

struct Right {}
impl Move for Right {
    fn is_ok(&self, tile: Tile) -> bool {
        tile.1 < 19
    }
    fn get_next_tile(&self, tile: Tile) -> Tile {
        (tile.0, tile.1 + 1)
    }

    fn get_tile_mult(&self, tile: Tile, depth: i32) -> Tile {
        (tile.0, tile.1 + depth)
    }
}

struct Top {}
impl Move for Top {
    fn is_ok(&self, tile: Tile) -> bool {
        tile.0 > -1
    }
    fn get_next_tile(&self, tile: Tile) -> Tile {
        (tile.0 - 1, tile.1)
    }

    fn get_tile_mult(&self, tile: Tile, depth: i32) -> Tile {
        (tile.0 - depth, tile.1)
    }
}

struct Bottom {}
impl Move for Bottom {
    fn is_ok(&self, tile: Tile) -> bool {
        tile.0 < 19
    }
    fn get_next_tile(&self, tile: Tile) -> Tile {
        (tile.0 + 1, tile.1)
    }

    fn get_tile_mult(&self, tile: Tile, depth: i32) -> Tile {
        (tile.0 + depth, tile.1)
    }
}

struct TopLeft {}
impl Move for TopLeft {
    fn is_ok(&self, tile: Tile) -> bool {
        tile.0 > -1 && tile.1 > -1
    }
    fn get_next_tile(&self, tile: Tile) -> Tile {
        (tile.0 - 1, tile.1 - 1)
    }

    fn get_tile_mult(&self, tile: Tile, depth: i32) -> Tile {
        (tile.0 - depth, tile.1 - depth)
    }
}

struct TopRight {}
impl Move for TopRight {
    fn is_ok(&self, tile: Tile) -> bool {
        tile.0 > -1 && tile.1 < 19
    }
    fn get_next_tile(&self, tile: Tile) -> Tile {
        (tile.0 - 1, tile.1 + 1)
    }

    fn get_tile_mult(&self, tile: Tile, depth: i32) -> Tile {
        (tile.0 - depth, tile.1 + depth)
    }
}

struct BottomLeft {}
impl Move for BottomLeft {
    fn is_ok(&self, tile: Tile) -> bool {
        tile.0 < 19 && tile.1 > -1
    }
    fn get_next_tile(&self, tile: Tile) -> Tile {
        (tile.0 + 1, tile.1 - 1)
    }

    fn get_tile_mult(&self, tile: Tile, depth: i32) -> Tile {
        (tile.0 + depth, tile.1 - depth)
    }
}

struct BottomRight {}
impl Move for BottomRight {
    fn is_ok(&self, tile: Tile) -> bool {
        tile.0 < 19 && tile.1 < 19
    }
    fn get_next_tile(&self, tile: Tile) -> Tile {
        (tile.0 + 1, tile.1 + 1)
    }

    fn get_tile_mult(&self, tile: Tile, depth: i32) -> Tile {
        (tile.0 + depth, tile.1 + depth)
    }
}

pub struct Moves {
    pub all_moves: Vec<Box<dyn Move>>,
    pub straight_moves: Vec<Vec<Box<dyn Move>>>,
}

impl Moves {
    pub fn new() -> Self {
        Self {
            all_moves: vec![
                Box::new(Left {}),
                Box::new(Right {}),
                Box::new(Top {}),
                Box::new(Bottom {}),
                Box::new(TopLeft {}),
                Box::new(TopRight {}),
                Box::new(BottomLeft {}),
                Box::new(BottomRight {}),
            ],
            straight_moves: vec![
                vec![Box::new(Left {}), Box::new(Right {})],
                vec![Box::new(Top {}), Box::new(Bottom {})],
                vec![Box::new(TopLeft {}), Box::new(BottomRight {})],
                vec![Box::new(TopRight {}), Box::new(BottomLeft {})],
            ],
        }
    }
}
