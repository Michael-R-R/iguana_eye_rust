pub mod entity;
pub mod entity_manager;
pub mod component_manager;

use serde::{Serialize, Deserialize};
use entity::Entity;
use entity_manager::EntityManager;
use component_manager::ComponentManager;

#[derive(Serialize, Deserialize)]
pub struct ECS {
    entity_manager: EntityManager,
    component_manager: ComponentManager,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
        }
    }

    // TODO add functions
}