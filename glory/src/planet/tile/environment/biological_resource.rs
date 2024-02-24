use std::collections::HashMap;

#[derive(Debug)]
pub struct BiologicalResource {
    pub living_species: HashMap<Species, i32>
}

#[derive(Debug)]
pub enum Species {}