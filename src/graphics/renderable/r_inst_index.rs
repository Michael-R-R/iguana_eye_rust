use std::io;

use serde::{Serialize, Deserialize};
use wgpu::{Device, SurfaceConfiguration};

use super::{Index, Instance, OnDeserialization};
use crate::graphics::{InstanceBuffer, Shader, VertexBuffer, Layout};

#[derive(Serialize, Deserialize)]
pub struct InstanceIndex {
    pub r_index: Index,
    pub r_instance: Instance,
}

impl InstanceIndex {
    pub fn new(
        hash: u64,
        device: &Device,
        config: &SurfaceConfiguration,
        shader: &Shader,
        buffer_list: Vec<VertexBuffer>,
        index_list: Vec<u16>,
        inst_list: Vec<InstanceBuffer>,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>,
    ) -> Result<Self, io::Error> {

        buffer_layouts.insert(0, InstanceBuffer::layout());

        let r_index = Index::new(hash, device, config, shader, buffer_list, index_list, buffer_layouts)?;
        let r_instance = Instance::new(device, inst_list)?;

        Ok(Self {
            r_index,
            r_instance,
        })
    }

    pub fn modify(
        &mut self,
        device: &Device,
        config: &SurfaceConfiguration,
        shader: &Shader,
        buffer_list: Vec<VertexBuffer>,
        index_list: Vec<u16>,
        inst_list: Vec<InstanceBuffer>,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>
    ) -> Result<(), io::Error> {

        buffer_layouts.insert(0, InstanceBuffer::layout());

        self.r_index.modify(device, config, shader, buffer_list, index_list, buffer_layouts)?;
        self.r_instance.modify(device, inst_list)?;

        Ok(())
    }
}

impl OnDeserialization for InstanceIndex {
    fn init(
        &mut self, 
        device: &Device,
        config: &SurfaceConfiguration,
        shader: &Shader,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>
    ) -> Result<(), std::io::Error> {
        
        self.r_index.init(device, config, shader, buffer_layouts)?;
        self.r_instance.init(device, config, shader, buffer_layouts)?;

        Ok(())
    }
}