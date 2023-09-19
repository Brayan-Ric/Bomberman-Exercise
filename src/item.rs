use crate::{
    constants::{DEFLECTION, DOWN, ENEMY, LEFT, NORMAL_BOMB, RIGHT, ROCK, TRANSFER_BOMB, UP, WALL},
    error::BombermanError,
};

/// Representa los elementos en el mundo del juego Bomberman.
///
/// El enum `Item` define los diversos elementos que pueden encontrarse en el mundo del juego Bomberman.
/// Cada variante enumera un tipo de elemento y, en algunos casos, incluye datos adicionales para describir
/// el elemento en más detalle.
///
/// # Variantes
///
/// - `Enemy(u32)`: Representa a un enemigo con su cantidad de vida (valor entero sin signo).
/// - `NormalBomb(u32)`: Representa una bomba normal con un rango de explosion especificado (valor entero sin signo).
/// - `TransferBomb(u32)`: Representa una bomba de transferencia con un rango de explosion especificado (valor entero sin signo).
/// - `Rock`: Representa una roca en el juego, un obstáculo que bloquea el paso de las bombas normales.
/// - `Wall`: Representa una pared en el juego, un obstáculo indestructible.
/// - `Deflection(char)`: Representa un elemento de desviación con una dirección especificada (carácter).
/// - `Empty`: Representa una casilla vacía sin ningún elemento.
///
/// # Importación
///
/// Debe tener acceso al enum `Item` definido en su proyecto para utilizar esta estructura de datos.
///
/// # Más información
///
/// Para obtener más información sobre la enumeración en Rust, consulte la documentación oficial:
/// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
///

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Item {
    Enemy(u32),
    NormalBomb(u32),
    TransferBomb(u32),
    Rock,
    Wall,
    Deflection(char),
    Empty,
}

impl Item {
    /// Crea un nuevo elemento `Item` a partir de una cadena de caracteres `s`.
    ///
    /// Esta función toma una cadena de caracteres `s` y la analiza para determinar qué tipo de elemento
    /// se representa. Luego, crea y devuelve un elemento `Item` correspondiente al tipo detectado.
    ///
    /// # Argumentos
    ///
    /// - `s`: Un `&str` que representa la cadena de caracteres que se analizará para determinar el tipo de elemento.
    ///
    /// # Errores
    ///
    /// Esta función puede devolver varios errores en caso de condiciones no válidas:
    ///
    /// - `BombermanError::InvalidItem`: Se produce cuando la cadena `s` está vacía o no se reconoce como ningún tipo de elemento válido.
    /// - `BombermanError::InvalidEnemyFormat`: Se produce cuando la cadena `s` representa un enemigo, pero el formato no es válido.
    /// - `BombermanError::InvalidNormalBombFormat`: Se produce cuando la cadena `s` representa una bomba normal, pero el formato no es válido.
    /// - `BombermanError::InvalidTransferBombFormat`: Se produce cuando la cadena `s` representa una bomba de transferencia, pero el formato no es válido.
    /// - `BombermanError::InvalidDeflectionFormat`: Se produce cuando la cadena `s` representa un elemento de desviación, pero el formato no es válido.
    ///
    /// # Importación
    ///
    /// Debe tener acceso a la estructura `Item` y al enum `BombermanError` definidos en su proyecto
    /// para utilizar esta función.
    ///
    /// # Más información
    ///
    /// Esta función facilita la creación de elementos `Item` a partir de cadenas de caracteres.
    /// Si desea crear elementos `Item` manualmente, consulte la documentación de la estructura `Item`.
    ///
    /// Para obtener más información sobre el manejo de errores en Rust, consulte:
    /// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
    ///
    pub fn new(s: &str) -> Result<Item, BombermanError> {
        if s.len() == 0 {
            return Err(BombermanError::InvalidItem);
        }

        if s.len() == 1 {
            match s {
                WALL => return Ok(Item::Wall),
                ROCK => return Ok(Item::Rock),
                _ => return Err(BombermanError::InvalidItem),
            }
        }

        let item = match s.chars().next() {
            Some(i) => i,
            None => return Err(BombermanError::InvalidItem),
        };

        match item {
            ENEMY => Ok(Item::Enemy(get_value(
                s,
                BombermanError::InvalidEnemyFormat,
            )?)),
            NORMAL_BOMB => Ok(Item::NormalBomb(get_value(
                s,
                BombermanError::InvalidNormalBombFormat,
            )?)),
            TRANSFER_BOMB => Ok(Item::TransferBomb(get_value(
                s,
                BombermanError::InvalidTransferBombFormat,
            )?)),
            DEFLECTION => Ok(Item::Deflection(get_address(
                s,
                BombermanError::InvalidDeflectionFormat,
            )?)),
            _ => Err(BombermanError::InvalidItem),
        }
    }
}

/// Implementación de la forma de mostrar (`std::fmt::Display`) para el enum `Item`.
///
/// Esta implementación permite mostrar un valor de tipo `Item` como una cadena de caracteres.
///
/// # Nota
///
/// Esta implementación se utiliza automáticamente cuando se usa la macro `format!` o la función `println!` con valores de tipo `Item`.
///
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Enemy(life) => write!(f, "F{}", life),
            Item::NormalBomb(scope) => write!(f, "B{}", scope),
            Item::TransferBomb(scope) => write!(f, "S{}", scope),
            Item::Deflection(direction) => write!(f, "D{}", direction),
            Item::Rock => write!(f, "R"),
            Item::Wall => write!(f, "W"),
            Item::Empty => write!(f, "_"),
        }
    }
}

