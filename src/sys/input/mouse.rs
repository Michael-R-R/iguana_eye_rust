use std::collections::HashMap;
use winit::event::{MouseButton, ElementState};
use serde::{Serialize, Deserialize};

use super::Button;

#[derive(Serialize, Deserialize)]
pub struct Mouse {
    pub hotkeys: HashMap<String, Button>,

    #[serde(skip)]
    buttons: HashMap<MouseButton, bool>
}

impl Mouse {
    pub fn new() -> Self {
        let hotkeys = HashMap::new();
        let buttons = HashMap::new();

        Self {
            hotkeys,
            buttons,
        }
    }

    pub fn add_button(&mut self, name: String, button: Button) {
        if self.hotkeys.contains_key(&name){
            return;
        }

        self.hotkeys.insert(name, button);
    }

    pub fn remove_button(&mut self, name: String) {
        self.hotkeys.remove(&name);
    }

    pub fn modify_button(&mut self, name: String, button: Button) {
        if self.hotkeys.contains_key(&name) {
            self.hotkeys.insert(name, button);
        }
    }

    pub fn state(&self, name: String) -> bool {
        let hotkey = match self.hotkeys.get(&name) {
            Some(val) => val,
            None => return false
        };

        match self.buttons.get(&hotkey.button) {
            Some(state) => {
                return *state
            },
            None => return false
        }
    }

    pub fn handle_input(&mut self, state: &ElementState, button: &MouseButton) {
        match state {
            ElementState::Pressed => {
                self.buttons.insert(button.clone(), true);
            },
            ElementState::Released => {
                self.buttons.insert(button.clone(), false);
            }
        }
    }
}