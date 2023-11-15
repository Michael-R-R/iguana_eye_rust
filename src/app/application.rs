use image::GenericImageView;
use winit::dpi::PhysicalSize;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::{Window, WindowBuilder, Icon, Fullscreen};
use winit::event::{Event, WindowEvent, KeyboardInput, ModifiersState};

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

        let viewport = Viewport::new(&window).await;
        let game = Game::new();
        let ui = UI::new(&window, &viewport);

        Self {
            width: config.width,
            height: config.height,
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
            WindowEvent::Resized(size) => self.handle_resize(size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => self.handle_resize(*new_inner_size),
            WindowEvent::CloseRequested => cf.set_exit(),
            WindowEvent::KeyboardInput { input, .. } => self.handle_input(input),
            WindowEvent::ModifiersChanged(m) => { self.handle_modifiers(m) },
            _ => {}
        }
    }

    fn handle_event(&mut self, event: &Event<'_, ()>) {
        self.ui.handle_event(&self.window, event)
    }

    fn handle_update(&mut self, dt: f32) {
        let window = &self.window;
        self.game.update(window, dt);
        self.ui.update(window, dt);
    }

    fn handle_render(&mut self, dt: f32) {
        let window = &self.window;
        let viewport = &self.viewport;
        let mut frame = Frame::begin(&self.viewport);
        {
            let mut rp = frame.render_pass();
            self.game.render(window, &rp, dt);
            self.ui.render(window, viewport, &self.game, &mut rp, dt);
        }
        frame.end(&self.viewport);
    }

    fn handle_resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.viewport.resize(size);
            self.game.resize(size);
            self.ui.resize(size);
        }
    }

    fn handle_input(&self, input: KeyboardInput) {
        self.game.input(&input);
        self.ui.input(&input);
    }

    fn handle_modifiers(&self, m: ModifiersState) {
        self.game.modifiers(&m);
        self.ui.modifiers(&m);
    }
}