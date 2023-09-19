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
    let mut game = match Game::new(&conf) {
        Ok(game) => game,
        Err(e) => {
            println!("Error: {}", e.message());
            return;
        }
    };

    match game.denotate_bomb(conf.x as u32, conf.y as u32) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e.message());
            return;
        }
    };

    match game.save_game(&conf.path_output) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e.message()),
    }
}
