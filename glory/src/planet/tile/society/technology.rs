use std::collections::HashMap;

#[derive(Debug)]
pub struct Technologies {
    pub established_technology: HashMap<Technology, i32>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Technology {
    Primitive,
    Developed
}