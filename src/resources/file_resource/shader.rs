use std::io::Error;
use wgpu::{ShaderModule, Device};
use serde::{Serialize, Deserialize};

use crate::resources::file_resource::FileResource;

#[derive(Serialize, Deserialize)]
pub struct Shader {
    pub resource: FileResource,

    #[serde(skip)]
    pub module: Option<ShaderModule>,
}

impl Shader {
    pub fn new(path: &str, device: &Device) -> Result<Self, Error> {
        let resource = FileResource::new(path)?;
        let module = Shader::create(path, device)?;

        Ok(Self {
            resource,
            module,
        })
    }

    pub fn modify(&mut self, path: &str, device: &Device) -> Result<(), Error> {
        self.resource = FileResource::new(path)?;
        self.module = Shader::create(path, device)?;

        Ok(())
    }

    fn create(path: &str, device: &Device) -> Result<Option<ShaderModule>, Error> {
        let content = std::fs::read_to_string(&path)?;
        let module = Some(device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(path),
            source: wgpu::ShaderSource::Wgsl(content.into()),
        }));

        return Ok(module)
    }
}

impl PartialEq for Shader {
    fn eq(&self, other: &Self) -> bool {
        return self.resource == other.resource
    }
}