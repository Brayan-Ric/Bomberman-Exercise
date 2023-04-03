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

    let file = match ChessBoardLoader::open_file(&file_name){
        Ok(file) => file,
        Err(error_type) => {
            chess_error::print_error_messages(error_type);
            return ();
        }
    };

    let mut matrix =['-'; 64];

    match ChessBoardLoader::read_file(&file, &mut matrix) {
        Ok(()) => (),
        Err(error_type) => {
            chess_error::print_error_messages(error_type);
            return ();
        },
    };


    if let Err(error) = ChessEngine::recreate_future_moves(&matrix) {
        chess_error::print_error_messages(error);
    }
}