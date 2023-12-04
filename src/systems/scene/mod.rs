use std::io::Error;
use serde::{Serialize, Deserialize};

use super::ecs::ECS;
use crate::managers::RscManager;
use crate::graphics::shader::Shader;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub shader_manager: RscManager<Shader>,
    pub ecs: ECS,
}

impl Scene {
    pub fn new() -> Result<Self, Error> {
        let shader_manager = RscManager::new();
        let ecs = ECS::new()?;

        Ok(Self {
            shader_manager,
            ecs,
        })
    }
}