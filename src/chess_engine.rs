pub use super::chess_board_loader;
pub use super::chess_error;
pub use super::chess_error::ChessError;
pub use super::chess_piece::ChessPiece;
pub use super::chess_syntax_validator;
use std::{collections::HashMap, ops::Div};

const SQUARE_SEPARATOR: char = ' ';
const NUMBER_OF_COLUMNS: usize = 8;
const CHESS_PIECES: [char; 12] = ['R', 'D', 'A', 'C', 'T', 'P', 'r', 'd', 'a', 'c', 't', 'p'];

pub fn complete_row(row: &str, i_row: usize, matrix: &mut [char; 64]) {
    let mut pos = i_row * NUMBER_OF_COLUMNS;
    for data in row.split(SQUARE_SEPARATOR) {
        matrix[pos] = data.chars().next().unwrap();
        pos += 1;
    }
}

/// Process Row
/// Precondiciones: La matriz debe ser de tamaño 8x8
/// Arg:
///     Recibe el numero de fila, numero de casilla y una matriz 8x8 tipo char
/// Return:
///     Si no se cumplen las precondiciones se devolvera un error de tipo ChessError
///     Si todo sale bien se completara la matriz con los datos recibidos
pub fn process_row(row: &String, i_row: usize, matrix: &mut [char; 64]) -> Result<(), ChessError> {
    match chess_syntax_validator::validate_row_length(row) {
        Ok(()) => complete_row(row, i_row, matrix),
        Err(type_error) => return Err(type_error),
    };
    // complete_row(row, i_row, matrix);
    Ok(())
}

/// Recreate Future Moves
/// Recrea las proximas capturas
/// Precondicones:
///     La matriz tiene que tener 2 id de piezas de ajedrez, una blanca y una negra
///     Los valores de la matriz deben ser id de piezas o id de casilla vacia
/// Arg:
///     Recibe una matriz 8x8, tipo char
/// Return:
///     Si no se cumplen las precondiones devuelve un error de tipo ChessError
///     Si se cumplen las precondiciones, devolvera el id de las posibles capturas
pub fn recreate_future_moves(matrix: &[char; 64]) -> Result<char, ChessError> {
    match chess_syntax_validator::validate_board_pieces(matrix) {
        Ok(()) => (),
        Err(error_type) => return Err(error_type),
    };

    match chess_syntax_validator::validate_one_black_one_white(matrix) {
        Ok(()) => (),
        Err(error_type) => return Err(error_type),
    }

    // Con validate_one_black_one_white me asegure que hay 2 piezas,
    // asi que lo siguiente no me dara error
    let (pieces1, pieces2) = get_chess_pieces(matrix)?;

    Ok(simulate_next_move(&pieces1, &pieces2))
}

/// Get Chess Pieces
/// Precondicion:
///     Se tuvo que validar que la matriz tenga 2 id de piezas validos.
///     Si tiene mas solo se devolvera las 2 1eras.
/// Arg:
///     Recibe una matriz de 8x8 tipo char
/// Return:
///     Devuelve 2 ChessPiece encontradas en la matriz
///     Si no se cumplio las precondiciones, devolvera un error del tipo ChessError
///     
fn get_chess_pieces(matrix: &[char; 64]) -> Result<(ChessPiece, ChessPiece), ChessError> {
    let chess_pieces = find_chess_pieces(matrix);

    let mut iter = chess_pieces.keys();

    //Creo 1era pieza
    let key = iter.next().unwrap();
    let (i, j) = chess_pieces.get(key).unwrap();
    let piece1 = ChessPiece::chess_piece_from(*key, *i, *j)?;

    //Creo 2da pieza
    let key = iter.next().unwrap();
    let (i, j) = chess_pieces.get(key).unwrap();
    let piece2 = ChessPiece::chess_piece_from(*key, *i, *j)?;

    Ok((piece1, piece2))
}

/// Find Chess Pieces
/// Arg:
///     Recibe una matriz de 8x8 tipo char
/// Return:
///     Devuelve un diccionario del tipo:
///         Clave: Id de la pieza
///         Valor: (row, colum), donde row y column es la posicion de la pieza en la matriz
fn find_chess_pieces(matrix: &[char; 64]) -> HashMap<char, (usize, usize)> {
    let mut chess_pieces: HashMap<char, (usize, usize)> = HashMap::new();

    for (i, square) in matrix.iter().enumerate() {
        for piece in CHESS_PIECES {
            if *square == piece {
                let row = i.div(NUMBER_OF_COLUMNS);
                let column = i % NUMBER_OF_COLUMNS;
                chess_pieces.insert(piece, (row + 1, column + 1));
            }
        }
    }
    chess_pieces
}

/// Simulate Next Move
/// Arg:
///     Recibe 2 piezas ChessPiece
/// Return:
///     Devuelve un char, especificando las posibles capturas
///     'N': Negras ganan
///     'B': Blancas ganan
///     'E': Empate
///     'P': Nadie gana
fn simulate_next_move(piece_1: &ChessPiece, piece_2: &ChessPiece) -> char {
    let piece_1_can_capture = piece_1.can_capture(piece_2);
    let piece_2_can_capture = piece_2.can_capture(piece_1);

    if piece_1_can_capture && piece_2_can_capture {
        'E'
    } else if !piece_1_can_capture && !piece_2_can_capture {
        'P'
    } else if (piece_1_can_capture && piece_1.is_white_piece())
        || (piece_2_can_capture && piece_2.is_white_piece())
    {
        'B'
    } else {
        'N'
    }
}

/// Game
/// Arg:
///     file_name: Path del tablero a leer
/// Return:
///     Si no hay errores se devuelve un char que simboliza las posibles capturas del juego
///     Si hubo un error devuelve un enum ChessError especifico por el error causado
pub fn game(file_name: &str) -> Result<char, ChessError> {
    let file = chess_board_loader::open_file(file_name)?;

    let mut matrix = ['-'; 64];

    match chess_board_loader::read_file(&file, &mut matrix) {
        Ok(()) => (),
        Err(error_type) => return Err(error_type),
    };

    recreate_future_moves(&matrix)
}
