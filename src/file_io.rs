use std::{
    any::Any,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::error::BombermanError;
type Operacion = fn(&String, usize, u32, &mut dyn Any) -> Result<(), BombermanError>;

/// Lee un archivo de entrada en la ruta especificada y aplica una operación personalizada
/// a cada línea del archivo.
///
/// Esta función toma tres argumentos: una referencia a una cadena de texto `path` que
/// representa la ruta del archivo de entrada, una función de operación personalizada `process`
/// que se aplicará a cada línea del archivo, y un puntero mutable a un objeto dinámico `ptr`.
///
/// Donde `line` es la línea actual del archivo, `row` es el número de fila actual y `ptr`
/// es un puntero mutable al objeto dinámico que se puede utilizar para almacenar resultados
/// o datos adicionales.
///
/// # Errores
///
/// Esta función puede devolver un error personalizado `BombermanError` si se encuentra un
/// problema al leer el archivo o al aplicar la función de operación personalizada. Los
/// detalles del error se incluirán en el resultado.
///
pub fn read_input(
    path: &str,
    max_value: u32,
    process: Operacion,
    ptr: &mut dyn Any,
) -> Result<(), BombermanError> {
    let file = open_file_for_reading(path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut line = String::new();
    let mut row: usize = 0;

    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break; // Fin del archivo
        }
        process(&line, row, max_value, ptr)?;
        row += 1;
        line.clear();
    }
    Ok(())
}
pub fn get_matrix_dimensions(path: &str) -> Result<Option<usize>, BombermanError> {
    let file = open_file_for_reading(path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut line = String::new();
    let mut expected_columns: Option<usize> = None;
    let mut row: usize = 0;

    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break; // Fin del archivo
        }

        let columns: Vec<&str> = line.split_whitespace().collect();

        // Verificar si este es el primer renglón para establecer el número esperado de columnas
        if let Some(expected) = expected_columns {
            if columns.len() != expected {
                return Ok(None);
            }
        } else {
            expected_columns = Some(columns.len());
        }
        row += 1;
        line.clear();
    }
    if row != expected_columns.unwrap_or(0) {
        return Err(BombermanError::NonSquareBoardError);
    }
    Ok(expected_columns)
}

/// Abre un archivo en modo de lectura en la ruta especificada.
///
/// Esta función toma una referencia a una cadena de texto `path` que representa la
/// ruta del archivo y devuelve un resultado (`Result`) que contiene un archivo (`File`)
/// en caso de éxito o un error personalizado (`BombermanError::InputPathError`) en caso
/// de fallo.
///
/// # Errores
///
/// Esta función puede devolver un error personalizado `BombermanError::InputPathError` si no
/// se puede abrir el archivo en la ruta especificada debido a un problema en el sistema de
/// archivos, permisos insuficientes o si el archivo no existe.
///
/// # Notas
///
/// Asegúrese de manejar adecuadamente los resultados devueltos por esta función para gestionar
/// los errores de apertura o lectura de archivos.
///
/// # Panics
///
/// No se producirán "panics" en esta función. Si ocurre un error, se informará a través del
/// resultado (`Result`) en lugar de hacer que el programa falle.
///
/// # Importación
///
/// Debe importar el módulo `std::fs::File` y tener acceso al enum personalizado `BombermanError`
/// definido en su proyecto para utilizar esta función.
///
/// # Más información
///
/// Para obtener más información sobre el manejo de archivos en Rust, consulte la documentación
/// oficial de Rust: https://doc.rust-lang.org/std/fs/struct.File.html
///
pub fn open_file_for_reading(path: &str) -> Result<File, BombermanError> {
    match File::open(path) {
        Ok(f) => Ok(f),
        Err(_) => Err(BombermanError::InputPathError),
    }
}

/// Abre un archivo en modo de escritura en la ruta especificada.
///
/// Esta función toma una cadena de texto `path` que representa la ruta del archivo
/// y devuelve un resultado (`Result`) que contiene un archivo (`File`) en caso de éxito
/// o un error personalizado (`BombermanError::OutputPathError`) en caso de fallo.
///
/// # Errores
///
/// Esta función puede devolver un error personalizado `BombermanError::OutputPathError`
/// si no se puede abrir o crear el archivo en la ruta especificada debido a un problema
/// en el sistema de archivos o permisos insuficientes.
///
/// # Notas
///
/// Asegúrese de manejar adecuadamente los resultados devueltos por esta función para
/// gestionar los errores de apertura o escritura en archivos.
///
/// # Panics
///
/// No se producirán "panics" en esta función. Si ocurre un error, se informará a través
/// del resultado (`Result`) en lugar de hacer que el programa falle.
///
/// # Importación
///
/// Debe importar el módulo `std::fs::File` y tener acceso al enum personalizado `BombermanError`
/// definido en su proyecto para utilizar esta función.
///
/// # Más información
///
/// Para obtener más información sobre el manejo de archivos en Rust, consulte la documentación
/// oficial de Rust: https://doc.rust-lang.org/std/fs/struct.File.html
///
pub fn open_file_for_writing(path: &str) -> Result<File, BombermanError> {
    match File::create(path) {
        Ok(f) => Ok(f),
        Err(_) => Err(BombermanError::OutputPathError),
    }
}

#[cfg(test)]
mod tests {
    use crate::game::process_line;

    use super::*;

    #[test]
    fn test_open_file_for_reading_file_not_found() {
        let file_path = "archivo_que_no_existe.txt";
        let result = open_file_for_reading(file_path);
        assert_eq!(result.unwrap_err(), BombermanError::InputPathError);
    }

    #[test]
    fn test_open_file_for_writing_file_not_found() {
        let file_path = "path_no_existe/archivo_que_no_existe.txt";
        let result = open_file_for_writing(file_path);
        assert_eq!(result.unwrap_err(), BombermanError::OutputPathError);
    }

    #[test]
    fn test_read_input_not_found() {
        let path = "archivo_que_no_existe.txt".to_string();
        let mut ptr = "ptr".to_string();
        let result = read_input(&path, 0, process_line, &mut ptr);
        assert_eq!(result.unwrap_err(), BombermanError::InputPathError);
    }

    #[test]
    fn test_get_matrix_dimensions_not_found() {
        let file_path = "archivo_que_no_existe.txt";
        let result = get_matrix_dimensions(file_path);
        assert_eq!(result.unwrap_err(), BombermanError::InputPathError);
    }

    #[test]
    fn test_get_matrix_dimensions_non_square() {
        let file_path = "./tests/inputs/non_square_matrix.txt";
        let result = get_matrix_dimensions(file_path);
        assert_eq!(result.unwrap_err(), BombermanError::NonSquareBoardError);
    }
}
