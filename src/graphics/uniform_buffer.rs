use cgmath::SquareMatrix;
use wgpu::{util::DeviceExt, BindGroupLayout, Device, Buffer, BindGroup};
use serde::{Serialize, Deserialize};

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
#[derive(Serialize, Deserialize)]
pub struct CameraUBuffer {
    pub view_projection: [[f32; 4]; 4]
}

impl CameraUBuffer {
    pub fn new(device: &wgpu::Device) -> (Self, wgpu::Buffer) {
        let view_projection = cgmath::Matrix4::identity().into();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[view_projection]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        return (Self{view_projection}, buffer)
    }

    pub fn layout(device: &wgpu::Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ]
        })
    }

    pub fn bind_group(
        device: &Device, 
        layout: &BindGroupLayout,
        buffer: &Buffer
    ) -> BindGroup {

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
            ],
        })
    }
}