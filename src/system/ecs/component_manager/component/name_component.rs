use serde::{Serialize, Deserialize};
use std::{collections::HashSet, io};
use super::{Component, Componentable};
use crate::game::Game;
use crate::{system::ecs::Entity, app::Viewport};
use crate::util::hash;

#[derive(Serialize, Deserialize)]
struct Data {
    entity: Vec<Entity>,
    name: Vec<(u64, String)>,
    tags: Vec<HashSet<(u64, String)>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            entity: Vec::new(),
            name: Vec::new(),
            tags: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NameComponent {
    pub component: Component,
    data: Data,
    hash_list: HashSet<u64>,
}

#[typetag::serde]
impl Componentable for NameComponent {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn attach(&mut self, entity: Entity) -> Result<usize, std::io::Error> {
        if self.component.does_exist(&entity) {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                "ERROR::NameComponent::attach()::entity already exist"))
        }

        let index = self.component.entities.len();

        self.component.entities.insert(entity, index);

        self.data.entity.push(entity);
        self.data.name.push((0, String::from("")));
        self.data.tags.push(HashSet::new());

        Ok(index)
    }

    fn detach(&mut self, entity: Entity) -> Result<(), std::io::Error> {
        if !self.component.does_exist(&entity) {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                "ERROR::NameComponent::detach()::entity doesn't exist"))
        }

        let to_remove = self.component.entities[&entity];
        let last = self.component.entities.len() - 1;
        let swapped = self.data.entity[last];

        let hash = self.data.name[to_remove].0;
        self.hash_list.remove(&hash);

        self.data.entity.swap(to_remove, last);
        self.data.name.swap(to_remove, last);
        self.data.tags.swap(to_remove, last);

        self.data.entity.pop();
        self.data.name.pop();
        self.data.tags.pop();

        self.component.entities.insert(swapped, to_remove);
        self.component.entities.remove(&entity);

        return Ok(())
    }

    fn handle_update(&mut self, _dt: f32, _game: &Game) {

    }

    fn handle_render(&mut self, _dt: f32, _game: &Game, _viewport: &Viewport){

    }
}

impl NameComponent {
    pub fn new() -> Self {
        Self {
            component: Component::new(),
            data: Data::new(),
            hash_list: HashSet::from([0])
        }
    }

    pub fn get_name(&self, index: usize) -> Option<&(u64, String)> {
        if !self.component.bounds_check(index) {
            return None
        }

        return self.data.name.get(index)
    }

    pub fn get_tags(&self, index: usize) -> Option<&HashSet<(u64, String)>> {
        if !self.component.bounds_check(index) {
            return None
        }

        return self.data.tags.get(index)
    }

    pub fn set_name(&mut self, index: usize, name: String) -> bool {
        if !self.component.bounds_check(index) {
            return false
        }

        let val = self.hash_name(name);

        self.hash_list.insert(val.0);
        self.data.name[index] = val;

        return true
    }

    pub fn add_tag(&mut self, index: usize, name: String) -> bool {
        if !self.component.bounds_check(index) {
            return false
        }

        let hash = hash::hasher(&name);
        return self.data.tags[index].insert((hash, name))
    }

    pub fn remove_tag(&mut self, index: usize, name: String) -> bool {
        if !self.component.bounds_check(index) {
            return false
        }

        let hash = hash::hasher(&name);
        return self.data.tags[index].remove(&(hash, name))
    }

    pub fn has_tag(&self, index: usize, name: String) -> bool {
        if !self.component.bounds_check(index) {
            return false
        }

        let hash = hash::hasher(&name);
        return self.data.tags[index].contains(&(hash, name))
    }

    fn hash_name(&self, name: String) -> (u64, String) {
        let mut hash = hash::hasher(&name);
        let mut temp = name.clone();
        let mut count = 0;

        while self.hash_list.contains(&hash) {
            temp = name.clone() + "_" + &count.to_string();
            hash = hash::hasher(&temp);
            count += 1;
        }

        return (hash, temp)
    }
}