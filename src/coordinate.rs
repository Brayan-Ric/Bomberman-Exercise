#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
    pub max_value: u32,
}
impl Coordinate {
    pub fn new(x: u32, y: u32, max_value: u32) -> Coordinate {
        Coordinate { x, y, max_value }
    }

    pub fn down(&self) -> Option<Coordinate> {
        if self.x + 1 == self.max_value {
            return None;
        }
        Some(Coordinate::new(self.x + 1, self.y, self.max_value))
    }

    pub fn up(&self) -> Option<Coordinate> {
        if self.x == 0 {
            return None;
        }
        Some(Coordinate::new(self.x - 1, self.y, self.max_value))
    }

    pub fn right(&self) -> Option<Coordinate> {
        if self.y + 1 == self.max_value {
            return None;
        }
        Some(Coordinate::new(self.x, self.y + 1, self.max_value))
    }

    pub fn left(&self) -> Option<Coordinate> {
        if self.y == 0 {
            return None;
        }
        Some(Coordinate::new(self.x, self.y - 1, self.max_value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_down_within_bounds() {
        let coordinate = Coordinate::new(2, 3, 4);
        let result = coordinate.down();
        assert_eq!(result, Some(Coordinate::new(3, 3, 4)));
    }

    #[test]
    fn test_down_at_max_value() {
        let coordinate = Coordinate::new(3, 3, 4);
        let result = coordinate.down();
        assert_eq!(result, None);
    }

    #[test]
    fn test_up() {
        let coordinate = Coordinate::new(2, 3, 5);
        let result = coordinate.up();
        assert_eq!(result, Some(Coordinate::new(1, 3, 5)));
    }

    #[test]
    fn test_up_at_minimum_x() {
        let coordinate = Coordinate::new(0, 3, 5);
        let result = coordinate.up();
        assert_eq!(result, None);
    }

    #[test]
    fn test_right() {
        let coordinate = Coordinate::new(2, 3, 5);
        let result = coordinate.right();
        assert_eq!(result, Some(Coordinate::new(2, 4, 5)));
    }

    #[test]
    fn test_right_at_maximum_y() {
        let coordinate = Coordinate::new(2, 4, 5);
        let result = coordinate.right();
        assert_eq!(result, None);
    }
    #[test]
    fn test_left() {
        let coord = Coordinate::new(2, 3, 5);
        let result = coord.left();
        assert_eq!(result, Some(Coordinate::new(2, 2, 5)));
    }

    #[test]
    fn test_left_at_minimum_y() {
        let coord = Coordinate::new(2, 0, 5);
        let result = coord.left();
        assert_eq!(result, None);
    }

}
