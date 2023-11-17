use std::io;
use wgpu::util::DeviceExt;
use serde::{Serialize, Deserialize};

use super::{VertexRenderable, OnDeserialization};
use crate::graphics::{Vertex, Shader};

#[derive(Serialize, Deserialize)]
pub struct IndexRenderable {
    #[serde(skip)]
    pub index_buffer: Option<wgpu::Buffer>,

    pub v_renderable: VertexRenderable,
    pub index_count: u32,
    pub index_list: Vec<u16>,
}

impl IndexRenderable {
    pub fn new(
        hash: u64,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &Shader,
        vertex_list: Vec<Vertex>,
        index_list: Vec<u16>,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>,
    ) -> Result<Self, io::Error> {
        
        let v_renderable = VertexRenderable::new(hash, device, config, shader, vertex_list, buffer_layouts)?;
        let index_buffer = Some(IndexRenderable::create_index_buffer(device, &index_list));
        let index_count = index_list.len() as u32;

        Ok(Self {
            v_renderable,
            index_buffer,
            index_list,
            index_count
        })
    }

    pub fn modify(
        &mut self,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &Shader,
        vertex_list: Vec<Vertex>,
        index_list: Vec<u16>,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>
    ) -> Result<(), io::Error> {

        self.v_renderable.modify(device, config, shader, vertex_list, buffer_layouts)?;
        self.index_buffer = Some(IndexRenderable::create_index_buffer(device, &index_list));
        self.index_count = index_list.len() as u32;
        self.index_list = index_list;

        Ok(())
    }

    fn create_index_buffer(
        device: &wgpu::Device,
        index_list: &Vec<u16>
    ) -> wgpu::Buffer {

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&index_list),
            usage: wgpu::BufferUsages::INDEX
        })
    }
}

impl OnDeserialization for IndexRenderable {
    fn init(
        &mut self, 
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &crate::graphics::Shader,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>
    ) -> Result<(), std::io::Error> {

        self.v_renderable.init(device, config, shader, buffer_layouts)?;
        self.index_buffer = Some(IndexRenderable::create_index_buffer(device, &self.index_list));

        Ok(())
    }
}