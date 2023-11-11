mod app;
mod util;

fn main() {
    let app = app::Application::new(
            800, 
            600, 
            "Iguana Eye", 
            ".\\resource\\icon\\app\\IguanaEye.png");

    pollster::block_on(app.run());
}
