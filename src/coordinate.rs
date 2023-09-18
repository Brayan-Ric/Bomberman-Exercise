#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}
impl Coordinate {
    pub fn new(x: u32, y: u32) -> Coordinate {
        Coordinate { x, y }
    }

    // pub fn blast_area(&self, game: &mut Game, range: u32) {
    //     self.explode_left(game, range);
    //     self.explode_right(game, range);
    //     self.explode_up(game, range);
    //     self.explode_down(game, range);
    // }

    pub fn right(&self) -> Coordinate {
        Coordinate::new(self.x + 1, self.y)
    }

    pub fn left(&self) -> Coordinate {
        Coordinate::new(self.x - 1, self.y)
    }

    pub fn up(&self) -> Coordinate {
        Coordinate::new(self.x, self.y + 1)
    }

    pub fn down(&self) -> Coordinate {
        Coordinate::new(self.x, self.y - 1)
    }

    // pub fn explode_left(&self, game: &mut Game, range: u32) {
    //     if range == 0 {
    //         return;
    //     }
    //     //Hago algo
    //     game.normal_bomb_effect(& Coordinate::new(self.x - 1, self.y), range, explode_left);
    //     // Coordinate::new(self.x - 1, self.y).explode_left(range - 1);
    // }

    // fn explode_right(&self, game: &mut Game, range: u32) {
    //     if range == 0 {
    //         return;
    //     }
    //     //Hago algo

    //     Coordinate::new(self.x + 1, self.y).explode_right(range - 1);
    // }

    // fn explode_up(&self, game: &mut Game, range: u32) {
    //     if range == 0 {
    //         return;
    //     }
    //     //Hago algo

    //     Coordinate::new(self.x, self.y + 1).explode_up(range - 1);
    // }

    // fn explode_down(&self, game: &mut Game, range: u32) {
    //     if range == 0 {
    //         return;
    //     }
    //     //Hago algo

    //     Coordinate::new(self.x, self.y - 1).explode_down(range - 1);
    // }
}

// fn explode_left_2(c: &Coordinate, game: &mut Game, range: u32) {}
