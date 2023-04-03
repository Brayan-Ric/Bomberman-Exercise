use std::usize;


pub struct ChessPosition {
    row: usize,
    column: usize,
}

impl ChessPosition {
    pub fn create_position(row: usize, column: usize) -> Self{
        ChessPosition { row, column}
    }

    pub fn is_adjacent(&self, another_position: &ChessPosition) -> bool{
        let dx = (self.row as i32 - another_position.row as i32).abs();
        let dy = (self.column as i32 - another_position.column as i32).abs();

        dx <= 1 && dy <= 1 && !(dx == 0 && dy == 0)
    }

    pub fn are_aligned(&self, another_position: &ChessPosition) -> bool{
        self.are_in_same_row_or_column(another_position) || self.are_in_diagonal(another_position)
    }

    pub fn are_in_same_row_or_column(&self, another_position: &ChessPosition) -> bool{
        self.row == another_position.row || self.column == another_position.column
    }

    pub fn are_in_diagonal(&self, another_position: &ChessPosition) -> bool{
        let dx = ((self.row as i32) - (another_position.row as i32)).abs();
        let dy = ((self.column as i32) - (another_position.column as i32)).abs();
        dx == dy
    }

    pub fn are_in_l(&self, another_position: &ChessPosition) -> bool{
        let dx = (self.row as i32 - another_position.row as i32).abs();
        let dy = (self.column as i32 - another_position.column as i32).abs();
        (dx == 1 && dy == 2) || (dx == 2 && dy == 1)
    }

    pub fn are_diagonal_1(&self, another_position: &ChessPosition) -> bool{
        let dx = (self.row as i32 - another_position.row as i32).abs();
        let dy = (self.column as i32 - another_position.column as i32).abs();
        dx == 1 && dy == 1
    }

    pub fn one_position_down_diagonal(&self, another_position: &ChessPosition) -> bool{
        let dx = (self.row as i32) - (another_position.row as i32);
        let dy = (self.column as i32) - (another_position.column as i32);
        dx.abs() == 1 && dy.abs() == 1 && dy > 0
    }

    pub fn one_position_upward_diagonal(&self, another_position: &ChessPosition) -> bool{
        let dx = (self.row as i32) - (another_position.row as i32);
        let dy = (self.column as i32) - (another_position.column as i32);
        dx.abs() == 1 && dy.abs() == 1 && dy < 0
    }
    
}