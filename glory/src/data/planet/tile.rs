pub mod society;
pub mod environment;

use std::vec::Vec;

use society::technology::Technology;

#[derive(Clone, Debug)]
pub struct Tile {
    society: society::Society,
    environment: environment::Environment,
    x: usize,
    y: usize,
}

impl Tile {
    pub fn put_people(&mut self, number: i32) {
        self.society.put_people(number);
    }

    pub fn update_from_past_state(&mut self, past_tiles: &Vec<Vec<Tile>>) {
        self.upgrade_technology(past_tiles.get(self.y).unwrap().get(self.x).unwrap());
    }

    fn upgrade_technology(&mut self, past: &Tile) {
        let past_population = past.society.population.number;
        let past_technology = &past.society.technologies.established_technology;
        let present_technology = &mut self.society.technologies.established_technology;
        if past_population > 0 {
            if past_technology.contains_key(&Technology::Primitive) {
                if past_technology[&Technology::Primitive] < 10 {
                    *present_technology.entry(Technology::Primitive).or_insert(0) += 1;
                } else {
                    present_technology.insert(Technology::Developed, 0);
                }
            } else {
                present_technology.insert(Technology::Primitive, 0);
            }
        } else {
            present_technology.clear();
        }
    }

    pub fn is_still_survive(&self) -> bool {
        self.society.population.number > 0
    }

    pub fn is_finished(&self) -> bool {
        self.society.technologies.established_technology.contains_key(&Technology::Developed)
    }
}

pub fn create_tile(x: usize, y: usize) -> Tile {
    Tile {
        society: society::create_empty_society(),
        environment: environment::create_empty_environment(),
        x, y, }
}