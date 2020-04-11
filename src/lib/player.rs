use std::collections::HashSet;

#[derive(Clone)]
pub struct Player {
    pub tiles: HashSet<i32>,
    pub last_played: i32,
    pub captured: u8,
}

impl Player {
    pub fn new() -> Player {
        Player {
            tiles: HashSet::new(),
            last_played: 0,
            captured: 0,
        }
    }

    pub fn contains(&self, tile: i32) -> bool {
        self.tiles.contains(&tile)
    }

    pub fn insert(&mut self, tile: i32) {
        self.tiles.insert(tile);
    }

    pub fn insert_mult(&mut self, tiles: &Vec<i32>) {
        tiles.iter().for_each(|tile| {
            self.insert(*tile);
        });
    }

    pub fn remove(&mut self, tile: i32) {
        self.tiles.remove(&tile);
    }

    pub fn remove_mult(&mut self, tiles: &Vec<i32>) {
        tiles.iter().for_each(|tile| {
            self.remove(*tile);
        });
    }
}
