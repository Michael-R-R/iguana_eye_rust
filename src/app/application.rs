use image::GenericImageView;
use winit::dpi::PhysicalSize;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::{Window, WindowBuilder, Icon};
use winit::event::{Event, WindowEvent, KeyboardInput, ElementState, ModifiersState};

use crate::file;

pub struct Application {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub window: Window,
    pub event_loop: EventLoop<()>,
}

impl Application {
    pub fn new(w: u32, h: u32, title: &str, icon_path: &str) -> Self {
        
        env_logger::init();

        // Create the title bar icon
        let abs_path = match file::absolute_path(icon_path) {
            Some(i) => i,
            None => panic!("ERROR::Application::new()::could not read icon absolute path - [{icon_path}]")
        };
        let img_bytes = std::fs::read(abs_path).expect("ERROR::Application::new()::could not read icon file");
        let img = image::load_from_memory(&img_bytes).expect("ERROR::Application::new()::could not load icon image from memory");
        let img_rgba = img.to_rgba8();
        let img_size = img.dimensions();
        let icon = match Icon::from_rgba(img_rgba.into_vec(), img_size.0, img_size.1) {
            Ok(i) => Some(i),
            Err(e) => {
                println!("{e}");
                None
            },
        };

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_window_icon(icon)
            .with_title("Iguana Eye")
            .with_inner_size(winit::dpi::LogicalSize::new(w, h))
            .build(&event_loop)
            .expect("Failed to create a window");

        Self {
            width: w,
            height: h,
            title: String::from(title),
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
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => Application::handle_scl_fct_change(new_inner_size),
            WindowEvent::CloseRequested => cf.set_exit(),
            WindowEvent::KeyboardInput { input, .. } => Application::handle_input(input),
            WindowEvent::ModifiersChanged(m) => { Application::handle_mod_change(m) },
            _ => {}
        }
    }

    fn handle_update() {
        // Logic and Rendering happens here
    }

    fn handle_resize(_size: PhysicalSize<u32>) {

    }

    fn handle_scl_fct_change(_size: &mut PhysicalSize<u32>) {

    }

    fn handle_input(input: KeyboardInput) {
        match input.state {
            ElementState::Pressed => { },
            ElementState::Released => { },
            _ => {}
        }
    }

    fn handle_mod_change(m: ModifiersState) {
        match m {
            ModifiersState::ALT => {},
            ModifiersState::CTRL => {},
            ModifiersState::SHIFT => {},
            ModifiersState::LOGO => {},
            _ => {}
        }
    }
}