use std::collections::HashSet;
use std::io;
use serde::{Serialize, Deserialize};

use crate::{system::ecs::entity::Entity, game::Game, app::Viewport};
use crate::util::hash;

use super::{Componentable, Component};

#[derive(Serialize, Deserialize)]
struct Data {
    entity: Vec<Entity>,
    parent: Vec<Option<Entity>>,
    children: Vec<HashSet<Entity>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            entity: Vec::new(),
            parent: Vec::new(),
            children: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct HierarchyComponent {
    component: Component,
    data: Data,
}

#[typetag::serde]
impl Componentable for HierarchyComponent {
    fn attach(&mut self, entity: Entity) -> Result<usize, std::io::Error>  {
        if self.component.does_exist(&entity) {
            return Err(io::Error::new(io::ErrorKind::Other,
                "ERROR::HierarchyComponent::attach()::entity already exist"))
        }

        let index = self.component.entities.len();
        self.component.entities.insert(entity, index);

        self.data.entity.push(entity);
        self.data.parent.push(None);
        self.data.children.push(HashSet::new());

        Ok(index)
    }

    fn detach(&mut self, entity: Entity) -> Result<(), std::io::Error>  {
        if !self.component.does_exist(&entity) {
            return Err(io::Error::new(io::ErrorKind::Other,
                "ERROR::HierarchyComponent::detach()::entity doesn't exist"))
        }

        let to_remove = self.component.entities[&entity];
        let last = self.component.entities.len() - 1;
        let swapped = self.data.entity[last];

        // Remove from parent
        match self.data.parent[to_remove] {
            Some(p) => {
                match self.component.find_index(&p) {
                    Some(p_index) => self.remove_child(p_index, to_remove)?,
                    None => {}
                };
            },
            None => {}
        };

        self.data.entity.swap(to_remove, last);
        self.data.parent.swap(to_remove, last);
        self.data.children.swap(to_remove, last);

        self.data.entity.pop();
        self.data.parent.pop();
        self.data.children.pop();

        self.component.entities.insert(swapped, to_remove);
        self.component.entities.remove(&entity);

        Ok(())
    }

    fn handle_update(&mut self, _dt: f32, _game: &Game) {
        todo!()
    }

    fn handle_render(&mut self, _dt: f32, _game: &Game, _viewport: &Viewport) {
        todo!()
    }

    fn is_empty(&self) -> bool {
        self.component.entities.is_empty() 
    }

    fn get_hash(&self) -> u64 {
        hash::get(&String::from(std::any::type_name::<HierarchyComponent>()))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

impl HierarchyComponent {
    pub fn new() -> Self {
        Self {
            component: Component::new(),
            data: Data::new(),
        }
    }

    pub fn add_child(&mut self, p_index: usize, c_index: usize) -> Result<(), io::Error> {
        if !self.component.bounds_check(p_index) || 
            !self.component.bounds_check(c_index) {
                return Err(io::Error::new(io::ErrorKind::Other,
                    "ERROR::HierarchyComponent::add_child()::out of bounds"))
        }

        let p = self.data.entity[p_index];
        let c = self.data.entity[c_index];

        self.data.parent[c_index] = Some(p);
        self.data.children[p_index].insert(c);

        Ok(())
    }

    pub fn move_child(&mut self, old_p_index: usize, p_index: usize, c_index: usize) -> Result<(), io::Error> {
        if !self.component.bounds_check(old_p_index) || 
            !self.component.bounds_check(p_index) ||
            !self.component.bounds_check(c_index) {
                return Err(io::Error::new(io::ErrorKind::Other,
                    "ERROR::HierarchyComponent::move_child()::out of bounds"))
        }

        self.remove_child(old_p_index, c_index)?;
        self.add_child(p_index, c_index)?;

        Ok(())
    }

    pub fn remove_child(&mut self, p_index: usize, c_index: usize) -> Result<(), io::Error> {
        if !self.component.bounds_check(p_index) || 
            !self.component.bounds_check(c_index) {
                return Err(io::Error::new(io::ErrorKind::Other,
                    "ERROR::HierarchyComponent::remove_child()::out of bounds"))
        }

        let c = self.data.entity[c_index];
        self.data.children[p_index].remove(&c);
        self.data.parent[c_index] = None;

        Ok(())
    }

    pub fn get_entity(&self, index: usize) -> Result<Entity, io::Error> {
        if !self.component.bounds_check(index) {
            return Err(io::Error::new(io::ErrorKind::Other,
                "ERROR::HierarchyComponent::get_entity()::out of bounds"))
        }

        return Ok(self.data.entity[index])
    }

    pub fn get_parent(&self, index: usize) -> Result<Option<Entity>, io::Error> {
        if !self.component.bounds_check(index) {
            return Err(io::Error::new(io::ErrorKind::Other,
                "ERROR::HierarchyComponent::get_parent()::out of bounds"))
        }

        return Ok(self.data.parent[index])
    }

    pub fn get_children(&self, index: usize) -> Result<&HashSet<Entity>, io::Error> {
        if !self.component.bounds_check(index) {
            return Err(io::Error::new(io::ErrorKind::Other,
                "ERROR::HierarchyComponent::get_children()::out of bounds"))
        }

        return Ok(&self.data.children[index])
    }
}