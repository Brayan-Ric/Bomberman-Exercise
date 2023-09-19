
2) Imprimo un espacio de mas al final de la matriz 
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
5) EL +1 en range es por detalles de implementacion, porque cuento desde donde estoy
7) URGENTE: INVERTIR COORDENADAS X e Y en el input (Listo)
8) Un enemigo solo puede tener 1 a 3 puntos de vida
9) Si un enemigo se encuentra en medio de dos o mas bombas, recibe daño de todas las rafagas.
En cambio, si misma rafaga de una misma bomba impacta en un enemigo mas de una vez (por ejemplo, si el enemigo se encontrase entre una bomba y un desvio), entonces recibe daño solo una vez.
10) Debe imprimir por la salida estandas. El config