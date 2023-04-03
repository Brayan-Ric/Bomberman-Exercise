pub use super::chess_error::ChessError;
pub use super::chess_piece_type;
pub use super::chess_piece_type::ChessPieceType;
pub use super::chess_position::ChessPosition;
enum ChessPieceColor {
    White,
    Black,
}

fn get_chess_piece_color(chess_piece_char: char) -> Result<ChessPieceColor, ChessError> {
    match chess_piece_char {
        'R' => Ok(ChessPieceColor::Black),
        'D' => Ok(ChessPieceColor::Black),
        'C' => Ok(ChessPieceColor::Black),
        'A' => Ok(ChessPieceColor::Black),
        'T' => Ok(ChessPieceColor::Black),
        'P' => Ok(ChessPieceColor::Black),
        'r' => Ok(ChessPieceColor::White),
        'd' => Ok(ChessPieceColor::White),
        'c' => Ok(ChessPieceColor::White),
        'a' => Ok(ChessPieceColor::White),
        't' => Ok(ChessPieceColor::White),
        'p' => Ok(ChessPieceColor::White),
        _ => Err(ChessError::InvalidPiece),
    }
}

pub struct ChessPiece {
    piece_type: ChessPieceType,
    position: ChessPosition,
    color: ChessPieceColor,
}

impl ChessPiece {
    pub fn chess_piece_from(
        chess_piece_char: char,
        row: usize,
        column: usize,
    ) -> Result<ChessPiece, ChessError> {
        let piece_type = chess_piece_type::get_chess_piece_type(chess_piece_char)?;
        let position = ChessPosition::create_position(row, column);
        let color = get_chess_piece_color(chess_piece_char)?;
        Ok(ChessPiece {
            piece_type,
            position,
            color,
        })
    }

    pub fn is_black_piece(&self) -> bool {
        match self.color {
            ChessPieceColor::Black => true,
            ChessPieceColor::White => false,
        }
    }

    pub fn is_white_piece(&self) -> bool {
        match self.color {
            ChessPieceColor::White => true,
            ChessPieceColor::Black => false,
        }
    }

    pub fn can_capture(&self, another_piece: &ChessPiece) -> bool {
        self.capture(another_piece)
    }

    fn capture(&self, another_piece: &ChessPiece) -> bool {
        match self.piece_type {
            ChessPieceType::King => self.king_capture(another_piece),
            ChessPieceType::Queen => self.queen_capture(another_piece),
            ChessPieceType::Rook => self.rook_capture(another_piece),
            ChessPieceType::Bishop => self.bishop_capture(another_piece),
            ChessPieceType::Knight => self.knight_capture(another_piece),
            ChessPieceType::Pawn => self.pawn_capture(another_piece),
        }
    }

    fn king_capture(&self, another_piece: &ChessPiece) -> bool {
        self.position.is_adjacent(&another_piece.position)
    }

    fn queen_capture(&self, another_piece: &ChessPiece) -> bool {
        self.position.are_aligned(&another_piece.position)
    }

    fn rook_capture(&self, another_piece: &ChessPiece) -> bool {
        self.position
            .are_in_same_row_or_column(&another_piece.position)
    }

    fn bishop_capture(&self, another_piece: &ChessPiece) -> bool {
        self.position.are_in_diagonal(&another_piece.position)
    }

    fn knight_capture(&self, another_piece: &ChessPiece) -> bool {
        self.position.are_in_l(&another_piece.position)
    }

    fn pawn_capture(&self, another_piece: &ChessPiece) -> bool {
        self.position.are_diagonal_1(&another_piece.position)
    }
}
