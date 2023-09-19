/// Representa una coordenada en un sistema bidimensional con valores enteros no negativos.
///
/// `Coordinate` se utiliza para representar posiciones en un tablero o en cualquier espacio
/// bidimensional donde `x` y `y` son las coordenadas en los ejes horizontal y vertical,
/// respectivamente.
///
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
    pub max_value: u32,
}
impl Coordinate {
    /// Crea una nueva instancia de `Coordinate` con las coordenadas especificadas.
    ///
    /// # Argumentos
    ///
    /// * `x`: Valor de la coordenada en el eje horizontal (X).
    /// * `y`: Valor de la coordenada en el eje vertical (Y).
    /// * `max_value`: Valor máximo permitido para las coordenadas.
    ///
    ///
    /// Esta función devuelve una nueva instancia de `Coordinate` con las coordenadas `x` e `y`
    /// especificadas, así como el valor máximo permitido para las coordenadas. Se utiliza para
    /// crear objetos `Coordinate` de manera conveniente.
    /// 
    /// # Observacion:
    /// Para mejorar la eficiencia del tp, se agrega el valor maximo de las coordenadas, para no tener que calcularlo cada vez que se llama a las funciones de movimiento.
    /// Pero tambien no se valida el maximo de las coordenadas, ya que la implementacion hecha nos asegura que no se va a salir del tablero.
    pub fn new(x: u32, y: u32, max_value: u32) -> Coordinate {
        Coordinate { x, y, max_value }
    }

    /// Esta función permite desplazar una coordenada hacia abajo en el eje vertical y devuelve una nueva
    /// coordenada si el desplazamiento es válido. Si el desplazamiento excede el valor máximo permitido,
    /// se devuelve `None`.
    pub fn down(&self) -> Option<Coordinate> {
        if self.x == self.max_value {
            return None;
        }
        Some(Coordinate::new(self.x + 1, self.y, self.max_value))
    }

    /// Esta función permite desplazar una coordenada hacia arriba en el eje vertical y devuelve una nueva
    /// coordenada si el desplazamiento es válido. Si la coordenada ya se encuentra en la posición más alta,
    /// se devuelve `None`.
    pub fn up(&self) -> Option<Coordinate> {
        if self.x == 0 {
            return None;
        }
        Some(Coordinate::new(self.x - 1, self.y, self.max_value))
    }
    /// Esta función permite desplazar una coordenada hacia la derecha en el eje horizontal y devuelve una nueva
    /// coordenada si el desplazamiento es válido. Si la coordenada ya se encuentra en la posición más a la derecha,
    /// se devuelve `None`.
    pub fn right(&self) -> Option<Coordinate> {
        if self.y == self.max_value {
            return None;
        }
        Some(Coordinate::new(self.x, self.y + 1, self.max_value))
    }

    /// Esta función permite desplazar una coordenada hacia la izquierda en el eje horizontal y devuelve una nueva
    /// coordenada si el desplazamiento es válido. Si la coordenada ya se encuentra en la posición más a la izquierda,
    /// se devuelve `None`.
    pub fn left(&self) -> Option<Coordinate> {
        if self.y == 0 {
            return None;
        }
        Some(Coordinate::new(self.x, self.y - 1, self.max_value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_down_within_bounds() {
        let coordinate = Coordinate::new(2, 3, 4);
        let result = coordinate.down();
        assert_eq!(result, Some(Coordinate::new(3, 3, 4)));
    }

    #[test]
    fn test_down_at_max_value() {
        let coordinate = Coordinate::new(4, 3, 4);
        let result = coordinate.down();
        assert_eq!(result, None);
    }

    #[test]
    fn test_up() {
        let coordinate = Coordinate::new(2, 3, 5);
        let result = coordinate.up();
        assert_eq!(result, Some(Coordinate::new(1, 3, 5)));
    }

    #[test]
    fn test_up_at_minimum_x() {
        let coordinate = Coordinate::new(0, 3, 5);
        let result = coordinate.up();
        assert_eq!(result, None);
    }

    #[test]
    fn test_right() {
        let coordinate = Coordinate::new(2, 3, 5);
        let result = coordinate.right();
        assert_eq!(result, Some(Coordinate::new(2, 4, 5)));
    }

    #[test]
    fn test_right_at_maximum_y() {
        let coordinate = Coordinate::new(2, 5, 5);
        let result = coordinate.right();
        assert_eq!(result, None);
    }
    #[test]
    fn test_left() {
        let coord = Coordinate::new(2, 3, 5);
        let result = coord.left();
        assert_eq!(result, Some(Coordinate::new(2, 2, 5)));
    }

    #[test]
    fn test_left_at_minimum_y() {
        let coord = Coordinate::new(2, 0, 5);
        let result = coord.left();
        assert_eq!(result, None);
    }

}
