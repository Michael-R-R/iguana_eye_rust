pub mod entity;
pub mod entity_manager;
pub mod component_manager;

use std::collections::HashSet;
use std::io::{Error, ErrorKind};

use serde::{Serialize, Deserialize};
use entity::Entity;
use entity_manager::EntityManager;
use component_manager::{ComponentManager, component::Componentable};
use component_manager::component::{name_component::NameComponent, hierarchy_component::HierarchyComponent};
use crate::{game::Game, app::Viewport};

#[derive(Serialize, Deserialize)]
pub struct ECS {
    pub component_manager: ComponentManager,
    entity_manager: EntityManager,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            component_manager: ComponentManager::new(),
            entity_manager: EntityManager::new(),
        }
    }

    pub fn handle_update(&self, _dt: f32, _game: &Game) {
        
    }

    pub fn handle_render(&self, _dt: f32, _game: &Game, _viewport: &Viewport) {

    }

    pub fn create_entity(&mut self) -> Result<Entity, Error> {
        let e = self.entity_manager.create();
        self.attach_component::<NameComponent>(e)?;
        self.attach_component::<HierarchyComponent>(e)?;

        Ok(e)
    }

    pub fn remove_entity(&mut self, e: Entity) -> Result<(), Error> {
        todo!()
    }

    pub fn attach_component<T: Componentable + 'static>(
        &mut self, 
        e: Entity
    ) -> Result<usize, Error> {

        let hash = ComponentManager::type_hash::<T>();
        if !self.component_manager.has(hash) {
            return Err(Error::new(ErrorKind::NotFound,
                "ERROR::ecs::attach_component()::cannot find component"));
        }

        if !self.entity_manager.attach_component(e, hash) {
            return Err(Error::new(ErrorKind::NotFound,
                "ERROR::ecs::attach_component()::cannot attach component"));
        }

        match self.component_manager.get_mut::<T>() {
            Some(c) => c.attach(e),
            None => {
                self.entity_manager.detach_component(e, hash);
                Err(Error::new(ErrorKind::Other,
                    "ERROR::ecs::attach_component()::cannot get component"))
            }
        }
    }

    pub fn detach_component<T: Componentable + 'static>(
        &mut self, 
        e: Entity
    ) -> Result<(), Error> {

        let hash = ComponentManager::type_hash::<T>();
        if !self.component_manager.has(hash) {
            return Err(Error::new(ErrorKind::NotFound,
                "ERROR::ecs::detach_component()::cannot find component"));
        }

        if !self.entity_manager.detach_component(e, hash) {
            return Err(Error::new(ErrorKind::NotFound,
                "ERROR::ecs::detach_component()::cannot detach component"));
        }

        match self.component_manager.get_mut::<T>() {
            Some(c) => c.detach(e),
            None => {
                return Err(Error::new(ErrorKind::NotFound,
                    "ERROR::ecs::detach_component()::cannot get component"));
            }
        }
    }
}