pub mod entity;
pub mod entity_manager;
pub mod component_manager;

use serde::{Serialize, Deserialize};
use entity::Entity;
use entity_manager::EntityManager;

#[derive(Serialize, Deserialize)]
pub struct ECS {
    entity_manager: EntityManager,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
        }
    }
}