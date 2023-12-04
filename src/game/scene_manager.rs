use serde::{Serialize, Deserialize};

use crate::managers::RscManager;
use crate::graphics::shader::Shader;

#[derive(Serialize, Deserialize)]
pub struct SceneManager {
    pub shader_manager: RscManager<Shader>,
}

impl SceneManager {
    pub fn new() -> Self {
        let shader_manager = RscManager::new();

        Self {
            shader_manager,
        }
    }
}