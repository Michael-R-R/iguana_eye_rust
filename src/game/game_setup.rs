use std::io::Error;
use winit::event::{VirtualKeyCode, ModifiersState};

use crate::systems::input::{Input, Key};
use super::scene_manager::SceneManager;
use crate::systems::ecs::ECS;
use crate::systems::ecs::component_manager::component::*;

pub fn create_input() -> Result<Input, Error> {
    let mut input = Input::new();
    input.keyboard.add_hotkey(String::from("Forward"), Key::new(VirtualKeyCode::W, ModifiersState::empty()));
    input.keyboard.add_hotkey(String::from("Backward"), Key::new(VirtualKeyCode::S, ModifiersState::empty()));
    input.keyboard.add_hotkey(String::from("Left"), Key::new(VirtualKeyCode::A, ModifiersState::empty()));
    input.keyboard.add_hotkey(String::from("Right"), Key::new(VirtualKeyCode::D, ModifiersState::empty()));
    input.keyboard.add_hotkey(String::from("Up"), Key::new(VirtualKeyCode::Q, ModifiersState::empty()));
    input.keyboard.add_hotkey(String::from("Down"), Key::new(VirtualKeyCode::E, ModifiersState::empty()));

    Ok(input)
}

pub fn create_scene_manager() -> Result<SceneManager, Error> {
    let scene_manager = SceneManager::new();

    Ok(scene_manager)
}

pub fn create_ecs() -> Result<ECS, Error> {
    let mut ecs = ECS::new();
    ecs.add_component(Box::new(name_component::NameComponent::new()))?;
    ecs.add_component(Box::new(hierarchy_component::HierarchyComponent::new()))?;

    Ok(ecs)
}