/// Obtiene el valor numérico a partir de una cadena de caracteres `s`.
///
/// Esta función toma una cadena de caracteres `s` y extrae el valor numérico que sigue a la primera letra.
///
/// # Argumentos
///
/// - `s`: Un `&str` que representa la cadena de caracteres de la cual se extraerá el valor numérico.
/// - `e`: Un valor de tipo `BombermanError` que se utilizará como error en caso de que no se pueda extraer un valor numérico válido.
///
/// # Precondición
///
/// Antes de llamar a esta función, se asume que la primera letra en la cadena `s` representa el tipo de Item.
///
/// # Errores
///
/// Esta función devuelve un resultado que contiene un valor numérico (`u32`) o un error en caso de que no se pueda extraer un valor numérico válido.
///
/// - `BombermanError`: Se produce cuando no se puede extraer un valor numérico válido de la cadena `s`.
///
/// # Importación
///
/// Debe tener acceso al enum `BombermanError` definido en su proyecto para utilizar esta función.
///
/// # Más información
///
/// Esta función es útil para extraer valores numéricos de cadenas de caracteres que siguen un formato específico.
/// Si necesita extraer otros tipos de datos, considere adaptar esta función según sus necesidades.
///
fn get_value(s: &str, e: BombermanError) -> Result<u32, BombermanError> {
    let num_str: String = s.chars().skip(1).collect();

    // Intentamos convertir la cadena en un número
    match num_str.parse::<u32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(e),
    }
}

/// Extrae y retorna el segundo carácter de una cadena como un valor `char`.
///
/// Esta función toma una cadena como entrada y verifica si tiene exactamente dos caracteres. Si es así, extrae y retorna el segundo carácter como un `char`. Si la cadena no tiene dos caracteres, se devuelve un error `BombermanError`.
///
/// # Argumentos
///
/// * `s`: La cadena de entrada de la cual se extraerá el segundo carácter.
/// * `e`: El error a devolver en caso de que la cadena no tenga exactamente dos caracteres.
///
/// # Errores
///
/// Si la cadena de entrada no tiene exactamente dos caracteres, se devuelve un error `BombermanError`.
///
fn get_address(s: &str, e: BombermanError) -> Result<char, BombermanError> {
    if s.len() != 2 {
        return Err(e);
    }

    match s.chars().nth(1) {
        Some(c) if c == RIGHT || c == LEFT || c == DOWN || c == UP => Ok(c),
        _ => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_address_valid_l() {
        assert_eq!(
            get_address("XL", BombermanError::InvalidDeflectionFormat),
            Ok(LEFT)
        );
    }

    #[test]
    fn test_get_address_valid_r() {
        assert_eq!(
            get_address("XR", BombermanError::InvalidDeflectionFormat),
            Ok(RIGHT)
        );
    }

    #[test]
    fn test_get_address_valid_u() {
        assert_eq!(
            get_address("XU", BombermanError::InvalidDeflectionFormat),
            Ok(UP)
        );
    }

    #[test]
    fn test_get_address_valid_d() {
        assert_eq!(
            get_address("XD", BombermanError::InvalidDeflectionFormat),
            Ok(DOWN)
        );
    }

    #[test]
    fn test_get_address_invalid() {
        assert!(get_address("X", BombermanError::InvalidDeflectionFormat).is_err());
    }

    #[test]
    fn test_get_value_valid_enemy() {
        assert_eq!(get_value("F10", BombermanError::InvalidEnemyFormat), Ok(10));
    }

    #[test]
    fn test_get_value_valid_bomb_normal() {
        assert_eq!(
            get_value("B11", BombermanError::InvalidNormalBombFormat),
            Ok(11)
        );
    }

    #[test]
    fn test_get_value_valid_bomb_transfer() {
        assert_eq!(
            get_value("S22", BombermanError::InvalidTransferBombFormat),
            Ok(22)
        );
    }

    #[test]
    fn test_get_value_invalid() {
        assert!(get_value("FXYZ", BombermanError::InvalidEnemyFormat).is_err());
    }
    #[test]
    fn test_new_item_enemy() {
        assert_eq!(Item::new("F123"), Ok(Item::Enemy(123)));
    }

    #[test]
    fn test_new_item_normal_bomb() {
        assert_eq!(Item::new("B10"), Ok(Item::NormalBomb(10)));
    }

    #[test]
    fn test_new_item_transfer_bomb() {
        assert_eq!(Item::new("S11"), Ok(Item::TransferBomb(11)));
    }

    #[test]
    fn test_new_item_deflection() {
        assert_eq!(Item::new("DD"), Ok(Item::Deflection(DOWN)));
    }

    #[test]
    fn test_new_item_wall() {
        assert_eq!(Item::new("W"), Ok(Item::Wall));
    }

    #[test]
    fn test_new_item_rock() {
        assert_eq!(Item::new("R"), Ok(Item::Rock));
    }

    #[test]
    fn test_new_item_invalid_1() {
        assert_eq!(Item::new("X"), Err(BombermanError::InvalidItem));
    }

    #[test]
    fn test_new_item_invalid_2() {
        assert_eq!(Item::new(""), Err(BombermanError::InvalidItem));
    }

    #[test]
    fn test_new_item_invalid_3() {
        assert_eq!(Item::new("C183"), Err(BombermanError::InvalidItem));
    }
}
