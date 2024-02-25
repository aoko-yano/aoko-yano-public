pub mod biological_resource;
pub mod natural_resource;
pub mod nature;

use std::default::Default;

use biological_resource::BiologicalResource;
use natural_resource::NaturalResources;
use nature::Nature;

#[derive(Clone, Debug)]
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