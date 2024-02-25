use piston_window::{Context, G2d};

pub mod planet;

pub enum Status {
    Continue,
    GameClear,
    GameOver,
}

pub struct Data {
    pub status: Status,
    pub history: Vec<planet::Planet>,
}

impl Data {
    pub fn draw(self: &Data, _c: Context, _g: &mut G2d) {
        // println!("================ Turn: {} ================", self.history.len());
        // println!("{:?}", &self.history.last().unwrap());
        // TODO: implementation.
    }
}