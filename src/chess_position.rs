use std::usize;

pub struct ChessPosition {
    row: usize,
    column: usize,
}

impl ChessPosition {
    ///Crea una posicon del ajedrez
    /// Las posicones se cuentan desde 1 al 8
    pub fn create_position(row: usize, column: usize) -> Self {
        ChessPosition { row, column }
    }

    ///Devuelve true si another_position es su adjacente
    pub fn is_adjacent(&self, another_position: &ChessPosition) -> bool {
        let dx = (self.row as i32 - another_position.row as i32).abs();
        let dy = (self.column as i32 - another_position.column as i32).abs();

        dx <= 1 && dy <= 1 && !(dx == 0 && dy == 0)
    }

    ///Devuelve true si another_position en su misma columna o fila o diagonal
    pub fn are_aligned(&self, another_position: &ChessPosition) -> bool {
        self.are_in_same_row_or_column(another_position) || self.are_in_diagonal(another_position)
    }

    ///Devuelve true si another_position en su misma columna o fila
    pub fn are_in_same_row_or_column(&self, another_position: &ChessPosition) -> bool {
        self.row == another_position.row || self.column == another_position.column
    }

    ///Devuelve true si another_position esta en su misma diagonal
    pub fn are_in_diagonal(&self, another_position: &ChessPosition) -> bool {
        let dx = ((self.row as i32) - (another_position.row as i32)).abs();
        let dy = ((self.column as i32) - (another_position.column as i32)).abs();
        dx == dy
    }

    ///Devuelve true si another_position esta a una distancia L(movimiento del caballo, ajedrez)
    pub fn are_in_l(&self, another_position: &ChessPosition) -> bool {
        let dx = (self.row as i32 - another_position.row as i32).abs();
        let dy = (self.column as i32 - another_position.column as i32).abs();
        (dx == 1 && dy == 2) || (dx == 2 && dy == 1)
    }

    ///Devuelve true si another_position esta en su misma diagonal, pero a una distancia
    pub fn are_diagonal_1(&self, another_position: &ChessPosition) -> bool {
        let dx = (self.row as i32 - another_position.row as i32).abs();
        let dy = (self.column as i32 - another_position.column as i32).abs();
        dx == 1 && dy == 1
    }

    ///Devuelve true si another_position esta en su misma diagonal inferior, pero a una distancia
    pub fn one_position_down_diagonal(&self, another_position: &ChessPosition) -> bool {
        let dx = (self.row as i32) - (another_position.row as i32);
        let dy = (self.column as i32) - (another_position.column as i32);
        dx == -1 && dy.abs() == 1
    }

    ///Devuelve true si another_position esta en su misma diagonal superior, pero a una distancia
    pub fn one_position_upward_diagonal(&self, another_position: &ChessPosition) -> bool {
        let dx = (self.row as i32) - (another_position.row as i32);
        let dy = (self.column as i32) - (another_position.column as i32);
        dx == 1 && dy.abs() == 1
    }
}
