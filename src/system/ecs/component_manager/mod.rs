pub mod component;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use component::Componentable;

use crate::util::hash;

#[derive(Serialize, Deserialize)]
pub struct ComponentManager {
    indices: HashMap<u64, usize>,
    components: Vec<Box<dyn Componentable>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        let indices = HashMap::new();
        let components = Vec::new();

        Self {
            indices,
            components,
        }
    }

    pub fn add<T: Componentable>(&mut self, c: Box<dyn Componentable>) -> bool {
        let index = self.components.len();
        let hash = hash::hasher(&String::from(std::any::type_name::<T>()));
        self.indices.insert(hash, index);
        self.components.push(c);

        return true
    }

    pub fn get<T: Componentable + 'static>(&self) -> Option<&T> {
        let hash = hash::hasher(&String::from(std::any::type_name::<T>()));
        let index = *self.indices.get(&hash)?;
        let c = self.components.get(index)?;
        let c = c.as_any().downcast_ref::<T>()?;

        return Some(c)
    }
}