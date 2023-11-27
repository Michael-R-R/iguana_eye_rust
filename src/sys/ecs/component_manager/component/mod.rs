pub mod name_component;

#[typetag::serde(tag = "type")]
pub trait Componentable {
    fn attach(&mut self, entity: Entity) -> Result<usize, std::io::Error>;
    fn detach(&mut self, entity: Entity) -> Result<(), std::io::Error>;
    fn handle_update(&mut self, dt: f32, window: &Window);
    fn handle_render(&mut self, dt: f32, window: &Window, viewport: &Viewport);
}

use std::collections::HashMap;
use winit::window::Window;
use serde::{Serialize, Deserialize};
use crate::{sys::ecs::Entity, app::Viewport};

#[derive(Serialize, Deserialize)]
pub struct Component {
    pub entities: HashMap<Entity, usize>, // <entity, index>
}

impl Component {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    pub fn find_index(&self, entity: &Entity) -> Option<usize> {
        match self.entities.get(entity) {
            Some(index) => return Some(*index),
            None => return None
        }
    }

    pub fn does_exist(&self, entity: &Entity) -> bool {
        return self.entities.contains_key(entity)
    }

    pub fn bounds_check(&self, index: usize) -> bool {
        return index > 0 && index < self.entities.len()
    }
}