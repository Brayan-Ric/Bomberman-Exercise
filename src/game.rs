use std::any::Any;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{BufWriter, Write};

use crate::constants::{DOWN, EMPTY_SQUARE, LEFT, RIGHT, UP};
use crate::coordinate::{self, Coordinate};
use crate::file_io;
use crate::item::Item;
use crate::{config::Config, error::BombermanError};

type Displacement = fn(&Coordinate) -> Option<Coordinate>;
type Expansion = fn(&HashMap<Coordinate, Item>, &Coordinate, Displacement) -> Option<Displacement>;

#[derive(Debug)]
pub struct Game {
    map: HashMap<Coordinate, Item>,
    map_dimension: u32,
}

impl Game {
    pub fn new(conf: &Config) -> Result<Game, BombermanError> {
        let mut map: HashMap<Coordinate, Item> = HashMap::new();
        let map_dimension = match file_io::get_matrix_dimensions(&conf.name_input) {
            Some(dimension) => dimension as u32,
            None => return Err(BombermanError::NonSquareBoardError),
        };
        file_io::read_input(&conf.name_input, map_dimension, process_line, &mut map)?;

        Ok(Game {
            map,
            map_dimension, // CHANGE THIS
        })
    }

    pub fn denotate_bomb(&mut self, x: u32, y: u32) -> Result<(), BombermanError> {
        // let map_rc = Rc::new(RefCell::new(self.map.clone()));
        let bomb_detonate = Coordinate::new(x, y, self.map_dimension);

        let affected = match self.map.get(&bomb_detonate) {
            Some(Item::NormalBomb(range)) => {
                detonate_explosion_2(&self.map, &bomb_detonate, *range, normal_bomb_effect)
            }
            Some(Item::TransferBomb(range)) => {
                detonate_explosion_2(&self.map, &bomb_detonate, *range, normal_transfer_effect)
            }
            _ => return Err(BombermanError::InvalidBombCoordinate),
        };
        self.update_damage(&affected);
        Ok(())
    }

    pub fn update_damage(&mut self, affected: &HashMap<Coordinate, u32>) {
        for (coordinate, damage) in affected {
            match self.map.get(coordinate) {
                Some(Item::Enemy(life)) => {
                    if *life <= *damage {
                        self.map.remove(coordinate);
                    } else {
                        self.map.insert(*coordinate, Item::Enemy(life - damage));
                    }
                }
                Some(Item::NormalBomb(_)) => {
                    self.map.remove(coordinate);
                }
                Some(Item::TransferBomb(_)) => {
                    self.map.remove(coordinate);
                }
                _ => (),
            }
        }
    }

    pub fn save_game(&self, path: &str) -> Result<(), BombermanError> {
        let file = file_io::open_file_for_writing(path)?;
        let mut writer = BufWriter::new(file);
        for x in 0..self.map_dimension {
            for y in 0..self.map_dimension {
                let key = Coordinate::new(x, y, self.map_dimension);
                let value = self.map.get(&key).unwrap_or(&Item::Empty);
                write_item(&mut writer, value)?;
            }
            let _ = writer.write_all(b"\n");
        }
        Ok(())
    }
}

fn write_item(writer: &mut BufWriter<std::fs::File>, value: &Item) -> Result<(), BombermanError> {
    match writer.write_all(format!("{} ", value).as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err(BombermanError::Write),
    };
    match writer.flush() {
        Ok(_) => Ok(()),
        Err(_) => Err(BombermanError::Write),
    }
}

fn process_generic_ptr(
    ptr: &mut dyn Any,
) -> Result<&mut HashMap<Coordinate, Item>, BombermanError> {
    match ptr.downcast_mut::<HashMap<Coordinate, Item>>() {
        Some(h) => Ok(h),
        None => Err(BombermanError::InvalidCoordinate),
    }
}

