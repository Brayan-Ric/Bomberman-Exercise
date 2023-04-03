pub enum ChessError {
    ArgsError,
    FileOpenError,
    InvalidPiece,
}

pub fn print_error_messages(error_type: ChessError){
    let error_description = match error_type {
        ChessError::InvalidPiece => "Se encontro un tipo de pieza no valida",
        ChessError::ArgsError => "Use --> cargo run -- archivo.txt",
        ChessError::FileOpenError => "Archivo no encontrado",
    };
    println!("ERROR: [{}]", error_description);
}