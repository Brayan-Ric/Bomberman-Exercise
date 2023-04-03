pub use ajedrez::chess_board_loader;
pub use ajedrez::chess_engine;
pub use ajedrez::chess_error;
pub use ajedrez::chess_error::ChessError;
pub use ajedrez::chess_syntax_validator;

fn main() {
    let file_name = match chess_board_loader::command_reader() {
        Ok(path) => path,
        Err(error_type) => {
            chess_error::print_error_messages(error_type);
            return ;
        }
    };

    let file = match chess_board_loader::open_file(&file_name) {
        Ok(file) => file,
        Err(error_type) => {
            chess_error::print_error_messages(error_type);
            return ;
        }
    };

    let mut matrix = ['-'; 64];

    match chess_board_loader::read_file(&file, &mut matrix) {
        Ok(()) => (),
        Err(error_type) => {
            chess_error::print_error_messages(error_type);
            return ;
        }
    };

    if let Err(error) = chess_engine::recreate_future_moves(&matrix) {
        chess_error::print_error_messages(error);
    }
}
