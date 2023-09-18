use std::{
    any::Any,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::error::BombermanError;
type Operacion = fn(&String, usize, &mut dyn Any) -> Result<(), BombermanError>;


/// Lee un archivo de entrada en la ruta especificada y aplica una operación personalizada
/// a cada línea del archivo.
///
/// Esta función toma tres argumentos: una referencia a una cadena de texto `path` que
/// representa la ruta del archivo de entrada, una función de operación personalizada `process`
/// que se aplicará a cada línea del archivo, y un puntero mutable a un objeto dinámico `ptr`.
/// La función `process` debe tener la siguiente firma:
///
/// ```
/// fn process(line: &str, row: usize, ptr: &mut dyn Any) -> Result<(), BombermanError>;
/// ```
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
/// # Notas
///
/// - Asegúrese de proporcionar una función de operación `process` válida que cumpla con la
///   firma requerida.
/// - Los resultados o datos adicionales se pueden almacenar y manipular a través del objeto
///   `ptr`, que debe implementar el trait `std::any::Any`.
///
/// # Importación
///
/// Debe importar el módulo `std::fs::File`, `std::io::BufReader`, y tener acceso al enum
/// personalizado `BombermanError` definido en su proyecto para utilizar esta función.
///
/// # Más información
///
/// Para obtener más información sobre la lectura de archivos en Rust, consulte la documentación
/// oficial de Rust: https://doc.rust-lang.org/std/fs/struct.File.html
///
pub fn read_input(
    path: &String,
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
        process(&line, row, ptr)?;
        row += 1;
        line.clear();
    }
    Ok(())
}

/// Abre un archivo en modo de lectura en la ruta especificada.
///
/// Esta función toma una referencia a una cadena de texto `path` que representa la
/// ruta del archivo y devuelve un resultado (`Result`) que contiene un archivo (`File`)
/// en caso de éxito o un error personalizado (`BombermanError::InputPathError`) en caso
/// de fallo.
///
/// # Ejemplo
///
/// ```
/// use std::fs::File;
/// use bomberman::file_io::open_file_for_reading;
///
/// let result = open_file_for_reading("input.txt");
/// match result {
///     Ok(file) => {
///         // Operaciones de lectura en el archivo.
///     },
///     Err(error) => {
///         println!("Error al abrir el archivo: {}", error.message());
///     }
/// }
/// ```
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
        Err(_) => return Err(BombermanError::InputPathError),
    }
}

/// Abre un archivo en modo de escritura en la ruta especificada.
///
/// Esta función toma una cadena de texto `path` que representa la ruta del archivo
/// y devuelve un resultado (`Result`) que contiene un archivo (`File`) en caso de éxito
/// o un error personalizado (`BombermanError::OutputPathError`) en caso de fallo.
///
/// # Ejemplo
///
/// ```
/// use std::fs::File;
/// use bomberman::file_io::open_file_for_writing;
/// 
/// let result = open_file_for_writing("output.txt");
/// match result {
///     Ok(file) => {
///         // Operaciones de escritura en el archivo.
///     },
///     Err(error) => {
///         println!("Error al abrir el archivo: {}", error.message());
///     }
/// }
/// ```
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
