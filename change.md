
3) Item no reconocido -> F SOLA, no ayuda nada
4) Falta documentar esta funcion: file::io -> get_matrix_dimensions (
5) Si falla al obtener las dimensiones de la matriz me dice que el tablero noe s cuadrado y eso esta mal (Listo)
6) Fijarse si el desvio hace que la bomba vuelva a estallar
Test unitarios:
    - Config
    - Item
        if self.x + 1 == self.max_value {
            return None;
        } // Pensar en estos mas 1
En cambio, si misma rafaga de una misma bomba impacta en un enemigo mas de una vez (por ejemplo, si el enemigo se encontrase entre una bomba y un desvio), entonces recibe daño solo una vez.