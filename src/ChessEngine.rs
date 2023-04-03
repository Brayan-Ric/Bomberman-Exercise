pub use super::ChessSyntaxValidator;
use std::{collections::HashMap, ops::Div};
pub use super::chess_error::ChessError;
pub use super::chess_piece::ChessPiece;


const SQUARE_SEPARATOR: char = ' ';
const NUMBER_OF_COLUMNS: usize = 8;
const CHESS_PIECES: [char; 12] = ['R', 'D', 'A', 'C', 'T', 'P', 'r', 'd', 'a', 'c', 't', 'p'];

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


pub fn recreate_future_moves(matrix:&[char; 64]) -> Result<(), ChessError> {
    ChessSyntaxValidator::validate_board_pieces(matrix);
    // ChessSyntaxValidator::validate_number_of_pieces(matrix); validate_one_black_one_white hace los mismo y mejor
    ChessSyntaxValidator::validate_one_black_one_white(matrix);

    // Con validate_one_black_one_white me asegure que hay 2 piezas,
    // asi que lo siguiente no me dara error
    // let mut chess_pieces = find_chess_pieces(matrix);
    let (pieces1, pieces2) = get_chess_pieces(matrix)?;
    
    simulate_next_move(&pieces1, &pieces2);
    // Ok(true)
    Ok(())
}


// Hago unwrap porque antes valide que tenia 2 piezas en el tablero
fn get_chess_pieces(matrix:&[char; 64]) -> Result<(ChessPiece, ChessPiece), ChessError>{
    let chess_pieces = find_chess_pieces(matrix);

    let mut iter = chess_pieces.keys();

    //Creo 1era pieza
    let key = iter.next().unwrap();
    let (i, j) = chess_pieces.get(&key).unwrap();
    let piece1 = ChessPiece::chess_piece_from(*key, *i, *j)?;
    

    //Creo 2da pieza
    let key = iter.next().unwrap();
    let (i, j) = chess_pieces.get(&key).unwrap();
    let piece2 = ChessPiece::chess_piece_from(*key, *i, *j)?;

    Ok((piece1, piece2))
}


fn find_chess_pieces(matrix:&[char; 64]) -> HashMap<char, (usize, usize)> {
    let mut chess_pieces: HashMap<char, (usize, usize)> =  HashMap::new();

    let mut i = 0;
    for square in matrix{
        for piece in CHESS_PIECES {
            if *square == piece {
                let row = i.div(NUMBER_OF_COLUMNS);
                let column = i % NUMBER_OF_COLUMNS;
                chess_pieces.insert(piece, (row + 1, column + 1));
            }
        }
        i += 1;
    }
    chess_pieces
}

fn simulate_next_move(piece_1: &ChessPiece, piece_2: &ChessPiece){
    let piece_1_can_capture = piece_1.can_capture(piece_2);
    let piece_2_can_capture = piece_2.can_capture(piece_1);

    if piece_1_can_capture && piece_2_can_capture {
        println!("E");
    }else if !piece_1_can_capture && !piece_2_can_capture {
        println!("P");
    } else if piece_1_can_capture && piece_1.is_white_piece() {
        println!("B");
    } else if piece_2_can_capture && piece_2.is_white_piece() {
        println!("B");
    } else {
        println!("N");
    }
}