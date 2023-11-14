use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
}

impl Config {
    pub fn default() -> Self {
        Self { 
            width: 800,
            height: 600,
            fullscreen: false,
        }
    }
}