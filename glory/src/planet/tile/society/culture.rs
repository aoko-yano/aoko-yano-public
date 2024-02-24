use std::collections::HashMap;

#[derive(Debug)]
pub struct Cultures {
    pub established_culture: HashMap<Culture, i32>,
}

#[derive(Debug)]
pub enum Culture {}