pub mod planet;

use crate::planet::{create_planet, create_planet_from_past_state};

fn main() {
    let mut history = vec!{ create_planet(1) };
    history.get_mut(0).unwrap().put_first_people(0);
    println!("{:?}", history.last());
    while history.last().unwrap().is_still_survives() {
        println!("================ Turn: {} ================", history.len());
        history.push(create_planet_from_past_state(history.last().unwrap()));
        println!("{:?}", &history.last().unwrap());
        if history.last().unwrap().is_finished() {
            break;
        }
    }
}
