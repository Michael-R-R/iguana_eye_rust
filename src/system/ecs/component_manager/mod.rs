pub mod component;

use std::collections::HashMap;
use std::io;
use serde::{Serialize, Deserialize};

use component::Componentable;

use crate::util::hash;

use self::component::name_component::NameComponent;

#[derive(Serialize, Deserialize)]
pub struct ComponentManager {
    indices: HashMap<u64, usize>,
    components: Vec<Box<dyn Componentable>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        let mut obj = Self {
            indices: HashMap::new(),
            components: Vec::new(),
        };

        _ = obj.add(Box::new(NameComponent::new()));

        return obj
    }

    pub fn add(&mut self, c: Box<dyn Componentable>) -> Result<(), io::Error> {
        let hash = c.get_hash();
        if self.indices.contains_key(&hash) {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                "ERROR::ComponentManager::add()::already exist"))
        }

        let index = self.components.len();
        self.indices.insert(hash, index);
        self.components.push(c);

        return Ok(())
    }

    pub fn insert<T: Componentable>(
        &mut self, 
        index: usize, 
        c: Box<dyn Componentable>
    ) -> Result<(), io::Error> {

        let hash = c.get_hash();
        if self.indices.contains_key(&hash) {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                "ERROR::ComponentManager::insert()::already exist"))
        }

        if index >= self.components.len() {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                "ERROR::ComponentManager::insert()::out of bounds"))
        }

        self.indices.insert(hash, index);
        self.components.insert(index, c);
        self.update_indices(index);

        Ok(())
    }

    pub fn remove<T: Componentable>(&mut self) -> Result<(), std::io::Error> {
        let hash = hash::get(&String::from(std::any::type_name::<T>()));
        if !self.indices.contains_key(&hash) {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                "ERROR::ComponentManager::remove()::doesn't exist"))
        }

        let index = self.indices[&hash];
        let c = &self.components[index];
        if c.is_empty() {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                "ERROR::ComponentManager::remove()::component not empty"))
        }

        self.indices.remove(&hash);
        self.components.remove(index);
        self.update_indices(index);

        return Ok(())
    }

    pub fn get<T: Componentable + 'static>(&self) -> Option<&T> {
        let hash = hash::get(&String::from(std::any::type_name::<T>()));
        let index = *self.indices.get(&hash)?;
        let c = self.components.get(index)?;
        let c = c.as_any().downcast_ref::<T>()?;

        return Some(c)
    }

    pub fn has<T: Componentable>(&self) -> bool {
        let hash = hash::get(&String::from(std::any::type_name::<T>()));
        return self.indices.contains_key(&hash)
    }

    fn update_indices(&mut self, start: usize) {
        for i in start..self.components.len() {
            let c = &self.components[i];
            self.indices.insert(c.get_hash(), i);
        }
    }
}