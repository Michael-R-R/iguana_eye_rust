use std::io;

use serde::{Serialize, Deserialize};
use wgpu::{Device, SurfaceConfiguration};

use super::{OnDeserialization, Vertex, Instance};
use crate::graphics::shader::Shader;
use crate::graphics::buffer::{VertexBuffer, InstanceBuffer, Layout};

#[derive(Serialize, Deserialize)]
pub struct InstanceVertex {
    pub r_vertex: Vertex,
    pub r_instance: Instance,
}

impl InstanceVertex {
    pub fn new(
        hash: u64,
        device: &Device,
        config: &SurfaceConfiguration,
        shader: &Shader,
        buffer_list: Vec<VertexBuffer>,
        inst_list: Vec<InstanceBuffer>,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>,
        bind_layouts: &Vec<&wgpu::BindGroupLayout>
    ) -> Result<Self, io::Error> {

        buffer_layouts.insert(0, InstanceBuffer::layout());

        let r_vertex = Vertex::new(hash, device, config, shader, buffer_list, buffer_layouts, bind_layouts)?;
        let r_instance = Instance::new(device, inst_list)?;

        Ok(Self {
            r_vertex,
            r_instance,
        })
    }

    pub fn modify(
        &mut self,
        device: &Device,
        config: &SurfaceConfiguration,
        shader: &Shader,
        buffer_list: Vec<VertexBuffer>,
        inst_list: Vec<InstanceBuffer>,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>,
        bind_layouts: &Vec<&wgpu::BindGroupLayout>
    ) -> Result<(), io::Error> {

        buffer_layouts.insert(0, InstanceBuffer::layout());

        self.r_vertex.modify(device, config, shader, buffer_list, buffer_layouts, bind_layouts)?;
        self.r_instance.modify(device, inst_list)?;

        Ok(())
    }
}

impl OnDeserialization for InstanceVertex {
    fn init(
        &mut self, 
        device: &Device,
        config: &SurfaceConfiguration,
        shader: &Shader,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>,
        bind_layouts: &Vec<&wgpu::BindGroupLayout>
    ) -> Result<(), std::io::Error> {

        self.r_vertex.init(device, config, shader, buffer_layouts, bind_layouts)?;
        self.r_instance.init(device, config, shader, buffer_layouts, bind_layouts)?;

        Ok(())
    }
}