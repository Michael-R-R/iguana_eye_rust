use wgpu::RenderPass;
use winit::{window::Window, dpi::PhysicalSize, event::{KeyboardInput, ModifiersState}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Game {
}

impl Game {
    pub fn new() -> Self {
        Self { }
    }

    pub fn update(&self, _window: &Window, _dt: f32) {

    }

    pub fn render(&self, _window: &Window, _rp: &RenderPass, _dt: f32) {
        // --- Draw here --- //

        // ----------------- //
    }

    pub fn resize(&self, _size: PhysicalSize<u32>) {

    }

    pub fn input(&self, _input: &KeyboardInput) {
        
    }

    pub fn modifiers(&self, _mod: &ModifiersState) {

    }
}