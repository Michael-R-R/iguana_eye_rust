use std::io;
use serde::{Serialize, Deserialize};
use wgpu::{Device, util::DeviceExt, Queue, BufferAddress};

use super::OnDeserialization;
use crate::graphics::InstanceBuffer;

#[derive(Serialize, Deserialize)]
pub struct Instance {
    pub inst_list: Vec<InstanceBuffer>,

    #[serde(skip)]
    pub inst_buffer: Option<wgpu::Buffer>,
}

impl Instance {
    pub fn new(
        device: &Device,
        inst_list: Vec<InstanceBuffer>
    ) -> Result<Self, io::Error> {

        let inst_buffer = Some(Instance::create_inst_buffer(device, &inst_list));

        Ok(Self {
            inst_list,
            inst_buffer,
        })
    }

    pub fn modify(
        &mut self,
        device: &Device,
        inst_list: Vec<InstanceBuffer>
    ) -> Result<(), io::Error> {

        self.inst_buffer = Some(Instance::create_inst_buffer(device, &inst_list));
        self.inst_list = inst_list;

        Ok(())
    }

    pub fn add_instance(&mut self, device: &Device, data: InstanceBuffer) -> usize {
        let index = self.inst_list.len();
        
        self.inst_list.push(data);
        self.inst_buffer = Some(Instance::create_inst_buffer(device, &self.inst_list));

        return index;
    }

    pub fn modify_instance(
        &mut self, 
        queue: &Queue, 
        index: usize, 
        data: InstanceBuffer
    ) -> Result<(), io::Error> {

        if !self.bounds_check(index) {
            return Err(io::Error::new(io::ErrorKind::Other, 
                "ERROR::r_inst_index::modify_instance()::index out of bounds"))
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
                    "ERROR::r_inst_index::modify_instance()::invalid instance buffer"))
            }
        }

        Ok(())
    }

    pub fn remove_instance(&mut self, device: &Device, index: usize) -> Result<usize, io::Error> {
        if !self.bounds_check(index) {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                "ERROR::r_inst_index::remove_instance()::index out of bounds"))
        }

        let last = self.inst_list.len() - 1;
        self.inst_list[index] = self.inst_list[last];
        self.inst_list.pop();

        self.inst_buffer = Some(Instance::create_inst_buffer(device, &self.inst_list));

        Ok(last)
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

impl OnDeserialization for Instance {
    fn init(
        &mut self, 
        device: &wgpu::Device,
        _config: &wgpu::SurfaceConfiguration,
        _shader: &crate::graphics::Shader,
        _buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>
    ) -> Result<(), std::io::Error> {
        
        self.inst_buffer = Some(Instance::create_inst_buffer(device, &self.inst_list));

        Ok(())
    }
}