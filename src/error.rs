use std::{fs::File, io::Write};

#[derive(Debug, PartialEq)]
pub enum BombermanError {
    InvalidCoordinate,
    InsufficientInput,
    InputPathError,
    InvalidItem,
    InvalidItemFormat,
    InvalidEnemyFormat,
    InvalidNormalBombFormat,
    InvalidTransferBombFormat,
    InvalidDeflectionFormat,
    InvalidBombCoordinate,
    OutputPathError,
    Write,
    NonSquareBoardError,
}

impl BombermanError {
    pub fn message(&self) -> &str {
        match self {
            BombermanError::InvalidCoordinate => {
                "Las coordenadas deben ser enteros de 0 hasta n(#filas de la cantidad de la matriz)"
            }
            BombermanError::InsufficientInput => {
                "Use: cargo new -- tablero.txt path/output.txt x y"
            }
            BombermanError::InputPathError => {
                "El archivo input no exite"
            }
            BombermanError::InvalidItem => {
                "Item no reconocido"
            }
            BombermanError::InvalidItemFormat => {
                "No se reconoce el formato del item"
            }
            BombermanError::InvalidEnemyFormat => {
                "No se cumple el formato del item enemy, use: EXXX con XXX igual a un numero natural"
            }
            BombermanError::InvalidNormalBombFormat => {
                "No se cumple el formato del item Bomba Normal, use: BXXX con XXX igual a un numero natural"
            }
            BombermanError::InvalidTransferBombFormat => {
                "No se cumple el formato del item Bomba de Traspaso, use: SXXX con XXX igual a un numero natural"
            }
            BombermanError::InvalidDeflectionFormat => {
                "No se cumple el formato del item Desvio, use: DX con X una direccion valida (L, R, U, D)"
            }
            BombermanError::InvalidBombCoordinate => {
                "En la coordenada proporcionada no se encontro una bomba"
            }
            BombermanError::OutputPathError => {
                "El archivo output no se pudo crear"
            }
            BombermanError::Write => {
                "No se pudo escribir en el archivo output"
            }
            BombermanError::NonSquareBoardError => {
                "El tablero no es cuadrado"
            }
        }
    }

    pub fn send(&self, path_output: String) {
        let error = format!("Error: {}", self.message());

        let mut file = match File::create(path_output) {
            Ok(file) => file,
            Err(_) => {
                println!("{}", error);
                return;
            }
        };
        match file.write_all(error.as_bytes()) {
            Ok(_) => (),
            Err(_) => println!("{}", error),
        }
    }
}
