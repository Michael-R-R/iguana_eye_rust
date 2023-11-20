use std::io;
use wgpu::util::DeviceExt;
use serde::{Serialize, Deserialize};

use crate::graphics::{VertexBuffer, Shader, Layout};

#[derive(Serialize, Deserialize)]
pub struct Vertex {
    pub hash: u64,
    pub shader_hash: u64,
    pub buffer_list: Vec<VertexBuffer>,

    #[serde(skip)]
    pub pipeline: Option<wgpu::RenderPipeline>,

    #[serde(skip)]
    pub vertex_buffer: Option<wgpu::Buffer>,
}

impl Vertex {
    pub fn new(
        hash: u64,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &Shader,
        buffer_list: Vec<VertexBuffer>,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>,
    ) -> Result<Self, io::Error> {

        let pipeline = Some(Vertex::create_pipeline(device, config, shader, buffer_layouts)?);
        let vertex_buffer = Some(Vertex::create_vertex_buffer(device, &buffer_list));
        let shader_hash = shader.hash;

        Ok(Self {
            pipeline,
            vertex_buffer,
            hash,
            shader_hash,
            buffer_list,
        })
    }

    pub fn modify(
        &mut self,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &Shader,
        buffer_list: Vec<VertexBuffer>,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>,
    ) -> Result<(), io::Error> {

        self.pipeline = Some(Vertex::create_pipeline(device, config, shader, buffer_layouts)?);
        self.vertex_buffer = Some(Vertex::create_vertex_buffer(device, &buffer_list));
        self.shader_hash = shader.hash;

        Ok(())
    }

    fn create_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &Shader,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>,
    ) -> Result<wgpu::RenderPipeline, io::Error> {

            buffer_layouts.insert(0, VertexBuffer::layout());

            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[]
            });

            let shader_module = match &shader.module {
                Some(val) => val,
                None => {
                    return Err(io::Error::new(io::ErrorKind::InvalidData,
                        "ERROR::renderable::modify()::shader module invalid"))
                }
            };

            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: "vs_main",
                    buffers: &buffer_layouts,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0, // all masks
                    alpha_to_coverage_enabled: false, // anti-aliasing
                },
                multiview: None,
            });

            Ok(pipeline)
    }

    fn create_vertex_buffer(
        device: &wgpu::Device,
        buffer_list: &Vec<VertexBuffer>
    ) -> wgpu::Buffer {

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&buffer_list),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
}

impl super::OnDeserialization for Vertex {
    fn init(
        &mut self, 
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &crate::graphics::Shader,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>
    ) -> Result<(), std::io::Error> {
        
        self.pipeline = Some(Vertex::create_pipeline(device, config, shader, buffer_layouts)?); 
        self.vertex_buffer = Some(Vertex::create_vertex_buffer(device, &self.buffer_list));

        Ok(())
    }
}