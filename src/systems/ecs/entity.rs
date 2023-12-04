use serde::{Serialize, Deserialize};

#[derive(Clone, Copy)]
#[derive(Hash, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Entity {
    pub id: u64,
}

impl Entity {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
    }
}