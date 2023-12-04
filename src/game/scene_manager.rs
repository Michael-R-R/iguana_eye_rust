use serde::{Serialize, Deserialize};

use crate::managers::RscManager;
use crate::resources::file_resource::*;

#[derive(Serialize, Deserialize)]
pub struct SceneManager {
    pub shader_manager: RscManager<shader::Shader>,
}

impl SceneManager {
    pub fn new() -> Self {
        let shader_manager = RscManager::new();

        Self {
            shader_manager,
        }
    }
}