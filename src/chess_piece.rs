pub use super::chess_error::ChessError;


enum ChessPieceType {
    King,
    Queen,
    Knight,
    Pawn,
    Bishop,
    Rook,
}

// pub enum ChessPieceError {
//     InvalidPiece,
// }

struct ChessPosition {
    row: usize,
    column: usize,
}

fn get_chess_piece_type(chess_piece_char: char) -> Result<ChessPieceType, ChessError> {
    match chess_piece_char {
        'R' => Ok(ChessPieceType::King),
        'D' => Ok(ChessPieceType::Queen),
        'C' => Ok(ChessPieceType::Knight),
        'A' => Ok(ChessPieceType::Bishop),
        'T' => Ok(ChessPieceType::Rook),
        'P' => Ok(ChessPieceType::Pawn),
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

}
