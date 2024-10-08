use winit::event::MouseButton;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Button {
    pub button: MouseButton,
}

impl Button {
    pub fn new(button: MouseButton) -> Self {
        Self { 
            button, 
        }
    }
}