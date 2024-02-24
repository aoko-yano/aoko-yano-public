mod society;
mod environment;

use crate::planet::tile::society::technology::Technology;

#[derive(Debug)]
pub struct Tile {
    society: society::Society,
    environment: environment::Environment,
}

impl Tile {
    pub fn put_people(&mut self, number: i32) {
        self.society.put_people(number);
    }

    pub fn tick(&mut self) {
        self.upgrade_technology();
    }

    fn upgrade_technology(&mut self) {
        if self.society.population.number > 0 {
            if self.society.technologies.established_technology.contains_key(&Technology::Primitive) {
                if self.society.technologies.established_technology[&Technology::Primitive] < 10 {
                    *self.society.technologies.established_technology.entry(Technology::Primitive).or_insert(0) += 1;
                } else {
                    self.society.technologies.established_technology.insert(Technology::Developed, 0);
                }
            } else {
                self.society.technologies.established_technology.insert(Technology::Primitive, 0);
            }
        } else {
            self.society.technologies.established_technology.clear();
        }
    }

    pub fn is_still_survive(&self) -> bool {
        self.society.population.number > 0
    }

    pub fn is_finished(&self) -> bool {
        self.society.technologies.established_technology.contains_key(&Technology::Developed)
    }
}

pub fn create_tile() -> Tile {
    Tile {
        society: society::create_empty_society(),
        environment: environment::create_empty_environment() }
}