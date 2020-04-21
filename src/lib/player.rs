#[derive(Clone)]
pub struct Player {
    pub last_hits: Vec<(i32,i32)>,
    pub captured: u8,
}

impl Player {
    pub fn new() -> Player {
        Player {
            last_hits: vec![],
            captured: 0,
        }
    }

    pub fn push_hit(&mut self, idx: (i32, i32)) {
        self.last_hits.push(idx)
    }
}
