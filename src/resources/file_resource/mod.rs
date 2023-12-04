pub mod shader;
pub mod renderable;

use std::io::Error;
use serde::{Serialize, Deserialize};
use crate::util::{hash, file};

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct FileResource {
    pub path: String,
    pub hash: u64,
}

impl FileResource {
    pub fn new(path: &str) -> Result<Self, Error> {
        let path = file::absolute_path(&path)?;
        let hash = hash::get(&path);

        Ok(Self {
            path,
            hash,
        })
    }

    pub fn set(&mut self, path: String) -> Result<(), Error> {
        let path = file::absolute_path(&path)?;
        let hash = hash::get(&path);

        Ok(())
    }
}

impl PartialEq for FileResource {
    fn eq(&self, other: &Self) -> bool {
        return self.hash == other.hash
    }
}