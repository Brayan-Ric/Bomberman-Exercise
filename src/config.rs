// use constants::*;

use std::env;

use crate::constants::*;
use crate::error::BombermanError;

/// Configuración para la ejecución del programa Bomberman.
///
/// Esta estructura representa la configuración necesaria para ejecutar el programa Bomberman.
///
/// # Campos
///
/// * `name_input`: Nombre del archivo de entrada que contiene el mapa del juego.
/// * `path_output`: Ruta de la carpeta/directorio donde se guardarán los resultados del juego.
/// * `x`: Coordenada X de la primera bomba a detonar.
/// * `y`: Coordenada Y de la primera bomba a detonar.
///
/// Esta estructura encapsula la configuración necesaria para la ejecución de Bomberman, incluyendo
/// el nombre del archivo de entrada, la ruta de la carpeta de salida y las coordenadas de la
/// primera bomba a detonar.
///
/// # Notas
///
/// Asegúrate de proporcionar valores válidos para los campos `x` y `y` que estén dentro de los
/// límites del mapa del juego.
///
#[derive(Debug)]
pub struct Config {
    pub name_input: String,
    pub path_output: String,
    pub x: usize,
    pub y: usize,
}

impl Config {
    /// Crea una nueva instancia de `Config` a partir de los argumentos de línea de comandos.
    ///
    /// Esta función toma los argumentos de línea de comandos proporcionados por el usuario y crea una
    /// nueva instancia de `Config` que contiene la información necesaria para configurar la ejecución
    /// del juego Bomberman.
    ///
    /// # Argumentos
    ///
    /// * `args`: Un vector de cadenas que contiene los argumentos de línea de comandos. Debe tener
    ///   exactamente 5 elementos, incluyendo el nombre del programa.
    ///
    /// # Errores
    ///
    /// Esta función devuelve un resultado que indica si la creación de la configuración fue exitosa o
    /// si hubo errores en los argumentos de entrada.
    ///
    /// # Notas
    ///
    /// Asegúrate de proporcionar 5 argumentos de línea de comandos válidos, incluyendo el nombre del
    /// programa, la ruta al archivo de entrada, la ruta de la carpeta de salida, la coordenada X y la
    /// coordenada Y.
    ///
    pub fn new() -> Result<Config, BombermanError> {
        let args: Vec<String> = env::args().collect();
        if args.len() != 5 {
            return Err(BombermanError::InsufficientInput);
        }

        let x = get_coordinate(&args[X_IDX])?;
        let y = get_coordinate(&args[Y_IDX])?;
        Ok(Config {
            name_input: args[INPUT_IDX].clone(),
            path_output: args[OUTPUT_IDX].clone() + &args[INPUT_IDX].clone(),
            x,
            y,
        })
    }
}

/// Convierte una cadena en un número entero no negativo.
///
/// Esta función toma una cadena como entrada y la intenta convertir en un número entero no negativo.
/// Si la conversión es exitosa, devuelve el número entero. Si la conversión falla o el número es
/// negativo, se devuelve un error indicando que la coordenada no es válida.
///
/// # Argumentos
///
/// * `s`: Una referencia a una cadena que representa una coordenada.
///
/// # Errores
///
/// Esta función devuelve un resultado que contiene el número entero no negativo si la conversión es
/// exitosa. Si la conversión falla o el número es negativo, se devuelve un error de tipo
/// `BombermanError` que indica que la coordenada no es válida.
///
/// # Notas
///
/// Asegúrate de proporcionar una cadena que represente un número entero no negativo como entrada
/// para esta función. En caso contrario, se generará un error.
///
fn get_coordinate(s: &String) -> Result<usize, BombermanError> {
    match s.parse::<usize>() {
        Ok(value) => Ok(value),
        Err(_) => return Err(BombermanError::InvalidCoordinate),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_coordinate() {
        let input = "10".to_string();
        let result = get_coordinate(&input);
        assert_eq!(result, Ok(10));
    }

    #[test]
    fn test_invalid_coordinate() {
        let input = "abc".to_string();
        let result = get_coordinate(&input);
        assert_eq!(result, Err(BombermanError::InvalidCoordinate));
    }
}
