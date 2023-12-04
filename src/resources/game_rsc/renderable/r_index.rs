use std::io;
use wgpu::util::DeviceExt;
use serde::{Serialize, Deserialize};

use super::{Vertex, Deserialized};
use crate::resources::game_rsc::{buffer::VertexBuffer, shader::Shader};

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub r_vertex: Vertex,
    pub index_count: u32,
    pub index_list: Vec<u16>,

    #[serde(skip)]
    pub index_buffer: Option<wgpu::Buffer>,
}

impl Index {
    pub fn new(
        hash: u64,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &Shader,
        buffer_list: Vec<VertexBuffer>,
        index_list: Vec<u16>,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>,
        bind_layouts: &Vec<&wgpu::BindGroupLayout>
    ) -> Result<Self, io::Error> {
        
        let r_vertex = Vertex::new(hash, device, config, shader, buffer_list, buffer_layouts, bind_layouts)?;
        let index_buffer = Some(Index::create_index_buffer(device, &index_list));
        let index_count = index_list.len() as u32;

        Ok(Self {
            r_vertex,
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
        buffer_list: Vec<VertexBuffer>,
        index_list: Vec<u16>,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>,
        bind_layouts: &Vec<&wgpu::BindGroupLayout>
    ) -> Result<(), io::Error> {

        self.r_vertex.modify(device, config, shader, buffer_list, buffer_layouts, bind_layouts)?;
        self.index_buffer = Some(Index::create_index_buffer(device, &index_list));
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

impl Deserialized for Index {
    fn init(
        &mut self, 
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &Shader,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>,
        bind_layouts: &Vec<&wgpu::BindGroupLayout>
    ) -> Result<(), std::io::Error> {

        self.r_vertex.init(device, config, shader, buffer_layouts, bind_layouts)?;
        self.index_buffer = Some(Index::create_index_buffer(device, &self.index_list));

        Ok(())
    }
}