use wgpu::RenderPass;
use serde::{Serialize, Deserialize};
use winit::{window::Window, dpi::PhysicalSize};
use winit::event::{KeyboardInput, ModifiersState, MouseButton, ElementState};

use crate::sys::input::Input;

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub input: Input,
}

impl Game {
    pub fn new() -> Self {
        let input = Input::new();

        Self { 
            input,
        }
    }

    pub fn handle_update(&self, _window: &Window, _dt: f32) {

    }

    pub fn handle_render(&self, _window: &Window, _rp: &RenderPass, _dt: f32) {
        // --- Draw here --- //

        // ----------------- //
    }

    pub fn handle_resize(&self, _size: PhysicalSize<u32>) {

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