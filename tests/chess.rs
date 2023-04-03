use ajedrez::chess_engine;
use std::io::Write;

const WHITES_WIN: &str = "B\n";
const BLACKS_WIN: &str = "N\n";
const DRAW: &str = "E\n";
const NO_WINNER: &str = "P\n";

#[test]
fn blancas_ganan() {
    let file_name = "./TablerosDePrueba/BlancasGanan.txt".to_string();
    let mut buffer = Vec::new();
    std::io::stdout().write_all(&mut buffer).unwrap(); // redirige la salida de la función a `buffer`
    match chess_engine::game(&file_name) {
        Ok(_) => (),
        _ => assert!(false),
    } 
    // chess_engine::game(&file_name); // llama a la función
    assert_eq!(std::str::from_utf8(&buffer).unwrap(), WHITES_WIN); // verifica que la salida sea correcta
}