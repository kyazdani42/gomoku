use super::game::Tile;

#[derive(Clone)]
pub struct Player {
    pub last_hits: Vec<Tile>,
    pub captured: u8,
}

impl Player {
    pub fn new() -> Player {
        Player {
            last_hits: vec![],
            captured: 0,
        }
    }

    pub fn push_hit(&mut self, tile: Tile) {
        self.last_hits.push(tile)
    }
}
