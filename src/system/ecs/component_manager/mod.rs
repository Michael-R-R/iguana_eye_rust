pub mod component;

use std::collections::{HashMap, HashSet};
use std::io::{Error, ErrorKind};
use serde::{Serialize, Deserialize};

use super::entity::Entity;
use component::{Componentable, hierarchy_component::HierarchyComponent};
use crate::{util::hash, system::game::Game, app::Viewport};

#[derive(Serialize, Deserialize)]
pub struct ComponentManager {
    indices: HashMap<u64, usize>,
    components: Vec<Box<dyn Componentable>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            indices: HashMap::new(),
            components: Vec::new(),
        }
    }

    pub fn handle_update(&mut self, dt: f32, game: &Game) {
        for c in self.components.iter_mut() {
            c.handle_update(dt, game);
        }
    }

    pub fn handle_render(&mut self, dt: f32, game: &Game, viewport: &Viewport) {
        for c in self.components.iter_mut() {
            c.handle_render(dt, game, viewport);
        }
    }

    pub fn add(&mut self, c: Box<dyn Componentable>) -> Result<(), Error> {
        let hash = c.get_hash();
        if self.indices.contains_key(&hash) {
            return Err(Error::new(ErrorKind::NotFound,
                "ERROR::ComponentManager::add()::already exist"))
        }

        let index = self.components.len();
        self.indices.insert(hash, index);
        self.components.push(c);

        return Ok(())
    }

    pub fn insert(
        &mut self, 
        index: usize, 
        c: Box<dyn Componentable>
    ) -> Result<(), Error> {

        let hash = c.get_hash();
        if self.indices.contains_key(&hash) {
            return Err(Error::new(ErrorKind::NotFound,
                "ERROR::ComponentManager::insert()::already exist"))
        }

        if index >= self.components.len() {
            return Err(Error::new(ErrorKind::NotFound,
                "ERROR::ComponentManager::insert()::out of bounds"))
        }

        self.indices.insert(hash, index);
        self.components.insert(index, c);
        self.update_indices(index);

        Ok(())
    }

    pub fn remove<T: Componentable>(&mut self) -> Result<(), Error> {
        let hash = ComponentManager::type_hash::<T>();
        if !self.indices.contains_key(&hash) {
            return Err(Error::new(ErrorKind::NotFound,
                "ERROR::ComponentManager::remove()::doesn't exist"))
        }

        let index = self.indices[&hash];
        let c = &self.components[index];
        if !c.is_empty() {
            return Err(Error::new(ErrorKind::NotFound,
                "ERROR::ComponentManager::remove()::component not empty"))
        }

        self.indices.remove(&hash);
        self.components.remove(index);
        self.update_indices(index);

        return Ok(())
    }

    pub fn has(&self, hash: u64) -> bool {
        return self.indices.contains_key(&hash)
    }

    pub fn attach(
        &mut self, 
        e: Entity,
        hash: u64
    ) -> Result<usize, Error> {

        match self.get_by_hash_mut(hash) {
            Some(c) => c.attach(e),
            None => {
                return Err(Error::new(ErrorKind::NotFound,
                    "ERROR::component_manager::attach()::cannot find component"));
            }
        }
    }

    pub fn detach(
        &mut self, 
        e: Entity,
        hash: u64
    ) -> Result<(), Error> {

        match self.get_by_hash_mut(hash) {
            Some(c) => c.detach(e),
            None => {
                return Err(Error::new(ErrorKind::NotFound,
                    "ERROR::component_manager::detach()::cannot find component"));
            }
        }
    }

    pub fn get<T: Componentable + 'static>(&self) -> Option<&T> {
        let hash = ComponentManager::type_hash::<T>();
        let index = *self.indices.get(&hash)?;
        let c = self.components.get(index)?;
        let c = c.as_any().downcast_ref::<T>()?;

        return Some(c)
    }

    pub fn get_mut<T: Componentable + 'static>(&mut self) -> Option<&mut T> {
        let hash = ComponentManager::type_hash::<T>();
        let index = *self.indices.get(&hash)?;
        let c = self.components.get_mut(index)?;
        let c = c.as_any_mut().downcast_mut::<T>()?;

        return Some(c)
    }

    pub fn get_by_hash(&self, hash: u64) -> Option<&dyn Componentable> {
        let index = *self.indices.get(&hash)?;
        let c = self.components.get(index)?;
        return Some(c.as_ref());
    }

    pub fn get_by_hash_mut(&mut self, hash: u64) -> Option<&mut dyn Componentable> {
        let index = *self.indices.get(&hash)?;
        let c = self.components.get_mut(index)?;
        return Some(c.as_mut());
    }

    pub fn find_index<T: Componentable>(&self) -> Option<usize> {
        let hash = ComponentManager::type_hash::<T>();
        Some(*self.indices.get(&hash)?)
    }

    pub fn purge_entity(&mut self, e: Entity, hash_list: &HashSet<u64>) -> Result<(), Error> {
        self.has_children(e)?;

        for hash in hash_list {
            if let Some(c) = self.get_by_hash_mut(*hash) {
                c.detach(e)?;
            }
        }

        Ok(())
    }

    fn has_children(&self, e: Entity) -> Result<(), Error> {
        match self.get::<HierarchyComponent>() {
            Some(hc) => {
                match hc.component.find_index(&e) {
                    Some(index) => {
                        if let Some(children) = hc.get_children(index) {
                            if !children.is_empty() {
                                return Err(Error::new(ErrorKind::Other,
                                    "ERROR::component_manager::has_children()::children list not empty"))
                            }
                        }
                    },
                    None => {
                        return Err(Error::new(ErrorKind::NotFound,
                            "ERROR::component_manager::has_children()::cannot find index"))
                    }
                }
            },
            None => {
                return Err(Error::new(ErrorKind::NotFound,
                    "ERROR::component_manager::has_children()::cannot find hierarchy component"))
            }
        }

        Ok(())
    }

    fn update_indices(&mut self, start: usize) {
        for i in start..self.components.len() {
            let c = &self.components[i];
            self.indices.insert(c.get_hash(), i);
        }
    }

    pub fn type_hash<T: Componentable>() -> u64 {
        hash::get(&String::from(std::any::type_name::<T>()))
    }
}