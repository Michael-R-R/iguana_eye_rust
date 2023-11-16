use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use winit::event::{KeyboardInput, VirtualKeyCode, ModifiersState, ElementState};

use super::Key;

#[derive(Serialize, Deserialize)]
pub struct Keyboard {
    pub hotkeys: HashMap<String, Key>,
    keys: HashMap<VirtualKeyCode, bool>,
    modifiers: HashMap<ModifiersState, bool>,
}

impl Keyboard {
    pub fn new(hk: Option<HashMap<String, Key>>) -> Self {

        let hotkeys = match hk {
            Some(val) => val,
            None => HashMap::new()
        };
        let keys = HashMap::new();
        let modifiers = HashMap::from([(ModifiersState::empty(), true)]);

        Self { 
            hotkeys,
            keys,
            modifiers,
        }
    }

    pub fn add_hotkey(&mut self, name: String, key: Key) {
        self.hotkeys.insert(name, key);
    }

    pub fn remove_hotkey(&mut self, name: String) {
        self.hotkeys.remove(&name);
    }

    pub fn update_hotkey(&mut self, name: String, key: Key) {
        if self.hotkeys.contains_key(&name) {
            self.hotkeys.insert(name, key);
        }
    }

    pub fn state(&self, name: String) -> bool {
        let hotkey = match self.hotkeys.get(&name) {
            Some(val) => val,
            None => return false
        };

        match self.keys.get(&hotkey.code) {
            Some(key) => {
                match self.modifiers.get(&hotkey.modifier) {
                    Some(modifier) => {
                        return *key && *modifier
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
            _ => { self.modifiers.clear(); }
        }
        
        self.modifiers.insert(ModifiersState::empty(), m.is_empty());
    }
}