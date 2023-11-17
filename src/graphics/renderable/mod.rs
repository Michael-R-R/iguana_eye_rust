mod v_renderable;
mod i_renderable;

pub use self::v_renderable::VertexRenderable;
pub use self::i_renderable::IndexRenderable;

pub trait OnDeserialization {
    fn init(
        &mut self, 
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &crate::graphics::Shader,
        buffer_layouts: &mut Vec<wgpu::VertexBufferLayout<'static>>
    ) -> Result<(), std::io::Error>;
}