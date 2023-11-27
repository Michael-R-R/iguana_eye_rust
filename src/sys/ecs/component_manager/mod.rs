pub mod component;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use component::Componentable;

#[derive(Serialize, Deserialize)]
pub struct ComponentManager {
    indices: HashMap<u64, usize>,
    components: Vec<Box<dyn Componentable>>,
}

impl ComponentManager {

}