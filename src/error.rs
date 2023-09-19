use std::{fs::File, io::Write};

/// Enumeración que representa los posibles errores que pueden ocurrir durante la ejecución del programa Bomberman.
///
/// Cada variante de este enum representa un tipo específico de error que puede ocurrir, y se utiliza para
/// identificar y manejar los errores de manera adecuada.
///
/// # Variantes
///
/// - `InvalidCoordinate`: Indica que las coordenadas proporcionadas no son válidas.
/// - `InsufficientInput`: Indica que la entrada proporcionada no es suficiente.
/// - `InputPathError`: Indica que se produjo un error al acceder al archivo de entrada.
/// - `InvalidItem`: Indica que se encontró un item no reconocido en el tablero.
/// - `InvalidItemFormat`: Indica que el formato de un item no es válido.
/// - `InvalidEnemyFormat`: Indica que el formato del item "Enemy" no es válido.
/// - `InvalidNormalBombFormat`: Indica que el formato del item "Bomba Normal" no es válido.
/// - `InvalidTransferBombFormat`: Indica que el formato del item "Bomba de Traspaso" no es válido.
/// - `InvalidDeflectionFormat`: Indica que el formato del item "Desvío" no es válido.
/// - `InvalidBombCoordinate`: Indica que no se encontró una bomba en las coordenadas proporcionadas.
/// - `OutputPathError`: Indica que se produjo un error al acceder al archivo de salida.
/// - `Write`: Indica que ocurrió un error al escribir en el archivo de salida.
/// - `NonSquareBoardError`: Indica que el tablero no tiene dimensiones cuadradas.
/// - `InvalidEnemyLife`: Indica que la vida de un enemigo es inválida.
/// - `EmptyFileError`: Indica que el archivo de entrada está vacío.
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
    InvalidEnemyLife,
    EmptyFileError,
}

impl BombermanError {
    /// Obtiene el mensaje descriptivo correspondiente al error actual.
    ///
    /// Esta función devuelve un mensaje de error descriptivo basado en el tipo de error que se ha producido. Los mensajes
    /// de error proporcionados son informativos y ayudan a identificar la causa del error.
    ///
    /// # Retorno
    ///
    /// Un valor `&str` que contiene el mensaje descriptivo del error actual.
    ///
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
            BombermanError::InvalidEnemyLife => {
                "La vida de un enemigo esta fuera del rango. Rango: 1 al 3"
            }
            BombermanError::EmptyFileError => {
                "El archivo de entrada esta vacio"
            }
        }
    }

    /// Escribe el mensaje de error actual en un archivo especificado por `path_output`.
    ///
    /// Esta función toma el `message` actual del error y lo escribe en un archivo especificado por la ruta
    /// `path_output`. Si se puede crear el archivo y escribir el mensaje con éxito, no se muestra ningún mensaje
    /// adicional en la consola. En caso de que ocurra un error durante el proceso de escritura o creación del archivo,
    /// se mostrará el mensaje de error en la consola.
    ///
    /// # Argumentos
    ///
    /// * `path_output` - La ruta al archivo donde se escribirá el mensaje de error.
    ///
    /// Esta función se utiliza para escribir el mensaje de error actual en un archivo. Si la escritura es exitosa,
    /// no se mostrará ningún mensaje adicional en la consola. En caso de error, se mostrará el mensaje de error
    /// en la consola.
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
