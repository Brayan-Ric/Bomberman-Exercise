pub enum ChessError {
    ArgsError,
    FileOpenError,
    FileReadError,
    InvalidSyntax,
    InvalidPiece,
}

pub fn print_error_messages(error_type: ChessError){
    let error_description = match error_type {
        ChessError::ArgsError => "Use --> cargo run -- archivo.txt",
        ChessError::FileOpenError => "Archivo no encontrado",
        ChessError::FileReadError => "No se pudo leer el archivo",
        ChessError::InvalidSyntax => "Las lineas leidas no cumples con el formato pedido",
        ChessError::InvalidPiece => "Se encontro un tipo de pieza no valida",
    };
    println!("ERROR: [{}]", error_description);
}