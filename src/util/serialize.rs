use std::io;
use serde::{ser, de};
use crate::util::file;

pub fn write<T: ser::Serialize>(path: &str, object: &T) -> Result<String, io::Error> {
    let str = match serde_json::to_string_pretty(&object) {
        Ok(val) => val,
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                    "ERROR::serialize::write()::cannot get pretty json"))
        }
    };

    match std::fs::write(path, &str) {
        Ok(()) => Ok(str),
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::Other,
                    "ERROR::serialize::write()::cannot write to file"))
        }
    }
}

pub fn read<T: de::DeserializeOwned>(path: &str) -> Result<T, io::Error> {
    let abs_path = match file::absolute_path(path) {
        Ok(val) => val,
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                "ERROR::serialize::read()::cannot get absolute path"));
        },
    };

    let str = match std::fs::read_to_string(abs_path) {
        Ok(val) => val,
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::Other,
                "ERROR::serialize::read()::cannot read from file"));
        },
    };

    match serde_json::from_str(&str) {
        Ok(val) => Ok(val),
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                "ERROR::serialize::read()::failed to deserialize"));
        },
    }
}