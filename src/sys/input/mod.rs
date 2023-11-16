use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use winit::event::{KeyboardInput, ModifiersState};

mod key;
mod keyboard;

pub use self::key::Key;
pub use self::keyboard::Keyboard;

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub keyboard: Keyboard,
}

impl Input {
    pub fn new(hotkeys_kb: Option<HashMap<String, Key>>) -> Self {
        let keyboard = Keyboard::new(hotkeys_kb);

        Self { 
            keyboard,
         }
    }

    pub fn key_state(&self, name: String) -> bool {
        return self.keyboard.state(name);
    }

    pub fn handle_input(&mut self, input: &KeyboardInput) {
        self.keyboard.handle_input(input);
    }

    pub fn handle_modifiers(&mut self, m: &ModifiersState) {
        self.keyboard.handle_modifiers(m);
    }
}