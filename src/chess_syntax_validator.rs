const ROW_LENGTH: usize = 15;
const EMPTY_CELL: char = '_';
pub use super::chess_error::ChessError;

// const MAXIMUM_NUMBER_PIECES: i32 = 2;
const MAX_WHITE_PIECES_COUNT: i32 = 1;
const MAX_BLACK_PIECES_COUNT: i32 = 1;
const CHESS_PIECES: [char; 12] = ['R', 'D', 'A', 'C', 'T', 'P', 'r', 'd', 'a', 'c', 't', 'p'];
const CHESS_PIECES_WHITE: [char; 6] = ['r', 'd', 'a', 'c', 't', 'p'];
const CHESS_PIECES_BLACK: [char; 6] = ['R', 'D', 'A', 'C', 'T', 'P'];
const DESCRIPTIVE_CHESSBOARD: &str = "_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ D _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ t _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _";

// len("_ _ _ _ _ _ _ _") ==> 15

pub fn validate_argument_count(args: &Vec<String>) -> Result<(), ChessError> {
    if args.len() != 2 {
        return Err(ChessError::ArgsError);
    }
    Ok(())
}
pub fn validate_row_length(row: &String) -> Result<(), ChessError> {
    if row.len() != ROW_LENGTH {
        return Err(ChessError::InvalidSyntax);
    }
    Ok(())
}

fn validate_square(square: &char) -> Result<(), ChessError> {
    if *square == EMPTY_CELL {
        return Ok(());
    }
    for piece in CHESS_PIECES {
        if *square == piece {
            return Ok(());
        }
    }
    Err(ChessError::ChessInvalidCharError)
}

pub fn validate_board_pieces(matrix: &[char; 64]) -> Result<(), ChessError> {
    for c in matrix.iter() {
        match validate_square(c) {
            Ok(()) => continue,
            Err(error_type) => return Err(error_type),
        }
    }
    Ok(())
}

fn count_matching_values_in_matrix(matrix: &[char; 64], values: &[char; 6]) -> i32 {
    let mut count = 0;
    for c in matrix.iter() {
        for v in values.iter() {
            if *c == *v {
                count += 1;
                break;
            }
        }
        if count > 1 {
            break;
        }
    }
    count
}

pub fn validate_one_black_one_white(matrix: &[char; 64]) -> Result<(), ChessError> {
    let black_pieces_count = count_matching_values_in_matrix(matrix, &CHESS_PIECES_BLACK);
    if black_pieces_count != MAX_BLACK_PIECES_COUNT {
        return Err(ChessError::InvalidPieceCount);
    }
    let white_pieces_count = count_matching_values_in_matrix(matrix, &CHESS_PIECES_WHITE);
    if white_pieces_count != MAX_WHITE_PIECES_COUNT {
        return Err(ChessError::InvalidPieceCount);
    }
    Ok(())
}

pub fn panic_syntax_chessboard(error: &str) {
    panic!("\nError: [{}.\nGuiese del siguiente ejemplo:\n{}\nPara mas detalles: https://taller-1-fiuba-rust.github.io/proyecto/23C1/ejercicio_individual.html]", error, DESCRIPTIVE_CHESSBOARD);
}


#[cfg(test)]
pub mod test {
    use super::*;
    
    #[test]
    fn test_validate_argument_count_ok(){
        let args = vec!["value_1".to_string(), "value_2".to_string()];
        let result = validate_argument_count(&args);
        assert!(match result {
            Ok(()) => true,
            _ => false,
        });
    }

    #[test]
    fn test_validate_argument_count_error(){
        let args = vec!["value_1".to_string(), "value_2".to_string(), "value_3".to_string()];
        let result = validate_argument_count(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_row_length_ok(){
        let valid_row_format = "_ _ _ _ _ _ _ _".to_string();
        let result = validate_row_length(&valid_row_format);
        assert!(match result {
            Ok(()) => true,
            _ => false,
        });
    }
    
    #[test]
    fn test_validate_row_length_error(){
        let invalid_row_format = "_ _ _ _".to_string();
        let result = validate_row_length(&invalid_row_format);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_square_ok(){
        let valid_piece = 'P';
        let result = validate_square(&valid_piece);
        assert!(match result {
            Ok(()) => true,
            _ => false,
        });
    }

    #[test]
    fn test_validate_square_error(){
        let invalid_piece = '*';
        let result = validate_square(&invalid_piece);
        assert!(result.is_err());
    }
    
}
