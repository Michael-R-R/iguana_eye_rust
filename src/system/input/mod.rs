use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use winit::event::{KeyboardInput, ModifiersState, MouseButton, ElementState};

mod key;
mod button;
mod keyboard;
mod mouse;

pub use self::key::Key;
pub use self::button::Button;
pub use self::keyboard::Keyboard;
pub use self::mouse::Mouse;

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub keyboard: Keyboard,
    pub mouse: Mouse,
}

impl Input {
    pub fn new() -> Self {
        let keyboard = Keyboard::new();
        let mouse = Mouse::new();

        Self { 
            keyboard,
            mouse,
         }
    }

    pub fn set_keyboard_hotkeys(&mut self, hotkeys: HashMap<String, Key>) {
        self.keyboard.hotkeys = hotkeys;
    }

    pub fn set_mouse_hotkeys(&mut self, hotkeys: HashMap<String, Button>) {
        self.mouse.hotkeys = hotkeys;
    }

    pub fn key_state(&self, name: String) -> bool {
        return self.keyboard.state(name);
    }

    pub fn button_state(&self, name: String) -> bool {
        return self.mouse.state(name);
    }

    pub fn handle_modifiers(&mut self, m: &ModifiersState) {
        self.keyboard.handle_modifiers(m);
    }

    pub fn handle_kb_input(&mut self, input: &KeyboardInput) {
        self.keyboard.handle_input(input);
    }

    pub fn handle_mb_input(&mut self, state: &ElementState, input: &MouseButton) {
        self.mouse.handle_input(state, input);
    }
}