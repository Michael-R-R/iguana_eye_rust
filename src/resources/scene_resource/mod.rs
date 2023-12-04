pub mod camera;

use serde::{Serialize, Deserialize};

#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct SceneResource {
    pub hash: u64,
}

impl SceneResource {
    pub fn new(hash: u64) -> Self {
        Self {
            hash,
        }
    }

    pub fn set(&mut self, hash: u64) {
        self.hash = hash;
    }
}

impl PartialEq for SceneResource {
    fn eq(&self, other: &Self) -> bool {
        return self.hash == other.hash
    }
}