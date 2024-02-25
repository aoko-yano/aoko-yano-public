use piston_window::{Context, G2d};

use crate::data::planet::{create_planet, create_planet_from_past_state, Planet};

pub mod planet;

enum Status {
    Continue,
    GameClear,
    GameOver,
}

pub struct Data {
    status: Status,
    history: Vec<Planet>,
}

impl Data {
    pub fn put_first_people(self: &mut Data, x: usize, y: usize) {
        self.history.get_mut(0).unwrap().put_first_people(x, y);
    }
    pub fn draw(self: &Data, _c: Context, _g: &mut G2d) {
        // println!("================ Turn: {} ================", self.history.len());
        // println!("{:?}", &self.history.last().unwrap());
        // TODO: implementation.
    }
    pub fn update(self: &mut Data) {
        self.history.push(create_planet_from_past_state(self.history.last().unwrap()));
        if self.history.last().unwrap().is_still_survives() {
            self.status = Status::GameOver
        }
        if self.history.last().unwrap().is_finished() {
            self.status = Status::GameClear
        }
        self.status = Status::Continue
    }
}

pub fn create_data(x: usize, y: usize) -> Data {
    Data { status: Status::Continue, history: vec!{ create_planet(x, y) } }
}