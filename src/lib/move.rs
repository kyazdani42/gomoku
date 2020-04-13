use super::game::Tile;

pub trait Move {
    fn is_ok(&self, tile: &Tile) -> bool;
    fn get_next_tile(&self, tile: &Tile) -> Tile;
    fn get_tile_mult(&self, tile: &Tile, depth: i32) -> Tile;
}

struct Left {}
impl Move for Left {
    fn is_ok(&self, tile: &Tile) -> bool {
        tile.1 > -1
    }
    fn get_next_tile(&self, tile: &Tile) -> Tile {
        (tile.0, tile.1 - 1)
    }

    fn get_tile_mult(&self, tile: &Tile, depth: i32) -> Tile {
        (tile.0, tile.1 - depth)
    }
}

struct Right {
    board_size: i32,
}
impl Move for Right {
    fn is_ok(&self, tile: &Tile) -> bool {
        tile.1 < self.board_size
    }
    fn get_next_tile(&self, tile: &Tile) -> Tile {
        (tile.0, tile.1 + 1)
    }

    fn get_tile_mult(&self, tile: &Tile, depth: i32) -> Tile {
        (tile.0, tile.1 + depth)
    }
}

struct Top {}
impl Move for Top {
    fn is_ok(&self, tile: &Tile) -> bool {
        tile.0 > -1
    }
    fn get_next_tile(&self, tile: &Tile) -> Tile {
        (tile.0 - 1, tile.1)
    }

    fn get_tile_mult(&self, tile: &Tile, depth: i32) -> Tile {
        (tile.0 - depth, tile.1)
    }
}

struct Bottom {
    board_size: i32,
}
impl Move for Bottom {
    fn is_ok(&self, tile: &Tile) -> bool {
        tile.0 < self.board_size
    }
    fn get_next_tile(&self, tile: &Tile) -> Tile {
        (tile.0 + 1, tile.1)
    }

    fn get_tile_mult(&self, tile: &Tile, depth: i32) -> Tile {
        (tile.0 + depth, tile.1)
    }
}

struct TopLeft {}
impl Move for TopLeft {
    fn is_ok(&self, tile: &Tile) -> bool {
        tile.0 > -1 && tile.1 > -1
    }
    fn get_next_tile(&self, tile: &Tile) -> Tile {
        (tile.0 - 1, tile.1 - 1)
    }

    fn get_tile_mult(&self, tile: &Tile, depth: i32) -> Tile {
        (tile.0 - depth, tile.1 - depth)
    }
}

struct TopRight {
    board_size: i32,
}
impl Move for TopRight {
    fn is_ok(&self, tile: &Tile) -> bool {
        tile.0 > -1 && tile.1 < self.board_size
    }
    fn get_next_tile(&self, tile: &Tile) -> Tile {
        (tile.0 - 1, tile.1 + 1)
    }

    fn get_tile_mult(&self, tile: &Tile, depth: i32) -> Tile {
        (tile.0 - depth, tile.1 + depth)
    }
}

struct BottomLeft {
    board_size: i32,
}
impl Move for BottomLeft {
    fn is_ok(&self, tile: &Tile) -> bool {
        tile.0 < self.board_size && tile.1 > -1
    }
    fn get_next_tile(&self, tile: &Tile) -> Tile {
        (tile.0 + 1, tile.1 - 1)
    }

    fn get_tile_mult(&self, tile: &Tile, depth: i32) -> Tile {
        (tile.0 + depth, tile.1 - depth)
    }
}

struct BottomRight {
    board_size: i32,
}
impl Move for BottomRight {
    fn is_ok(&self, tile: &Tile) -> bool {
        tile.0 < self.board_size && tile.1 < self.board_size
    }
    fn get_next_tile(&self, tile: &Tile) -> Tile {
        (tile.0 + 1, tile.1 + 1)
    }

    fn get_tile_mult(&self, tile: &Tile, depth: i32) -> Tile {
        (tile.0 + depth, tile.1 + depth)
    }
}

pub struct Moves {
    pub all_moves: Vec<Box<dyn Move>>,
    pub straight_moves: Vec<Vec<Box<dyn Move>>>,
}

impl Moves {
    pub fn new(board_size: i32) -> Self {
        Self {
            all_moves: vec![
                Box::new(Left {}),
                Box::new(Right { board_size }),
                Box::new(Top {}),
                Box::new(Bottom { board_size }),
                Box::new(TopLeft {}),
                Box::new(TopRight { board_size }),
                Box::new(BottomLeft { board_size }),
                Box::new(BottomRight { board_size }),
            ],
            straight_moves: vec![
                vec![Box::new(Left {}), Box::new(Right { board_size })],
                vec![Box::new(Top {}), Box::new(Bottom { board_size })],
                vec![Box::new(TopLeft {}), Box::new(BottomRight { board_size })],
                vec![
                    Box::new(TopRight { board_size }),
                    Box::new(BottomLeft { board_size }),
                ],
            ],
        }
    }
}