pub fn process_line(
    line: &String,
    row: usize,
    max_value: u32,
    ptr: &mut dyn Any,
) -> Result<(), BombermanError> {
    let hash: &mut HashMap<Coordinate, Item> = process_generic_ptr(ptr)?;

    let words = line.split_whitespace();

    for (y, s) in words.enumerate() {
        if s == EMPTY_SQUARE {
            continue;
        }
        hash.insert(
            Coordinate::new(row as u32, y as u32, max_value),
            Item::new(s)?,
        );
    }
    Ok(())
}

fn normal_bomb_effect(
    map: &HashMap<Coordinate, Item>,
    coordinate: &Coordinate,
    f: Displacement,
) -> Option<Displacement> {
    match map.get(coordinate).unwrap_or(&Item::Empty) {
        Item::Rock | Item::Wall => None,
        Item::Deflection(UP) => Some(coordinate::Coordinate::up),
        Item::Deflection(DOWN) => Some(coordinate::Coordinate::down),
        Item::Deflection(LEFT) => Some(coordinate::Coordinate::left),
        Item::Deflection(RIGHT) => Some(coordinate::Coordinate::right),
        _ => Some(f),
    }
}

fn normal_transfer_effect(
    map: &HashMap<Coordinate, Item>,
    coordinate: &Coordinate,
    f: Displacement,
) -> Option<Displacement> {
    match map.get(coordinate).unwrap_or(&Item::Empty) {
        Item::Wall => None,
        Item::Deflection(UP) => Some(coordinate::Coordinate::up),
        Item::Deflection(DOWN) => Some(coordinate::Coordinate::down),
        Item::Deflection(LEFT) => Some(coordinate::Coordinate::left),
        Item::Deflection(RIGHT) => Some(coordinate::Coordinate::right),
        _ => Some(f),
    }
}
fn detonate_explosion_2(
    map: &HashMap<Coordinate, Item>,
    bomb: &Coordinate,
    range: u32,
    g: Expansion,
) -> HashMap<Coordinate, u32> {
    let mut affected: HashMap<Coordinate, u32> = HashMap::new();
    let mut detonated_bombs: HashSet<Coordinate> = HashSet::new();
    detonate_explosion(map, &mut affected, bomb, range, &mut detonated_bombs, g);
    affected
}

fn detonate_explosion(
    map: &HashMap<Coordinate, Item>,
    affected: &mut HashMap<Coordinate, u32>,
    bomb: &Coordinate,
    range: u32,
    detonated_bombs: &mut HashSet<Coordinate>,
    g: Expansion,
) {
    if range == 0 {
        return;
    }

    // let mut affected: HashMap<Coordinate, u32> = HashMap::new();
    detonated_bombs.insert(*bomb);

    expansive_wave(
        map,
        affected,
        bomb,
        range + 1,
        detonated_bombs,
        coordinate::Coordinate::right,
        g,
    );
    expansive_wave(
        map,
        affected,
        bomb,
        range + 1,
        detonated_bombs,
        coordinate::Coordinate::left,
        g,
    );
    expansive_wave(
        map,
        affected,
        bomb,
        range + 1,
        detonated_bombs,
        coordinate::Coordinate::up,
        g,
    );
    expansive_wave(
        map,
        affected,
        bomb,
        range + 1,
        detonated_bombs,
        coordinate::Coordinate::down,
        g,
    );
    // *affected
}

fn expansive_wave(
    map: &HashMap<Coordinate, Item>,
    affected: &mut HashMap<Coordinate, u32>,
    coordinate: &Coordinate,
    range: u32,
    detonated_bombs: &mut HashSet<Coordinate>,
    f: Displacement,
    g: Expansion,
) {
    if range == 0 {
        return;
    }
    *affected.entry(*coordinate).or_insert(0) += 1;

    // if !detonated_bombs.contains(coordinate) {
    //     return;
    // }
    let f = match g(map, coordinate, f) {
        Some(f) => f,
        None => return,
    };
    let prox_coordinate = match f(coordinate) {
        Some(coordinate) => coordinate,
        None => return,
    };
    expansive_wave(
        map,
        affected,
        &prox_coordinate,
        range - 1,
        detonated_bombs,
        f,
        g,
    )
}
