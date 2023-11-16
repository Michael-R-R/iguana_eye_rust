use serde::{Serialize, Deserialize};
use winit::event::{VirtualKeyCode, ModifiersState};

#[derive(Serialize, Deserialize, Debug)]
pub struct Key {
    pub code: VirtualKeyCode,
    pub modifier: ModifiersState,
}

impl Key {
    pub fn new(code: VirtualKeyCode, modifier: ModifiersState) -> Self {
        Self { code, modifier }
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        return (self.code == other.code) 
            && (self.modifier == other.modifier)
    }
}