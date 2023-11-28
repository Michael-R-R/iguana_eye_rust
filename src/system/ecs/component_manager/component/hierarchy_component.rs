use serde::{Serialize, Deserialize};

use crate::{system::ecs::entity::Entity, game::Game, app::Viewport};
use crate::util::hash;

use super::{Componentable, Component};

#[derive(Serialize, Deserialize)]
struct Data {

}

impl Data {
    
}

#[derive(Serialize, Deserialize)]
pub struct HierarchyComponent {
    component: Component,
    data: Data,
}

#[typetag::serde]
impl Componentable for HierarchyComponent {
    fn attach(&mut self, entity: Entity) -> Result<usize, std::io::Error>  {
        todo!()
    }

    fn detach(&mut self, entity: Entity) -> Result<(), std::io::Error>  {
        todo!()
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
        todo!()
    }
}