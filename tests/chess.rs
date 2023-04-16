use ajedrez::chess_engine;

const WHITES_WIN: char = 'B';
const BLACKS_WIN: char = 'N';
const DRAW: char = 'E';
const NO_WINNER: char = 'P';

#[test]
fn blancas_ganan_por_rey() {
    let file_name = "./TablerosDePrueba/BlancasGananPorRey.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, WHITES_WIN),
        Err(..) => assert!(false),
    }
}

#[test]
fn blancas_ganan_por_dama() {
    let file_name = "./TablerosDePrueba/BlancasGananPorDama.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, WHITES_WIN),
        Err(..) => assert!(false),
    }
}

#[test]
fn blancas_ganan_por_torre() {
    let file_name = "./TablerosDePrueba/BlancasGananPorTorre.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, WHITES_WIN),
        Err(..) => assert!(false),
    }
}

#[test]
fn blancas_ganan_por_alfil() {
    let file_name = "./TablerosDePrueba/BlancasGananPorAlfil.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, WHITES_WIN),
        Err(..) => assert!(false),
    }
}

#[test]
fn blancas_ganan_por_caballo() {
    let file_name = "./TablerosDePrueba/BlancasGananPorCaballo.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, WHITES_WIN),
        Err(..) => assert!(false),
    }
}

#[test]
fn blancas_ganan_por_peon() {
    let file_name = "./TablerosDePrueba/BlancasGananPorPeon.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, WHITES_WIN),
        Err(..) => assert!(false),
    }
}

#[test]
fn negras_ganan_por_rey() {
    let file_name = "./TablerosDePrueba/NegrasGananPorRey.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, BLACKS_WIN),
        Err(..) => assert!(false),
    }
}

#[test]
fn negras_ganan_por_dama() {
    let file_name = "./TablerosDePrueba/NegrasGananPorDama.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, BLACKS_WIN),
        Err(..) => assert!(false),
    }
}

#[test]
fn negras_ganan_por_torre() {
    let file_name = "./TablerosDePrueba/NegrasGananPorTorre.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, BLACKS_WIN),
        Err(..) => assert!(false),
    }
}

#[test]
fn negras_ganan_alfil() {
    let file_name = "./TablerosDePrueba/NegrasGananPorAlfil.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, BLACKS_WIN),
        Err(..) => assert!(false),
    }
}

#[test]
fn negras_ganan_por_caballo() {
    let file_name = "./TablerosDePrueba/NegrasGananPorCaballo.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, BLACKS_WIN),
        Err(..) => assert!(false),
    }
}

#[test]
fn negras_ganan_por_peon() {
    let file_name = "./TablerosDePrueba/NegrasGananPorPeon.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, BLACKS_WIN),
        Err(..) => assert!(false),
    }
}

#[test]
fn empate() {
    let file_name = "./TablerosDePrueba/Empate.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, DRAW),
        Err(..) => assert!(false),
    }
}

#[test]
fn nadie_gana() {
    let file_name = "./TablerosDePrueba/NadieGana.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, NO_WINNER),
        Err(..) => assert!(false),
    }
}

#[test]
fn validando_orientacion_del_peon_blanco() {
    let file_name = "./TablerosDePrueba/PeonBlancoNoPuedeCapturar.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, BLACKS_WIN),
        Err(..) => assert!(false),
    }
}

#[test]
fn validando_orientacion_del_peon_negro() {
    let file_name = "./TablerosDePrueba/PeonNegroNoPuedeCapturar.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, WHITES_WIN),
        Err(..) => assert!(false),
    }
}

#[test]
fn validando_orientacion_de_dos_peones() {
    let file_name = "./TablerosDePrueba/NingunPeonPuedeCapturar.txt".to_string();
    match chess_engine::game(&file_name) {
        Ok(c) => assert_eq!(c, NO_WINNER),
        Err(..) => assert!(false),
    }
}
