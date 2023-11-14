use wgpu::{SurfaceTexture, TextureView, CommandEncoder, RenderPass};
use winit::{window::Window, dpi::PhysicalSize};

pub struct Viewport {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

pub struct Frame {
    frame: SurfaceTexture,
    view: TextureView,
    encoder: CommandEncoder,
}

impl Viewport {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = match unsafe { instance.create_surface(&window) } {
            Ok(val) => val,
            Err(e) => panic!("{e}"),
        };

        let adapter = match instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await {
            Some(val) => val,
            None => panic!("ERROR::game::viewport::new()::cannot create adapter")
        };

        let (device, queue) = match adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
            },
            None
        ).await {
            Ok(val) => val,
            Err(e) => panic!("{e}")
        };

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        Self {
            size,
            surface, 
            device, 
            queue,
            config,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }
}

impl Frame {
    pub fn begin(viewport: &Viewport) -> Self {
        let frame = viewport.surface.get_current_texture()
            .expect("ERROR::app::viewport::frame::begin()::cannot get current texture");

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let encoder = viewport.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });

        Self { frame, view, encoder }
    }

    pub fn render_pass(&mut self) -> RenderPass {
        self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.2, g: 0.2, b: 0.2, a: 1.0,
                    }),
                    store: true,
                }
            })],
            depth_stencil_attachment: None,
        })
    }

    pub fn end(self, viewport: &Viewport) {
        viewport.queue.submit(std::iter::once(self.encoder.finish()));
        self.frame.present();
    }
}