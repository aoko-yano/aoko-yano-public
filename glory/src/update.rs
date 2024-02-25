use std::collections::HashMap;
use piston_window::Position;
use crate::data::{Data, Status};
use crate::data::planet::Planet;
use crate::data::planet::tile::society::technology::{Technologies, Technology};
use crate::data::planet::tile::Tile;
use crate::data::planet::tile::environment::biological_resource::BiologicalResource;
use crate::data::planet::tile::environment::Environment;
use crate::data::planet::tile::environment::natural_resource::NaturalResources;
use crate::data::planet::tile::environment::nature::Nature;
use crate::data::planet::tile::society::{Population, Society};
use crate::data::planet::tile::society::culture::Cultures;
use crate::piston::KeysState;

pub fn create_data(x: usize, y: usize) -> Data {
    Data { status: Status::Continue, history: vec!{ create_planet(x, y) } }
}

fn create_planet(x: usize, y: usize) -> Planet {
    let mut tiles = Vec::<Vec<Tile>>::new();
    for i in 0..y {
        let mut line = Vec::<Tile>::new();
        for j in 0..x {
            line.push(create_tile(i, j));
        }
        tiles.push(line);
    }
    Planet { tiles }
}

fn create_tile(x: usize, y: usize) -> Tile {
    let pos_x = x as i32;
    let pos_y = y as i32;
    Tile {
        society: create_empty_society(),
        environment: create_empty_environment(),
        position: Position{ x: pos_x, y: pos_y } }
}

fn create_empty_society() -> Society {
    Society {
        population: Population { number: 0 },
        cultures: Cultures { established_culture: Default::default() },
        technologies: Technologies { established_technology: HashMap::new() }
    }
}

fn create_empty_environment() -> Environment {
    Environment {
        biological_resource: BiologicalResource { living_species: Default::default() },
        natural_resources: NaturalResources { existing_natural_resource: Default::default() },
        nature: Nature {},
    }
}

pub fn put_people_to_data(data: &mut Data, position: &Position, number: usize) {
    let last_index = data.history.len() - 1;
    put_people_to_planet(data.history.get_mut(last_index).unwrap(), position, number);
}

fn put_people_to_planet(planet: &mut Planet, position: &Position, number: usize) {
    let x = position.x as usize;
    let y = position.y as usize;
    put_people_to_tile(planet.tiles.get_mut(y).unwrap().get_mut(x).unwrap(), number);
}

fn put_people_to_tile(tile: &mut Tile, number: usize) {
    put_people_to_society(&mut tile.society, number);
}

fn put_people_to_society(society: &mut Society, number: usize) {
    put_people(&mut society.population, number);
}

fn put_people(population: &mut Population, number: usize) {
    population.number += number;
}

pub fn update_data(data: &mut Data, _key: &KeysState) {
    data.history.push(create_planet_from_past_state(data.history.last().unwrap()));
    if data.history.last().unwrap().is_still_survives() {
        data.status = Status::GameOver
    }
    if data.history.last().unwrap().is_finished() {
        data.status = Status::GameClear
    }
    data.status = Status::Continue
}

fn create_planet_from_past_state(past: &Planet) -> Planet {
    let mut present = past.clone();
    for (_y, line) in present.tiles.iter_mut().enumerate() {
        for (_x, tile) in line.iter_mut().enumerate() {
            update_from_past_state(tile, &past.tiles);
        }
    }
    present
}

fn update_from_past_state(tile: &mut Tile, past_tiles: &Vec<Vec<Tile>>) {
    let x = tile.position.x as usize;
    let y = tile.position.y as usize;
    let past_tile = past_tiles.get(y).unwrap().get(x).unwrap();
    upgrade_technology(tile, past_tile);
}

fn upgrade_technology(tile: &mut Tile, past: &Tile) {
    let past_population = past.society.population.number;
    let past_technology = &past.society.technologies.established_technology;
    let present_technology = &mut tile.society.technologies.established_technology;
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