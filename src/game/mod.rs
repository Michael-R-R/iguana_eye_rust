use std::io::Error;
use serde::{Serialize, Deserialize};
use winit::{window::Window, dpi::PhysicalSize};
use winit::event::{KeyboardInput, ModifiersState, MouseButton, ElementState};

mod game_setup;
mod scene_manager;

use self::scene_manager::SceneManager;
use crate::systems::input::Input;
use crate::systems::ecs::ECS;
use crate::app::{Viewport, Frame};

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub input: Input,
    pub scene_manager: SceneManager,
    pub ecs: ECS,
}

impl Game {
    pub fn new() -> Result<Self, Error> {
        let input = game_setup::create_input()?;
        let scene_manager = game_setup::create_scene_manager()?;
        let ecs = game_setup::create_ecs()?;

        Ok(Self { 
            input,
            scene_manager,
            ecs,
        })
    }

    pub fn handle_update(&self, _window: &Window, _dt: f32) {

    }

    pub fn handle_render(&self, 
        _window: &Window, 
        _viewport: &Viewport, 
        frame: &mut Frame, 
        _dt: f32
    ) {
        let _rp = frame.render_pass_game();
    }

    pub fn handle_resize(&mut self, _size: PhysicalSize<u32>) {

    }

    pub fn handle_modifiers(&mut self, m: &ModifiersState) {
        self.input.handle_modifiers(m);
    }

    pub fn handle_kb_input(&mut self, input: &KeyboardInput) {
        self.input.handle_kb_input(input);
    }

    pub fn handle_mb_input(&mut self, state: &ElementState, input: &MouseButton) {
        self.input.handle_mb_input(state, input);
    }
}