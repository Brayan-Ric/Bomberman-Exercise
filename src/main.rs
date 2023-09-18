use bomberman::config::Config;

use bomberman::game::Game;

fn main() {
    let conf = match Config::new() {
        Ok(conf) => conf,
        Err(e) => {
            println!("Error: {}", e.message());
            return;
        }
    };
    // file_io::process_line(&"B2 R R _ F1 _ _".to_string());
    let game = match Game::new(&conf) {
        Ok(game) => game,
        Err(e) => {
            println!("Error: {}", e.message());
            return;
        }
    };
    match game.save_game(&conf.path_output) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e.message());
            return;
        }
    }
}
