mod tile;

use std::vec::Vec;

#[derive(Debug)]
pub struct Planet {
    pub tiles: Vec<tile::Tile>,
}

impl Planet {
    pub fn put_first_people(&mut self) {
        self.tiles.get_mut(0).unwrap().put_people(1);
    }

    pub fn tick(&mut self) {
        for tile in &mut self.tiles {
            tile.tick();
        }
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

pub fn create_planet() -> Planet {
    Planet { tiles: vec!{ tile::create_tile() } }
}