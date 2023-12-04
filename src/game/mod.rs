use std::io::Error;
use serde::{Serialize, Deserialize};
use winit::{window::Window, dpi::PhysicalSize};
use winit::event::{KeyboardInput, ModifiersState, MouseButton, ElementState};

use crate::app::{Viewport, Frame};
use crate::systems::input::Input;
use crate::systems::scene::Scene;

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub input: Input,
    pub scene: Scene,
}

impl Game {
    pub fn new() -> Result<Self, Error> {
        let input = Input::new();
        let scene = Scene::new()?;

        Ok(Self { 
            input,
            scene,
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