use std::collections::HashMap;
use std::convert::From;
use std::default::Default;

#[derive(Debug)]
struct Tile {
    society: Society,
    environment: Environment,
}

#[derive(Debug)]
struct Society {
    population: Population,
    cultures: Cultures,
    technologies: Technologies,
}

#[derive(Debug)]
struct Population {
    number: i32,
}

#[derive(Debug)]
struct Cultures {
    established_culture: HashMap<Culture, i32>,
}

#[derive(Debug)]
enum Culture {}

#[derive(Debug)]
struct Technologies {
    established_technology: HashMap<Technology, i32>,
}

#[derive(Debug)]
enum Technology {
    Primitive,
    Developed
}

#[derive(Debug)]
struct Environment {
    biological_resource: BiologicalResource,
    natural_resources: NaturalResources,
    nature: Nature,
}

#[derive(Debug)]
struct BiologicalResource {
    living_species: HashMap<Species, i32>
}

#[derive(Debug)]
enum Species {}

#[derive(Debug)]
struct NaturalResources {
    existing_natural_resource: HashMap<NaturalResource, i32>,
}

#[derive(Debug)]
enum NaturalResource {}

#[derive(Debug)]
struct Nature {}

fn main() {
    let tile = Tile{ society: Society {
        population: Population { number: 0 },
        cultures: Cultures { established_culture: Default::default() },
        technologies: Technologies { established_technology: HashMap::from([(Technology::Primitive,0)]) },
    }, environment: Environment {
        biological_resource: BiologicalResource { living_species: Default::default() },
        natural_resources: NaturalResources { existing_natural_resource: Default::default() },
        nature: Nature {},
    } };
    println!("{:?}", tile);
}
