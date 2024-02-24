mod biological_resource;
mod natural_resource;
mod nature;

use std::default::Default;
use crate::planet::tile::environment::biological_resource::BiologicalResource;
use crate::planet::tile::environment::natural_resource::NaturalResources;
use crate::planet::tile::environment::nature::Nature;

#[derive(Debug)]
pub struct Environment {
    pub biological_resource: BiologicalResource,
    pub natural_resources: NaturalResources,
    pub nature: Nature,
}

pub fn create_empty_environment() -> Environment {
    Environment {
        biological_resource: BiologicalResource { living_species: Default::default() },
        natural_resources: NaturalResources { existing_natural_resource: Default::default() },
        nature: Nature {},
    }
}