const ROW_LENGTH: usize = 15;
const EMPTY_CELL: char = '_';
pub use super::chess_error::ChessError;


const MAXIMUM_NUMBER_PIECES: i32 = 2;
const MAX_WHITE_PIECES_COUNT: i32 = 1;
const MAX_BLACK_PIECES_COUNT: i32 = 1;
const CHESS_PIECES: [char; 12] = ['R', 'D', 'A', 'C', 'T', 'P', 'r', 'd', 'a', 'c', 't', 'p'];
const CHESS_PIECES_WHITE:[char; 6] = ['r', 'd', 'a', 'c', 't', 'p'];
const CHESS_PIECES_BLACK:[char; 6] = ['R', 'D', 'A', 'C', 'T', 'P'];
const DESCRIPTIVE_CHESSBOARD: &str = 
"_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ D _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ t _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _";

// len("_ _ _ _ _ _ _ _") ==> 15

pub fn validate_argument_count(args: &Vec<String>) -> Result<(), ChessError>{
    if args.len() != 2 {
        return Err(ChessError::ArgsError);
    }
    Ok(())
}
pub fn validate_row_length(row: &String, i_row: usize) -> Result<(), ChessError>{
    if row.len() != ROW_LENGTH {
        // let error = format!("La fila {} no cumple con el formato establecido", i_row + 1);
        // panic_syntax_chessboard(&error);
        return Err(ChessError::InvalidSyntax);
    }
    Ok(())
}

fn count_pieces(matrix:& [char; 64]) -> i32{
    let mut count = 0;
    for square in matrix{
        for piece in CHESS_PIECES {
            if *square == piece {
                count += 1;
            }
        }
    }
    count
}

pub fn validate_number_of_pieces(matrix:& [char; 64]){
    if count_pieces(matrix) != MAXIMUM_NUMBER_PIECES {
        panic_with_format("Para simular las futuras jugadas se necesitan 2 piezas de ajedrez.\nSu archivo no cumple con ese requisito\n");
    }
}

fn validate_square(square: &char){
    if *square == EMPTY_CELL {
        return;
    }
    for piece in CHESS_PIECES{
        if *square == piece {
            return;
        }
    }
    panic_invalid_piece();
}

pub fn validate_board_pieces(matrix:& [char; 64]){
    for c in matrix.iter(){
        validate_square(c);
    }
}

fn count_matching_values_in_matrix(matrix:& [char; 64], values:& [char; 6]) -> i32{
    let mut count = 0;
    for c in matrix.iter(){
        for v in values.iter() {
            if *c == *v{
                count += 1;
                break;
            }
        }
        if count > 1 {
            break;
        }
    }
    return count;
}

pub fn validate_one_black_one_white(matrix:& [char; 64]){
    let black_pieces_count = count_matching_values_in_matrix(matrix, &CHESS_PIECES_BLACK);
    if black_pieces_count != MAX_BLACK_PIECES_COUNT {
        panic_with_format("Solo debe haber 1 pieza de cada color en el tablero");
    }
    let white_pieces_count = count_matching_values_in_matrix(matrix, &CHESS_PIECES_WHITE);
    if white_pieces_count != MAX_WHITE_PIECES_COUNT {
        panic_with_format("Solo debe haber 1 pieza de cada color en el tablero");
    }
}


fn panic_invalid_piece(){
    let error = format!("Pieza no valida encontrada.\nPiezas validas:{:?}\nCasilla vacia: ' '", CHESS_PIECES);
    panic_with_format(&error);
}

fn panic_with_format(error: &str){
    // format!()
    panic!("\nError: [{}]\n", error);
}

pub fn panic_syntax_chessboard(error: &str){
    panic!("\nError: [{}.\nGuiese del siguiente ejemplo:\n{}\nPara mas detalles: https://taller-1-fiuba-rust.github.io/proyecto/23C1/ejercicio_individual.html]", error, DESCRIPTIVE_CHESSBOARD);
}