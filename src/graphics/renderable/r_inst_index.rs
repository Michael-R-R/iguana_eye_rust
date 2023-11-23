use std::io;

use serde::{Serialize, Deserialize};
use wgpu::{Device, SurfaceConfiguration, util::DeviceExt, Queue, BufferUsages, BufferAddress};

use super::{Index, OnDeserialization};
use crate::graphics::{InstanceBuffer, Shader, VertexBuffer, Layout};

#[derive(Serialize, Deserialize)]
pub struct InstanceIndex {
    pub r_index: Index,
    pub inst_list: Vec<InstanceBuffer>,

    #[serde(skip)]
    pub inst_buffer: Option<wgpu::Buffer>,
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
        let inst_buffer = Some(InstanceIndex::create_inst_buffer(device, &inst_list));

        Ok(Self {
            r_index,
            inst_list,
            inst_buffer,
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
        self.inst_buffer = Some(InstanceIndex::create_inst_buffer(device, &inst_list));
        self.inst_list = inst_list;

        Ok(())
    }

    pub fn add_instance(&mut self, device: &Device, data: InstanceBuffer) -> usize {
        let index = self.inst_list.len();
        
        self.inst_list.push(data);
        self.inst_buffer = Some(InstanceIndex::create_inst_buffer(device, &self.inst_list));

        return index;
    }

    pub fn remove_instance(&mut self, device: &Device, index: usize) -> Result<usize, io::Error> {
        if !self.bounds_check(index) {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                "ERROR::r_inst_index::remove_instance()::index out of bounds"))
        }

        let last = self.inst_list.len() - 1;
        self.inst_list[index] = self.inst_list[last];
        self.inst_list.pop();

        self.inst_buffer = Some(InstanceIndex::create_inst_buffer(device, &self.inst_list));

        Ok(last)
    }

    pub fn modify_instance(&mut self, queue: &Queue, index: usize, data: InstanceBuffer) -> Result<(), io::Error> {

        if !self.bounds_check(index) {
            return Err(io::Error::new(io::ErrorKind::Other, 
                "ERROR::r_inst_index::modify_instance()::Out of bounds check"))
        }

        match &self.inst_buffer {
            Some(buffer) => {
                queue.write_buffer(
                    buffer, 
                    (index * std::mem::size_of::<InstanceBuffer>()) as BufferAddress, 
                    bytemuck::cast_slice(&[data]));
            },
            None => {
                return Err(io::Error::new(io::ErrorKind::Other, 
                    "ERROR::r_inst_index::modify_instance()::Invalid instance buffer"))
            }
        }
        Ok(())
    }

    fn create_inst_buffer(
        device: &Device,
        inst_list: &Vec<InstanceBuffer>
    ) -> wgpu::Buffer {

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&inst_list),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
    }

    fn bounds_check(&self, index: usize) -> bool {
        return index <= self.inst_list.len() - 1;
    }
}

impl OnDeserialization for InstanceIndex {
    fn init(
        &mut self, 
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &crate::graphics::Shader,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>
    ) -> Result<(), std::io::Error> {
        
        self.r_index.init(device, config, shader, buffer_layouts)?;
        self.inst_buffer = Some(InstanceIndex::create_inst_buffer(device, &self.inst_list));

        Ok(())
    }
}