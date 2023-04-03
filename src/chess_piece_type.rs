pub use super::chess_error::ChessError;

pub enum ChessPieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

pub fn get_chess_piece_type(chess_piece_char: char) -> Result<ChessPieceType, ChessError> {
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