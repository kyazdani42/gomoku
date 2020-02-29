#[derive(Clone)]
pub struct Player {
    pub tiles: Vec<i32>,
    pub last_played: i32,
    pub captured: u8,
}

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
