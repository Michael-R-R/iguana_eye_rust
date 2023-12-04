use std::io::Error;
use serde::{Serialize, Deserialize};

use super::ecs::ECS;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    ecs: ECS
}

impl Scene {
    pub fn new() -> Result<Self, Error> {
        let ecs = ECS::new()?;

        Ok(Self {
            ecs,
        })
    }
}