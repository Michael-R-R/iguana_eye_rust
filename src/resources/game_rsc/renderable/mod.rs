mod r_vertex;
mod r_index;
mod r_instance;
mod r_inst_vertex;
mod r_inst_index;

pub use self::r_vertex::Vertex;
pub use self::r_index::Index;
pub use self::r_instance::Instance;
pub use self::r_inst_vertex::InstanceVertex;
pub use self::r_inst_index::InstanceIndex;

pub trait Deserialized {
    fn init(
        &mut self, 
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &crate::resources::game_rsc::shader::Shader,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>,
        bind_layouts: &Vec<&wgpu::BindGroupLayout>
    ) -> Result<(), std::io::Error>;
}