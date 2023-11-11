use std::fs;
use std::io;
use std::path;

pub fn absolute_path(path: &str) -> Result<String, io::Error> {
    let abs_path = path::Path::new(path);
    match abs_path.canonicalize() {
        Ok(to_convert) => match to_convert.to_str() {
            Some(str) => Ok(String::from(str)),
            None => {
                return Err(io::Error::new(io::ErrorKind::InvalidData,
                         "ERROR::file::absolute_path()::cannot convert to string"))
            }
        },
        Err(e) => return Err(e)
    }
}

pub fn exist(path: &str) -> bool {
    let abs_path = match absolute_path(path) {
        Ok(val) => val,
        Err(_) => return false
    };

    return path::Path::new(&abs_path).exists();
}

pub fn extract_dir(path: &str) -> Result<String, io::Error> {
    let abs_path = match absolute_path(path) {
        Ok(val) => val,
        Err(e) => return Err(e)
    };

    let sys_path = path::Path::new(&abs_path);
    if !sys_path.is_file() {
        return Err(io::Error::new(io::ErrorKind::Other,
            "ERROR::file::extract_dir()::path is not a file"))
    }

    let mut buf = sys_path.to_path_buf();
    buf.pop();

    let extracted_path = match buf.to_str() {
        Some(val) => String::from(val),
        None => {
            return Err(io::Error::new(io::ErrorKind::Other,
                "ERROR::file::extract_dir()::cannot convert buf path to string"))
        }
    };

    Ok(extracted_path)
}

pub fn extract_file_name(path: &str) -> Result<String, io::Error> {
    let abs_path = match absolute_path(path) {
        Ok(val) => val,
        Err(e) => return Err(e)
    };

    let sys_path = path::Path::new(&abs_path);
    if !sys_path.is_file() {
        return Err(io::Error::new(io::ErrorKind::Other,
            "ERROR::file::extract_file_name()::path is not a file"))
    }

    let extracted_name = match sys_path.file_name() {
        Some(val) => {
            match val.to_str() {
                Some(val) => String::from(val),
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other,
                        "ERROR::file::extract_file_name()::cannot convert OsStr to string"))
                }
            }
        },
        None => {
            return Err(io::Error::new(io::ErrorKind::Other,
                "ERROR::file::extract_file_name()::cannot convert OsStr to string"))
        }
    };

    Ok(extracted_name)
}

pub fn extract_extension(path: &str) -> Result<String, io::Error> {
    let abs_path = match absolute_path(path) {
        Ok(val) => val,
        Err(e) => return Err(e)
    };

    let sys_path = path::Path::new(&abs_path);
    if !sys_path.is_file() {
        return Err(io::Error::new(io::ErrorKind::Other,
            "ERROR::file::extract_extension()::path is not a file"))
    }

    let extracted_ext = match sys_path.extension() {
        Some(val) => {
            match val.to_str() {
                Some(val) => String::from(val),
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other,
                        "ERROR::file::extract_extension()::cannot convert OsStr to string"))
                }
            }
        },
        None => {
            return Err(io::Error::new(io::ErrorKind::Other,
                "ERROR::file::extract_extension()::cannot convert OsStr to string"))
        }
    };

    Ok(extracted_ext)
}

pub fn remove_all_file(path: &str) -> Result<(), io::Error> {
    let dir = match fs::read_dir(path) {
        Ok(val) => val,
        Err(e) => return Err(e)
    };

    for entry in dir {
        match entry {
            Ok(file) => {
                match file.path().to_str() {
                    Some(path) => {
                        match fs::remove_file(path) {
                            Ok(()) => {},
                            Err(_) => continue
                        }
                    },
                    None => continue,
                }
            },
            Err(_) => continue,
        };
    }

    Ok(())
}