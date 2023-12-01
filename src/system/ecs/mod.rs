pub mod entity;
pub mod entity_manager;
pub mod component_manager;

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

    pub fn handle_update(&mut self, dt: f32, game: &Game) {
        self.component_manager.handle_update(dt, game);
    }

    pub fn handle_render(&mut self, dt: f32, game: &Game, viewport: &Viewport) {
        self.component_manager.handle_render(dt, game, viewport);
    }

    pub fn create_entity(&mut self) -> Result<Entity, Error> {
        let e = self.entity_manager.create();
        self.attach_component::<NameComponent>(e)?;
        self.attach_component::<HierarchyComponent>(e)?;

        Ok(e)
    }

    pub fn remove_entity(&mut self, e: Entity) -> Result<(), Error> {
        // Check if this entity has any children
        match self.component_manager.get::<HierarchyComponent>() {
            Some(hc) => {
                match hc.component.find_index(&e) {
                    Some(index) => {
                        let children = hc.get_children(index)?;
                        if !children.is_empty() {
                            return Err(Error::new(ErrorKind::Other,
                                "ERROR::ecs::remove_entity()::children list not empty"))
                        }
                    },
                    None => {
                        return Err(Error::new(ErrorKind::NotFound,
                            "ERROR::ecs::remove_entity()::cannot find index"))
                    }
                }
            },
            None => {
                return Err(Error::new(ErrorKind::NotFound,
                    "ERROR::ecs::remove_entity()::cannot find hierarchy component"))
            }
        };

        // Remove entity from all attached components
        if let Some(attached) = self.entity_manager.get_components(e) {
            for hash in attached {
                if let Some(c) = self.component_manager.get_by_hash_mut(*hash) {
                    c.detach(e)?;
                }
            }
        }

        self.entity_manager.remove(e);

        Ok(())
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
            return Err(Error::new(ErrorKind::Other,
                "ERROR::ecs::attach_component()::cannot attach component"));
        }

        match self.component_manager.get_mut::<T>() {
            Some(c) => c.attach(e),
            None => {
                self.entity_manager.detach_component(e, hash);
                Err(Error::new(ErrorKind::NotFound,
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
            return Err(Error::new(ErrorKind::Other,
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