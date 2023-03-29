pub use super::ChessSyntaxValidator;


const SQUARE_SEPARATOR: char = ' ';
const NUMBER_OF_COLUMNS: usize = 8;

pub fn complete_row<'a>(row: &'a String, i_row: usize, matrix:&mut [char; 64]){
    let mut pos = i_row * NUMBER_OF_COLUMNS;
    for data in row.split(SQUARE_SEPARATOR){
        matrix[pos] = data.chars().next().unwrap();
        pos += 1;
    }
}


pub fn process_row<'a>(row: &'a String, i_row: usize, matrix:&mut [char; 64]){
    ChessSyntaxValidator::validate_row_length(&row, i_row);
    complete_row(row, i_row, matrix);
}

pub fn recreate_future_moves(matrix:&[char; 64]){
    ChessSyntaxValidator::validate_board_pieces(matrix);
    ChessSyntaxValidator::validate_number_of_pieces(matrix);
}