mod r_vertex;
mod r_index;
mod r_inst_index;

pub use self::r_vertex::Vertex;
pub use self::r_index::Index;
pub use self::r_inst_index::InstanceIndex;

pub trait OnDeserialization {
    fn init(
        &mut self, 
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &crate::graphics::Shader,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>
    ) -> Result<(), std::io::Error>;
}