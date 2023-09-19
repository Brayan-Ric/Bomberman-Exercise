1) La dimension de la matriz ahora es constante / Listo
2) Imprimo un espacio de mas al final de la matriz
3) Item no reconocido -> F SOLA, no ayuda nada
4) Falta documentar esta funcion: file::io -> get_matrix_dimensions
5) Si falla al obtener las dimensiones de la matriz me dice que el tablero noe s cuadrado y eso esta mal
6) Fijarse si el desvio hace que la bomba vuelva a estallar
Test unitarios:
    - Config
    - Item
        if self.x + 1 == self.max_value {
            return None;
        } // Pensar en estos mas 1
5) EL +1 en range es por detalles de implementacion, porque cuento desde donde estoy