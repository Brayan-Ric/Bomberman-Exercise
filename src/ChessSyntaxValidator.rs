const ROW_LENGTH: usize = 15;
const EMPTY_CELL: char = '_';
const MAXIMUM_NUMBER_PIECES: i32 = 2;
const CHESS_PIECES: [char; 12] = ['R', 'D', 'A', 'C', 'T', 'P', 'r', 'd', 'a', 'c', 't', 'p'];
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

pub fn validate_argument_count(args: &Vec<String>){
    if args.len() != 2 {
        panic_with_format("Use --> cargo run -- archivo.txt");
    }
}

pub fn validate_row_length(row: &String, i_row: usize){
    if row.len() != ROW_LENGTH {
        let error = format!("La fila {} no cumple con el formato establecido", i_row + 1);
        panic_syntax_chessboard(&error);
    }
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