use image::GenericImageView;
use winit::dpi::PhysicalSize;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::{Window, WindowBuilder, Icon, Fullscreen};
use winit::event::{Event, WindowEvent, KeyboardInput, ElementState, ModifiersState};

use super::Config;
use crate::util::file;
use crate::util::serialize;

pub struct Application {
    pub width: u32,
    pub height: u32,
    pub window: Window,
    pub event_loop: EventLoop<()>,
}

impl Application {
    pub fn new(config_path: &str, icon_path: &str) -> Self {
        
        env_logger::init();

        let config: Config = match serialize::read(config_path) {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{e} - [app::application::new()][config]");
                Config::default()
            }
        };

        // Create title bar icon
        let icon = match file::absolute_path(icon_path) {
            Ok(abs_path) => {
                match std::fs::read(abs_path) {
                    Ok(img_bytes) => {
                        match image::load_from_memory(&img_bytes) {
                            Ok(img) => {
                                let img_rgba = img.to_rgba8();
                                let img_size = img.dimensions();
                                match Icon::from_rgba(img_rgba.into_vec(), img_size.0, img_size.1) {
                                    Ok(icon) => Some(icon),
                                    Err(_) => None
                                }
                            },
                            Err(_) => None
                        }
                    },
                    Err(_) => None
                }
            },
            Err(_) => None
        };

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_window_icon(icon)
            .with_title("Iguana Eye")
            .with_inner_size(winit::dpi::LogicalSize::new(config.width, config.height))
            .build(&event_loop)
            .expect("Failed to create a window");

        if config.fullscreen {
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
        }

        Self {
            width: config.width,
            height: config.height,
            window,
            event_loop,
        }
    }

    pub async fn run(self) {
        self.event_loop.run(move |event, _, cf| {
            match event {
                Event::MainEventsCleared => self.window.request_redraw(),
                Event::RedrawEventsCleared => Application::handle_update(),
                Event::WindowEvent { event, .. } => {
                    Application::handle_window_event(event, cf)
                },
                _ => {}
            };
        });
    }

    fn handle_window_event(event: WindowEvent, cf: &mut ControlFlow) {
        match event {
            WindowEvent::Resized(size) => Application::handle_resize(size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => Application::handle_scale_factor(new_inner_size),
            WindowEvent::CloseRequested => cf.set_exit(),
            WindowEvent::KeyboardInput { input, .. } => Application::handle_input(input),
            WindowEvent::ModifiersChanged(m) => { Application::handle_modifiers(m) },
            _ => {}
        }
    }

    fn handle_update() {
        // Logic and Rendering happens here
    }

    fn handle_resize(_size: PhysicalSize<u32>) {
        
    }

    fn handle_scale_factor(_size: &mut PhysicalSize<u32>) {

    }

    fn handle_input(input: KeyboardInput) {
        match input.state {
            ElementState::Pressed => { },
            ElementState::Released => { }
        }
    }

    fn handle_modifiers(m: ModifiersState) {
        match m {
            ModifiersState::ALT => {},
            ModifiersState::CTRL => {},
            ModifiersState::SHIFT => {},
            ModifiersState::LOGO => {},
            _ => {}
        }
    }
}