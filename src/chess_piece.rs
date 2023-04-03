pub use super::chess_error::ChessError;


enum ChessPieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

// pub enum ChessPieceError {
//     InvalidPiece,
// }

pub struct ChessPosition {
    row: usize,
    column: usize,
}

impl ChessPosition {

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
        let dx = ((self.row - another_position.row) as i32).abs();
        let dy = ((self.column - another_position.column) as i32).abs();
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

fn get_chess_piece_type(chess_piece_char: char) -> Result<ChessPieceType, ChessError> {
    match chess_piece_char {
        'R' => Ok(ChessPieceType::King),
        'r' => Ok(ChessPieceType::King),
        'D' => Ok(ChessPieceType::Queen),
        'd' => Ok(ChessPieceType::Queen),
        'C' => Ok(ChessPieceType::Knight),
        'c' => Ok(ChessPieceType::Knight),
        'A' => Ok(ChessPieceType::Bishop),
        'a' => Ok(ChessPieceType::Bishop),
        'T' => Ok(ChessPieceType::Rook),
        't' => Ok(ChessPieceType::Rook),
        'P' => Ok(ChessPieceType::Pawn),
        'p' => Ok(ChessPieceType::Pawn),
        _ => Err(ChessError::InvalidPiece),
    }
}

pub struct ChessPiece {
    piece_type: ChessPieceType,
    position: ChessPosition,
}



impl ChessPiece {
    pub fn chess_piece_from(chess_piece_char: char, row: usize, column: usize) ->  Result<ChessPiece, ChessError>{
        let piece_type = get_chess_piece_type(chess_piece_char)?;
        let position = ChessPosition { row, column };
        Ok(ChessPiece { piece_type, position })
    }

    pub fn can_capture(&self, another_piece: &ChessPiece) -> bool{
        self.capture(&another_piece)
    }

    fn capture(&self, another_piece: &ChessPiece) -> bool{
        match self.piece_type {
            ChessPieceType::King => self.king_capture(another_piece),
            ChessPieceType::Queen => self.queen_capture(another_piece),
            ChessPieceType::Rook => self.rook_capture(another_piece),
            ChessPieceType::Bishop => self.bishop_capture(another_piece),
            ChessPieceType::Knight => self.knight_capture(another_piece),
            ChessPieceType::Pawn => self.pawn_capture(another_piece),
        }
    }

    fn king_capture(&self, another_piece: &ChessPiece) -> bool{
        self.position.is_adjacent(&another_piece.position)
    }

    fn queen_capture(&self, another_piece: &ChessPiece) -> bool{
        self.position.are_aligned(&another_piece.position)
    }

    fn rook_capture(&self, another_piece: &ChessPiece) -> bool{
        self.position.are_in_same_row_or_column(&another_piece.position)
    }
    
    fn bishop_capture(&self, another_piece: &ChessPiece) -> bool{
        self.position.are_in_diagonal(&another_piece.position)
    }

    fn knight_capture(&self, another_piece: &ChessPiece) -> bool{
        self.position.are_in_l(&another_piece.position)
    }

    fn pawn_capture(&self, another_piece: &ChessPiece) -> bool{
        self.position.are_diagonal_1(&another_piece.position)
    }
}

