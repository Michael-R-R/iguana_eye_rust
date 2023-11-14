use winit::{window::Window, dpi::PhysicalSize, event::{KeyboardInput, ModifiersState}};
use crate::app::Viewport;

pub struct Game {
}

impl Game {
    pub fn new() -> Self {
        Self { }
    }

    pub fn update(&self, _window: &Window, _dt: f32) {

    }

    pub fn render(&self, _window: &Window, viewport: &Viewport, _dt: f32) -> Result<(), wgpu::SurfaceError> {
        let frame = viewport.surface.get_current_texture()?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = viewport.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });

        {
            let mut _rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2, g: 0.2, b: 0.2, a: 1.0,
                        }),
                        store: true,
                    }
                })],
                depth_stencil_attachment: None,
            });

            // --- Draw here --- //

            // ----------------- //
        }

        viewport.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }

    pub fn resize(&self, _size: PhysicalSize<u32>) {

    }

    pub fn input(&self, _input: &KeyboardInput) {
        
    }

    pub fn modifiers(&self, _mod: &ModifiersState) {

    }
}