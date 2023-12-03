pub mod entity;
pub mod entity_manager;
pub mod component_manager;

use std::{io::Error, collections::HashSet};

use serde::{Serialize, Deserialize};
use entity::Entity;
use entity_manager::EntityManager;
use component_manager::{ComponentManager, component::*, component::Componentable};
use crate::{game::Game, app::Viewport};

#[derive(Serialize, Deserialize)]
pub struct ECS {
    entity_manager: EntityManager,
    component_manager: ComponentManager,
}

impl ECS {
    pub fn new() -> Result<Self, Error> {
        let mut component_manager = ComponentManager::new();
        component_manager.add(Box::new(name_component::NameComponent::new()))?;
        component_manager.add(Box::new(hierarchy_component::HierarchyComponent::new()))?;

        Ok(Self {
            component_manager,
            entity_manager: EntityManager::new(),
        })
    }

    pub fn handle_update(&mut self, dt: f32, game: &Game) {
        self.component_manager.handle_update(dt, game);
    }

    pub fn handle_render(&mut self, dt: f32, game: &Game, viewport: &Viewport) {
        self.component_manager.handle_render(dt, game, viewport);
    }

    pub fn add_component(&mut self, c: Box<dyn Componentable>) -> Result<(), Error> {
        self.component_manager.add(c)
    }

    pub fn insert_component(
        &mut self, 
        index: usize, 
        c: Box<dyn Componentable>
    ) -> Result<(), Error> {
        self.component_manager.insert(index, c)
    }

    pub fn remove_component<T: Componentable>(&mut self) -> Result<(), Error> {
        self.component_manager.remove::<T>()
    }

    pub fn get_component<T: Componentable + 'static>(&self) -> Option<&T> {
        self.component_manager.get::<T>()
    }

    pub fn get_component_mut<T: Componentable + 'static>(&mut self) -> Option<&mut T> {
        self.component_manager.get_mut::<T>()
    }

    pub fn get_component_by_hash(&self, hash: u64) -> Option<&dyn Componentable> {
        self.component_manager.get_by_hash(hash)
    }

    pub fn get_component_by_hash_mut(&mut self, hash: u64) -> Option<&mut dyn Componentable> {
        self.component_manager.get_by_hash_mut(hash)
    }

    pub fn create_entity(&mut self) -> Result<Entity, Error> {
        let e = self.entity_manager.create();
        self.attach_component::<name_component::NameComponent>(e)?;
        self.attach_component::<hierarchy_component::HierarchyComponent>(e)?;

        Ok(e)
    }

    pub fn remove_entity(&mut self, e: Entity) -> Result<(), Error> {

        // purge entity from all attached components
        if let Some(hash_list) = self.entity_manager.get_attached(e) {
            self.component_manager.purge_entity(e, hash_list)?;
        }

        self.entity_manager.remove(e);

        Ok(())
    }

    pub fn attach_component<T: Componentable + 'static>(
        &mut self, 
        e: Entity
    ) -> Result<usize, Error> {

        let hash = ComponentManager::type_hash::<T>();
        match self.component_manager.attach(e, hash) {
            Ok(index) => {
                self.entity_manager.attach_component(e, hash);
                return Ok(index)
            },
            Err(e) => return Err(e)
        }
    }

    pub fn detach_component<T: Componentable + 'static>(
        &mut self, 
        e: Entity
    ) -> Result<(), Error> {

        let hash = ComponentManager::type_hash::<T>();
        match self.component_manager.detach(e, hash) {
            Ok(()) => {
                self.entity_manager.detach_component(e, hash);
                return Ok(())
            },
            Err(e) => return Err(e)
        }
    }

    pub fn get_attached(&self, entity: Entity) -> Option<&HashSet<u64>> {
        self.entity_manager.get_attached(entity)
    }

    pub fn does_entity_exist(&self, entity: Entity) -> bool {
        self.entity_manager.does_exist(entity)
    }

    pub fn has_component(&self, entity: Entity, component: u64) -> bool {
        self.entity_manager.has_component(entity, component)
    }

    pub fn count(&self) -> usize {
        self.entity_manager.count()
    }
}