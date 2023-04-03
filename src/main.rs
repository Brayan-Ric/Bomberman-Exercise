pub use ajedrez::ChessSyntaxValidator;
pub use ajedrez::ChessBoardLoader;
pub use ajedrez::ChessEngine;
pub use ajedrez::chess_error::ChessError;
pub use ajedrez::chess_error;


fn main() {

    let file_name = match ChessBoardLoader::command_reader() {
        Ok(path) => path,
        Err(error_type) => {
            chess_error::print_error_messages(error_type);
            return ();
        }
    };

    let file = ChessBoardLoader::open_file(&file_name);

    let mut matrix =['-'; 64];

    ChessBoardLoader::read_file(&file, &mut matrix);

    println!("{:?}", matrix);

    // match ChessEngine::recreate_future_moves(&matrix){
    //     Ok(_) => _,
    //     ChessError => print!("Hola")
    // } 

    if let Err(error) = ChessEngine::recreate_future_moves(&matrix) {
        chess_error::print_error_messages(error);
    }
}