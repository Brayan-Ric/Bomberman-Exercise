use std::fs::File;
use std::env;
use std::io::BufReader;
use std::io::prelude::*;
pub use super::ChessSyntaxValidator;
pub use super::ChessEngine;
pub use super::chess_error::ChessError;



pub fn command_reader() -> Result<String, ChessError>{
    let mut args: Vec<String> = env::args().collect();
    match ChessSyntaxValidator::validate_argument_count(&args)  {
        Ok(_) => (),
        Err(error) => return Err(error),
    }    
    return Ok(args.pop().unwrap());
}

pub fn open_file(file_name: &str) -> File{
    match File::open(file_name) {
        Ok(file) => file,
        _error => panic!("\n[Error: Archivo no encontrado]\n") 
    }
}

pub fn read_file(file: &File, matrix:&mut [char; 64]){
    let buf_reader = BufReader::new(file);
    println!("Contenido del Archivo");

    for (i, linea) in buf_reader.lines().enumerate() {
         match linea {
             Ok(linea) => ChessEngine::process_row(&linea, i, matrix),
             Err(..) => panic!( "LALAL"),
        }
    }
}
