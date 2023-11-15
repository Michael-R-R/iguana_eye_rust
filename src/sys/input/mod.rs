use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use winit::event::{KeyboardInput, VirtualKeyCode, ModifiersState, ElementState};

mod key;
pub use self::key::Key;

#[derive(Serialize, Deserialize)]
pub struct Input {
    hot_keys: HashMap<String, Key>,
    keys: HashMap<VirtualKeyCode, bool>,
    modifiers: HashMap<ModifiersState, bool>,
}

impl Input {
    pub fn new(hk: Option<HashMap<String, Key>>) -> Self {
        let hot_keys = match hk {
            Some(val) => val,
            None => HashMap::new()
        };
        let keys = HashMap::new();
        let modifiers = HashMap::new();

        Self { 
            hot_keys,
            keys,
            modifiers,
        }
    }

    pub fn status(&self, name: &String) -> bool {
        let key = match self.hot_keys.get(name) {
            Some(val) => val,
            None => return false
        };

        match self.keys.get(&key.code) {
            Some(code) => {
                match self.modifiers.get(&key.modifier) {
                    Some(modifier) => {
                        return *code && *modifier
                    },
                    None => return false
                }
            },
            None => return false
        }
    }

    pub fn handle_input(&mut self, input: &KeyboardInput) {
        let key = match input.virtual_keycode {
            Some(val) => val,
            None => return
        };
        
        match input.state {
            ElementState::Pressed => {
                self.keys.insert(key, true);
            },
            ElementState::Released => {
                self.keys.insert(key, false);
            }
        }
    }

    pub fn handle_modifiers(&mut self, m: &ModifiersState) {
        match *m {
            ModifiersState::ALT => { self.modifiers.insert(ModifiersState::ALT, m.alt()); },
            ModifiersState::CTRL => { self.modifiers.insert(ModifiersState::CTRL, m.ctrl()); },
            ModifiersState::SHIFT => { self.modifiers.insert(ModifiersState::SHIFT, m.shift()); },
            ModifiersState::LOGO => { self.modifiers.insert(ModifiersState::LOGO, m.logo()); },
            _ => { self.modifiers.clear(); }
        }
    }
}