use image::GenericImageView;
use winit::dpi::PhysicalSize;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::{Window, WindowBuilder, Icon, Fullscreen};
use winit::event::{Event, WindowEvent, KeyboardInput, ModifiersState, MouseButton, ElementState};

use super::Config;
use super::Viewport;
use super::Frame;
use super::Time;
use crate::game::Game;
use crate::editor::UI;
use crate::util::file;
use crate::util::serialize;

pub struct Application {
    pub width: u32,
    pub height: u32,
    pub window: Window,
    pub viewport: Viewport,
    pub game: Game,
    pub ui: UI,
}

impl Application {
    pub async fn new(config_path: &str, icon_path: &str, event_loop: &EventLoop<()>) -> Self {
        
        // Load app config file
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

        let window = WindowBuilder::new()
            .with_window_icon(icon)
            .with_title("Iguana Eye")
            .with_inner_size(winit::dpi::LogicalSize::new(config.width, config.height))
            .build(&event_loop)
            .expect("Failed to create a window");

        if config.fullscreen {
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
        }

        let width = config.width;
        let height = config.height;
        let viewport = Viewport::new(&window).await;
        let game = Game::new();
        let ui = UI::new(&window, &viewport);

        Self {
            width,
            height,
            window,
            viewport,
            game,
            ui,
        }
    }

    pub async fn run(mut self, event_loop: EventLoop<()>) {
        let mut time = Time::new();

        event_loop.run(move |event, _, cf| {
            self.handle_event(&event);

            match event {
                Event::MainEventsCleared => self.window.request_redraw(),
                Event::RedrawEventsCleared => {
                    let dt = time.update();
                    self.handle_update(dt);
                    self.handle_render(dt);
                },
                Event::WindowEvent { event, .. } => {
                    self.handle_window_event(event, cf)
                },
                _ => {}
            };
        });
    }

    fn handle_window_event(&mut self, event: WindowEvent, cf: &mut ControlFlow) {
        match event {
            WindowEvent::CloseRequested => cf.set_exit(),
            WindowEvent::Resized(size) => self.handle_resize(size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => self.handle_resize(*new_inner_size),
            WindowEvent::ModifiersChanged(m) => { self.handle_modifiers(m) },
            WindowEvent::KeyboardInput { input, .. } => self.handle_kb_input(input),
            WindowEvent::MouseInput { state, button, .. } => self.handle_mb_input(state, button),
            _ => {}
        }
    }

    fn handle_event(&mut self, event: &Event<'_, ()>) {
        self.ui.handle_event(&self.window, event)
    }

    fn handle_update(&mut self, dt: f32) {
        let window = &self.window;
        self.game.handle_update(window, dt);
        self.ui.handle_update(window, dt);
    }

    fn handle_render(&mut self, dt: f32) {
        let window = &self.window;
        let viewport = &self.viewport;
        let mut frame = Frame::begin(&self.viewport);
        {
            self.game.handle_render(window, viewport, &mut frame, dt);
            self.ui.handle_render(window, viewport, &self.game, &mut frame, dt);
        }
        frame.end(&self.viewport);
    }

    fn handle_resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.viewport.resize(size);
            self.game.handle_resize(size);
            self.ui.handle_resize(size);
        }
    }

    fn handle_modifiers(&mut self, m: ModifiersState) {
        self.game.handle_modifiers(&m);
        self.ui.handle_modifiers(&m);
    }

    fn handle_kb_input(&mut self, input: KeyboardInput) {
        self.game.handle_kb_input(&input);
        self.ui.handle_kb_input(&input);
    }

    fn handle_mb_input(&mut self, state: ElementState, input: MouseButton) {
        self.game.handle_mb_input(&state, &input);
        self.ui.handle_mb_input(&state, &input);
    }

}