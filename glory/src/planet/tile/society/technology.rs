use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Technologies {
    pub established_technology: HashMap<Technology, i32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Technology {
    Primitive,
    Developed
}