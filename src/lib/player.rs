use super::r#move::Move;

#[derive(Clone)]
pub struct Player {
    pub tiles: Vec<i32>,
    pub last_played: i32,
    pub captured: u8,
}

const MOVES: [Move; 8] = [
    Move::Left,
    Move::Right,
    Move::Top,
    Move::Bottom,
    Move::TopLeft,
    Move::TopRight,
    Move::BottomLeft,
    Move::BottomRight,
];

const STRAIGHT_MOVES: [[Move; 2]; 4] = [
    [Move::Left, Move::Right],
    [Move::Top, Move::Bottom],
    [Move::TopLeft, Move::BottomRight],
    [Move::TopRight, Move::BottomLeft],
];

impl Player {
    pub fn new() -> Player {
        Player {
            tiles: vec![],
            last_played: 0,
            captured: 0,
        }
    }

    pub fn contains(&self, tile: i32) -> bool {
        self.tiles.contains(&tile)
    }

    // TODO: there might be better way to store those
    pub fn get_alignments(&self, index: i32) -> Option<Vec<Vec<i32>>> {
        None
    }
}
