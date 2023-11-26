use std::io;
use wgpu::{ShaderModule, Device};
use serde::{Serialize, Deserialize};
use crate::util::{file, hash};

#[derive(Serialize, Deserialize)]
pub struct Shader {
    pub path: String,
    pub hash: u64,

    #[serde(skip)]
    pub module: Option<ShaderModule>,
}

impl Shader {
    pub fn new(path: &str, device: &Device) -> Result<Self, io::Error> {
        let (path, hash, module) = Shader::create(path, device)?;

        Ok(Self {
            path,
            hash,
            module,
        })
    }

    pub fn modify(&mut self, path: &str, device: &Device) -> Result<(), io::Error> {
        let (path, hash, module) = Shader::create(path, device)?;
        self.path = path;
        self.hash = hash;
        self.module = module;

        Ok(())
    }

    fn create(path: &str, device: &Device) -> Result<(String, u64, Option<ShaderModule>), io::Error> {
        let path = file::absolute_path(path)?;
        let content = std::fs::read_to_string(&path)?;
        let hash = hash::hasher(&path);
        let module = Some(device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&hash.to_string()),
            source: wgpu::ShaderSource::Wgsl(content.into()),
        }));

        Ok((path, hash, module))
    }
}

impl PartialEq for Shader {
    fn eq(&self, other: &Self) -> bool {
        return self.hash == other.hash;
    }
}