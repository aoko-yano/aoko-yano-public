mod tile;

use std::clone::Clone;
use std::iter::Iterator;
use std::vec::Vec;
use crate::planet::tile::Tile;

#[derive(Clone, Debug)]
pub struct Planet {
    pub tiles: Vec<Vec<Tile>>,
}

impl Planet {
    pub fn put_first_people(&mut self, x: usize, y: usize) {
        self.tiles.get_mut(y).unwrap().get_mut(x).unwrap().put_people(1);
    }

    pub fn is_still_survives(&self) -> bool {
        for line in &self.tiles {
            for tile in line {
                if tile.is_still_survive() {
                    return true
                }
            }
        }
        false
    }

    pub fn is_finished(&self) -> bool {
        for line in &self.tiles {
            for tile in line {
                if tile.is_finished() {
                    return true
                }
            }
        }
        false
    }
}

pub fn create_planet(x: usize, y: usize) -> Planet {
    let mut tiles = Vec::<Vec<Tile>>::new();
    for i in 0..y {
        let mut line = Vec::<Tile>::new();
        for j in 0..x {
            line.push(tile::create_tile(i, j));
        }
        tiles.push(line);
    }
    Planet { tiles }
}

pub fn create_planet_from_past_state(past: &Planet) -> Planet {
    let mut present = past.clone();
    for (_y, line) in present.tiles.iter_mut().enumerate() {
        for (_x, tile) in line.iter_mut().enumerate() {
            tile.update_from_past_state(&past.tiles);
        }
    }
    present
}