use std::collections::HashMap;

#[derive(Debug)]
pub struct NaturalResources {
    pub existing_natural_resource: HashMap<NaturalResource, i32>,
}

#[derive(Debug)]
pub enum NaturalResource {}