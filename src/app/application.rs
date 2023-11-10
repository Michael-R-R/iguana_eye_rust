use image::GenericImageView;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder, Icon};
use winit::event::{Event, WindowEvent, VirtualKeyCode};

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

        let img_bytes = std::fs::read(icon_path).expect("ERROR::Application::could not read file...");
        let img = image::load_from_memory(&img_bytes).expect("ERROR::Application::could not load app image from memory...");
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
                _ => {}
            }
        });
    }
}