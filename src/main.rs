pub use ajedrez::ChessSyntaxValidator;
pub use ajedrez::ChessBoardLoader;
pub use ajedrez::ChessEngine;


fn main() {

    let file_name = ChessBoardLoader::command_reader();

    let file = ChessBoardLoader::open_file(&file_name);

    let mut matrix =['-'; 64];

    ChessBoardLoader::read_file(&file, &mut matrix);

    println!("{:?}", matrix);

    ChessEngine::recreate_future_moves(&matrix);

}