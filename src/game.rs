use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::rc::Rc;

use crate::constants::EMPTY_SQUARE;
use crate::coordinate::{self, Coordinate};
use crate::file_io;
use crate::item::Item;
use crate::{config::Config, error::BombermanError};

type Operacion = fn(&Coordinate) -> Coordinate;

#[derive(Debug)]
pub struct Game {
    map: HashMap<Coordinate, Item>,
    map_dimension: i32,
}

impl Game {
    pub fn new(conf: &Config) -> Result<Game, BombermanError> {
        let mut map: HashMap<Coordinate, Item> = HashMap::new();
        file_io::read_input(&conf.name_input, process_line, &mut map)?;

        Ok(Game {
            map,
            map_dimension: 7, // CHANGE THIS
        })
    }

    pub fn denotate_bomb(&mut self, x: u32, y: u32) -> Result<(), BombermanError> {
        let map_rc = Rc::new(RefCell::new(&mut self.map));
        let bomb_detonate = Coordinate::new(x, y);

        match map_rc.clone().borrow().get(&bomb_detonate) {
            Some(Item::NormalBomb(range)) => {
                detonate_explosion(map_rc.clone(), &bomb_detonate, *range);
                return Ok(());
            }
            _ => return Err(BombermanError::InvalidBombCoordinate),
        }
    }

    pub fn save_game(&self, path: &String) -> Result<(), BombermanError> {
        let file = file_io::open_file_for_writing(path)?;
        let mut writer = BufWriter::new(file);
        for x in 0..self.map_dimension {
            for y in 0..self.map_dimension {
                let key = Coordinate::new(x as u32, y as u32);
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
        Err(_) => return Err(BombermanError::Write),
    }
}

// "B2 R R _ F1 _ _" -> [B2, R, R, _, F1, _, _]
fn process_generic_ptr(
    ptr: &mut dyn Any,
) -> Result<&mut HashMap<Coordinate, Item>, BombermanError> {
    match ptr.downcast_mut::<HashMap<Coordinate, Item>>() {
        Some(h) => Ok(h),
        None => Err(BombermanError::InvalidCoordinate),
    }
}

pub fn process_line(line: &String, row: usize, ptr: &mut dyn Any) -> Result<(), BombermanError> {
    let hash: &mut HashMap<Coordinate, Item> = process_generic_ptr(ptr)?;

    let words = line.trim().split_whitespace();

    for (y, s) in words.enumerate() {
        if s == EMPTY_SQUARE {
            continue;
        }
        hash.insert(Coordinate::new(row as u32, y as u32), Item::new(s)?);
    }
    Ok(())
}

// fn validate_bomb(map: &HashMap<Coordinate, Item>, bomb_detonate: &Coordinate) -> bool {
//     match map.get(bomb_detonate) {
//         Some(&Item::NormalBomb(_)) | Some(&Item::TransferBomb(_)) => true,
//         _ => false,
//     }
// }

fn normal_bomb_effect(
    map: &mut HashMap<Coordinate, Item>,
    coordinate: &Coordinate,
    f: Operacion,
) -> Operacion {
    match map.get(coordinate).unwrap_or(&Item::Empty) {
        Item::Enemy(life) => {
            if *life == 1 {
                map.remove(coordinate);
            } else {
                map.insert(*coordinate, Item::Enemy(life - 1));
            }
        }
        Item::Deflection(_) => {
            map.remove(coordinate);
        }
        Item::NormalBomb(_) => {
            map.remove(coordinate);
            // denotate_bomb(coordinate.x, coordinate.y);
        }
        Item::TransferBomb(_) => {
            map.remove(coordinate);
            // denotate_bomb(coordinate.x, coordinate.y);
        }
        _ => (),
    };
    f
}
fn detonate_explosion(map: Rc<RefCell<&mut HashMap<Coordinate, Item>>>, coordinate: &Coordinate, range: u32) {
    // let map_rc = Rc::new(RefCell::new(map));
    let map_right = map.clone();
    let map_left = map.clone();
    let map_up = map.clone();
    let map_down = map.clone();
    _detonate_explosion(map_right, coordinate, range,coordinate::Coordinate::right);
    _detonate_explosion(map_left, coordinate, range, coordinate::Coordinate::left);
    _detonate_explosion(map_up, coordinate, range, coordinate::Coordinate::up);
    _detonate_explosion(map_down, coordinate, range, coordinate::Coordinate::down);
}

fn _detonate_explosion(map: Rc<RefCell<&mut HashMap<Coordinate, Item>>>, coordinate: &Coordinate, range: u32,f: Operacion){
    if range == 0 {
        return;
    }
    let mut map_2 = map.borrow_mut();
    let coordinate = f(&coordinate);
    let f = normal_bomb_effect(&mut map_2, &coordinate, f);
    _detonate_explosion(map.clone(), &coordinate, range - 1,f);
}