use std::{collections::{HashMap, HashSet}, u64};
use serde::{Serialize, Deserialize};

use super::entity::Entity;

#[derive(Serialize, Deserialize)]
pub struct EntityManager {
    entities: HashMap<Entity, HashSet<u64>>,
    free_ids: Vec<u64>,
    next_id: u64,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            free_ids: Vec::new(),
            next_id: 0
        }
    }

    pub fn create(&mut self) -> Entity {
        let mut entity = Entity::new(0);

        match self.free_ids.pop() {
            Some(id) => entity.id = id,
            None => {
                self.next_id += 1;
                entity.id = self.next_id;
                while self.does_exist(entity) {
                    self.next_id += 1;
                    entity.id = self.next_id;
                }
            }
        }

        self.entities.insert(entity, HashSet::new());

        return entity;
    }

    pub fn remove(&mut self, entity: Entity) -> bool {
        match self.entities.remove(&entity) {
            Some(_) => {
                self.free_ids.push(entity.id);
                return true
            },
            None => return false
        }
    }

    pub fn attach_component(&mut self, entity: Entity, component: u64) -> bool {
        match self.entities.get_mut(&entity) {
            Some(set) => {
                return set.insert(component)
            },
            None => return false
        }
    }

    pub fn detach_component(&mut self, entity: Entity, component: u64) -> bool {
        match self.entities.get_mut(&entity) {
            Some(set) => {
                return set.remove(&component)
            },
            None => return false
        }
    }

    pub fn has_component(&self, entity: Entity, component: u64) -> bool {
        match self.entities.get(&entity) {
            Some(set) => {
                return set.contains(&component)
            },
            None => return false
        }
    }

    pub fn get_attached(&self, entity: Entity) -> Option<&HashSet<u64>> {
        return self.entities.get(&entity)
    }

    pub fn does_exist(&self, entity: Entity) -> bool {
        return self.entities.contains_key(&entity);
    }

    pub fn count(&self) -> usize {
        return self.entities.len()
    }
}