use serde::{Serialize, Deserialize};
use winit::event::{VirtualKeyCode, ModifiersState};

#[derive(Serialize, Deserialize)]
pub struct Key {
    pub code: VirtualKeyCode,
    pub modifier: ModifiersState,
}

impl Key {
    pub fn new(code: VirtualKeyCode, modifier: ModifiersState) -> Self {
        Self { code, modifier }
    }

    pub fn modifiy(&mut self, key: Key) {
        self.code = key.code;
        self.modifier = key.modifier;
    }
}