pub use super::chess_engine;
pub use super::chess_error::ChessError;
pub use super::chess_syntax_validator;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn command_reader() -> Result<String, ChessError> {
    let mut args: Vec<String> = env::args().collect();
    match chess_syntax_validator::validate_argument_count(&args) {
        Ok(_) => (),
        Err(error) => return Err(error),
    }
    Ok(args.pop().unwrap())
}

pub fn open_file(file_name: &str) -> Result<File, ChessError> {
    match File::open(file_name) {
        Ok(file) => Ok(file),
        _error => Err(ChessError::FileOpenError),
    }
}

pub fn read_file(file: &File, matrix: &mut [char; 64]) -> Result<(), ChessError> {
    let buf_reader = BufReader::new(file);

    for (i, linea) in buf_reader.lines().enumerate() {
        match linea {
            Ok(linea) => chess_engine::process_row(&linea, i, matrix)?,
            Err(..) => return Err(ChessError::FileReadError),
        }
    }
    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test_open_file_error(){
        let result = open_file("ArchivoNoExiste.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_open_file_ok(){
        let result = open_file("./TablerosDePrueba/TableroExistente.txt");
        assert!(match result {
        Ok(_) => true,
        _ => false,
        });
    }
}
