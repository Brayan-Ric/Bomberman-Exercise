#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
    pub max_value: u32,
    // pub max_y: u32,
}
impl Coordinate {
    pub fn new(x: u32, y: u32, max_value: u32) -> Coordinate {
        Coordinate { x, y, max_value }
    }

    // pub fn blast_area(&self, game: &mut Game, range: u32) {
    //     self.explode_left(game, range);
    //     self.explode_right(game, range);
    //     self.explode_up(game, range);
    //     self.explode_down(game, range);
    // }

    pub fn right(&self) -> Option<Coordinate> {
        if self.x + 1 == self.max_value {
            return None;
        }
        Some(Coordinate::new(self.x + 1, self.y, self.max_value))
    }

    pub fn left(&self) -> Option<Coordinate> {
        if self.x == 0 {
            return None;
        }
        Some(Coordinate::new(self.x - 1, self.y, self.max_value))
    }

    pub fn up(&self) -> Option<Coordinate> {
        if self.y + 1 == self.max_value {
            return None;
        }
        Some(Coordinate::new(self.x, self.y + 1, self.max_value))
    }

    pub fn down(&self) -> Option<Coordinate> {
        if self.y == 0 {
            return None;
        }
        Some(Coordinate::new(self.x, self.y - 1, self.max_value))
    }
}
