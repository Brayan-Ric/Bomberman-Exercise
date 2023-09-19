use std::{fs::File, io::Read};

use bomberman::{config::Config, error::BombermanError, game::Game};

fn simulator_game(
    name_input: String,
    path_output: String,
    x: usize,
    y: usize,
) -> Result<(), BombermanError> {
    let conf = Config {
        name_input,
        path_output: path_output.clone(),
        x,
        y,
    };
    let mut game = match Game::new(&conf) {
        Ok(game) => game,
        Err(e) => return Ok(e.send(path_output)),
    };

    match game.denotate_bomb(conf.x as u32, conf.y as u32) {
        Ok(_) => (),
        Err(e) => return Ok(e.send(path_output)),
    };

    match game.save_game(&conf.path_output) {
        Ok(_) => Ok(()),
        Err(e) => return Ok(e.send(path_output)),
    }
}

fn compare_files(file_path1: &str, file_path2: &str) -> bool {
    let mut file1 = match File::open(file_path1) {
        Ok(file) => file,
        Err(_) => return false,
    };
    let mut file2 = match File::open(file_path2) {
        Ok(file) => file,
        Err(_) => return false,
    };

    let mut buffer1 = Vec::new();
    let mut buffer2 = Vec::new();

    match file1.read_to_end(&mut buffer1) {
        Ok(_) => (),
        Err(_) => return false,
    };
    match file2.read_to_end(&mut buffer2) {
        Ok(_) => (),
        Err(_) => return false,
    };

    buffer1 == buffer2
}

#[test]
fn test_input_file_not_found() {
    let name_input = "file_not_found.txt".to_string();
    let path_output = "./tests/outputs/file_not_found.txt".to_string();
    let path_expected_output = "./tests/expected_output/file_not_found.txt".to_string();
    let _ = simulator_game(name_input.clone(), path_output.clone(), 0, 0);
    assert!(compare_files(&path_output, &path_expected_output));
}
#[test]
fn test_no_bomb_at_coordinates() {
    let name_input = "./tests/inputs/no_bomb_at_coordinates.txt".to_string();
    let path_output = "./tests/outputs/no_bomb_at_coordinates.txt".to_string();
    let path_expected_output = "./tests/expected_output/no_bomb_at_coordinates.txt".to_string();
    let _ = simulator_game(name_input.clone(), path_output.clone(), 0, 0);
    assert!(compare_files(&path_output, &path_expected_output));
}

#[test]
fn test_unrecognized_item() {
    let name_input = "./tests/inputs/unrecognized_item.txt".to_string();
    let path_output = "./tests/outputs/unrecognized_item.txt".to_string();
    let path_expected_output = "./tests/expected_output/unrecognized_item.txt".to_string();
    let _ = simulator_game(name_input.clone(), path_output.clone(), 0, 0);
    assert!(compare_files(&path_output, &path_expected_output));
}

#[test]
fn test_non_square_matrix() {
    let name_input = "./tests/inputs/non_square_matrix.txt".to_string();
    let path_output = "./tests/outputs/non_square_matrix.txt".to_string();
    let path_expected_output = "./tests/expected_output/non_square_matrix.txt".to_string();
    let _ = simulator_game(name_input.clone(), path_output.clone(), 0, 0);
    assert!(compare_files(&path_output, &path_expected_output));
}

#[test]
fn test_catedra_1() {
    let name_input = "./tests/inputs/catedra_1.txt".to_string();
    let path_output = "./tests/outputs/catedra_1.txt".to_string();
    let path_expected_output = "./tests/expected_output/catedra_1.txt".to_string();
    let _ = simulator_game(name_input.clone(), path_output.clone(), 0, 0);
    assert!(compare_files(&path_output, &path_expected_output));
}

#[test]
fn test_catedra_2() {
    let name_input = "./tests/inputs/catedra_2.txt".to_string();
    let path_output = "./tests/outputs/catedra_2.txt".to_string();
    let path_expected_output = "./tests/expected_output/catedra_2.txt".to_string();
    let _ = simulator_game(name_input.clone(), path_output.clone(), 4, 2);
    assert!(compare_files(&path_output, &path_expected_output));
}

#[test]
fn test_catedra_3() {
    let name_input = "./tests/inputs/catedra_3.txt".to_string();
    let path_output = "./tests/outputs/catedra_3.txt".to_string();
    let path_expected_output = "./tests/expected_output/catedra_3.txt".to_string();
    let _ = simulator_game(name_input.clone(), path_output.clone(), 4, 0);
    assert!(compare_files(&path_output, &path_expected_output));
}

#[test]
fn test_down_deviation() {
    let name_input = "./tests/inputs/deviation_down.txt".to_string();
    let path_output = "./tests/outputs/deviation_down.txt".to_string();
    let path_expected_output = "./tests/expected_output/deviation_down.txt".to_string();
    let _ = simulator_game(name_input.clone(), path_output.clone(), 0, 1);
    assert!(compare_files(&path_output, &path_expected_output));
}

#[test]
fn test_up_deviation() {
    let name_input = "./tests/inputs/deviation_up.txt".to_string();
    let path_output = "./tests/outputs/deviation_up.txt".to_string();
    let path_expected_output = "./tests/expected_output/deviation_up.txt".to_string();
    let _ = simulator_game(name_input.clone(), path_output.clone(), 2, 1);
    assert!(compare_files(&path_output, &path_expected_output));
}

#[test]
fn test_right_deviation() {
    let name_input = "./tests/inputs/deviation_right.txt".to_string();
    let path_output = "./tests/outputs/deviation_right.txt".to_string();
    let path_expected_output = "./tests/expected_output/deviation_right.txt".to_string();
    let _ = simulator_game(name_input.clone(), path_output.clone(), 0, 1);
    assert!(compare_files(&path_output, &path_expected_output));
}

#[test]
fn test_left_deviation() {
    let name_input = "./tests/inputs/deviation_left.txt".to_string();
    let path_output = "./tests/outputs/deviation_left.txt".to_string();
    let path_expected_output = "./tests/expected_output/deviation_left.txt".to_string();
    let _ = simulator_game(name_input.clone(), path_output.clone(), 0, 1);
    assert!(compare_files(&path_output, &path_expected_output));
}

#[test]
fn test_transfer_bomb() {
    let name_input = "./tests/inputs/transfer_bomb.txt".to_string();
    let path_output = "./tests/outputs/transfer_bomb.txt".to_string();
    let path_expected_output = "./tests/expected_output/transfer_bomb.txt".to_string();
    let _ = simulator_game(name_input.clone(), path_output.clone(), 0, 0);
    assert!(compare_files(&path_output, &path_expected_output));
}
