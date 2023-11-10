use std::fs;
use std::path;

pub fn absolute_path(path: &str) -> Option<String> {
    let abs_path = path::Path::new(path);
    match abs_path.canonicalize() {
        Ok(i) => match i.to_str() {
            Some(j) => Some(String::from(j)),
            None => None
        },
        Err(_) => {
            println!("ERROR::file::absolute_path()::cannot make absolute path");
            None
        }
    }
}

pub fn write(path: &str, data: String) {
    match fs::write(path, data) {
        Ok(_) => {},
        Err(e) => println!("ERROR::file::write()::{e}"),
    }
}

pub fn read(path: &str) -> Option<String> {
    let contents = match fs::read_to_string(path) {
        Ok(val) => val,
        Err(_) => {
            println!("ERROR::file::read()::cannot read path to string");
            return None
        }
    };

    return Some(contents);
}

pub fn mkdir(path: &str) {
    match fs::create_dir(path) {
        Ok(_) => println!("SUCCESS::file::mkdir()::directory created - [{path}]"),
        Err(e) => println!("ERROR::file::mkdir()::{e} - [{path}]")
    }
}

pub fn remove_dir(path: &str) {
    match fs::remove_dir(path) {
        Ok(_) => println!("SUCCESS::file::remove_dir()::directory removed - [{path}]"),
        Err(e) => println!("ERROR::file::remove_dir()::{e} - [{path}]")
    }
}

pub fn remove_file(path: &str) {
    match fs::remove_file(path) {
        Ok(_) => println!("SUCCESS::file::remove_file()::file removed - [{path}]"),
        Err(e) => println!("ERROR::file::remove_file()::{e} - [{path}]")
    }
}

pub fn remove_all_file(path: &str) {
    let dir = match fs::read_dir(path) {
        Ok(i) => i,
        Err(_) => {
            println!("ERROR::file::remove_all_file()::cannot read directory");
            return
        }
    };

    for i in dir {
        match i {
            Ok(j) => match j.path().to_str() {
                Some(path) => self::remove_file(path),
                None => continue,
            },
            Err(_) => continue,
        };
    }
}