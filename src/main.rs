use winit::event_loop::EventLoop;

mod app;
mod game;
mod editor;
mod util;

fn main() {
    
    env_logger::init();
    
    let event_loop = EventLoop::new();
    let app = pollster::block_on(app::Application::new(
        ".\\config\\app\\config.json",
        ".\\resource\\icon\\app\\IguanaEye.png",
        &event_loop));  

    pollster::block_on(app.run(event_loop));
}

