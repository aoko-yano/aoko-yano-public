mod tile;

use std::clone::Clone;
use std::iter::Iterator;
use std::vec::Vec;
use crate::planet::tile::Tile;

#[derive(Clone, Debug)]
pub struct Planet {
    pub tiles: Vec<Tile>,
}

impl Planet {
    pub fn put_first_people(&mut self, x: usize) {
        self.tiles.get_mut(x).unwrap().put_people(1);
    }

    pub fn is_still_survives(&self) -> bool {
        for tile in &self.tiles {
            if tile.is_still_survive() {
                return true
            }
        }
        false
    }

    pub fn is_finished(&self) -> bool {
        for tile in &self.tiles {
            if tile.is_finished() {
                return true
            }
        }
        false
    }
}

pub fn create_planet(x: usize) -> Planet {
    let mut tiles = Vec::<Tile>::new();
    for i in 0..x {
        tiles.push(tile::create_tile(i));
    }
    Planet { tiles }
}

pub fn create_planet_from_past_state(past: &Planet) -> Planet {
    let mut present = past.clone();
    for (x, tile) in present.tiles.iter_mut().enumerate() {
        tile.update_from_past_state(&past.tiles, x);
    }
    present
}