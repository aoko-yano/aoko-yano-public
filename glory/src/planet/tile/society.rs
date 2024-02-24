pub(crate) mod technology;
mod culture;

use std::collections::HashMap;
use std::default::Default;

use crate::planet::tile::society::culture::Cultures;
use crate::planet::tile::society::technology::Technologies;

#[derive(Debug)]
pub struct Society {
    pub population: Population,
    pub cultures: Cultures,
    pub technologies: Technologies,
}

impl Society {
    pub fn put_people(&mut self, number: i32) {
        self.population.put_people(number);
    }
}

#[derive(Debug)]
pub struct Population {
    pub number: i32,
}

impl Population {
    pub fn put_people(&mut self, number: i32) {
        self.number += number;
    }
}


pub fn create_empty_society() -> Society {
    Society {
        population: Population { number: 0 },
        cultures: Cultures { established_culture: Default::default() },
        technologies: Technologies { established_technology: HashMap::new() }
    }
}