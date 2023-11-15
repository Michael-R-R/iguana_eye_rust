use std::collections::HashMap;

use wgpu::RenderPass;
use winit::{window::Window, dpi::PhysicalSize, event::{KeyboardInput, ModifiersState}};
use serde::{Serialize, Deserialize};

use crate::sys::Input;

// TODO test
use winit::event::VirtualKeyCode;
use crate::sys::input::Key;

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub input: Input,
}

impl Game {
    pub fn new() -> Self {
        // TODO test
        let hk = Some(HashMap::from([
            ("Forward".to_string(), Key::new(VirtualKeyCode::W, ModifiersState::SHIFT)),
            ("Backward".to_string(), Key::new(VirtualKeyCode::S, ModifiersState::empty())),
        ]));

        Self { 
            input: Input::new(hk),
        }
    }

    pub fn update(&self, _window: &Window, _dt: f32) {
        // TODO test
        let forward = String::from("Forward");
        let backward = String::from("Backward");
        if self.input.status(&forward) {
            println!("Forward");
        }
        if self.input.status(&backward) { // TODO not working
            println!("Backward");
        }
    }

    pub fn render(&self, _window: &Window, _rp: &RenderPass, _dt: f32) {
        // --- Draw here --- //

        // ----------------- //
    }

    pub fn resize(&self, _size: PhysicalSize<u32>) {

    }

    pub fn input(&mut self, input: &KeyboardInput) {
        self.input.handle_input(input);
    }

    pub fn modifiers(&mut self, m: &ModifiersState) {
        self.input.handle_modifiers(m);
    }
}