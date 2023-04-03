pub enum ChessError {
    InvalidPiece,
}

pub fn print_error_messages(error_type: ChessError){
    let error_description = match error_type {
        ChessError::InvalidPiece => "Tipo de pieza no valido",
    };
    println!("ERROR: [{}]", error_description);
}