use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RscManager<T> {
    rsc_list: HashMap<u64, T>,
}

impl<T> RscManager<T> {
    pub fn new() -> Self {
        Self {
            rsc_list: HashMap::new(),
        }
    }

    pub fn add(&mut self, hash: u64, value: T) -> Result<(), Error> {
        if self.does_exist(hash) {
            return Err(Error::new(ErrorKind::Other, 
                "ERROR::RscManager::add()::hash already exists"))
        }

        self.rsc_list.insert(hash, value);

        return Ok(())
    }

    pub fn remove(&mut self, hash: u64) -> Result<(), Error> {
        if !self.does_exist(hash) {
            return Err(Error::new(ErrorKind::Other, 
                "ERROR::RscManager::remove()::hash doesn't exists"))
        }

        self.rsc_list.remove(&hash);

        return Ok(())
    }

    pub fn change_hash(&mut self, old: u64, new: u64) -> Result<(), Error> {
        if !self.does_exist(old) ||
            self.does_exist(new) {
                return Err(Error::new(ErrorKind::Other, 
                    "ERROR::RscManager::change_hash()::incorrect hash values"))
        }

        match self.rsc_list.remove(&old) {
            Some(temp) => {
                self.rsc_list.insert(new, temp);
                return Ok(())
            },
            None => {
                return Err(Error::new(ErrorKind::Other, 
                    "ERROR::RscManager::change_hash()::couldn't remove value"))
            }
        }
    }

    pub fn clear(&mut self) {
        self.rsc_list = HashMap::new();
    }

    pub fn does_exist(&self, hash: u64) -> bool {
        return self.rsc_list.contains_key(&hash)
    }

    pub fn get(&self, hash: u64) -> Option<&T> {
        return self.rsc_list.get(&hash)
    }

    pub fn get_mut(&mut self, hash: u64) -> Option<&mut T> {
        return self.rsc_list.get_mut(&hash)
    }
}