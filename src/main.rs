mod app;
mod util;

fn main() {
    let app = app::Application::new(
        ".\\config\\app\\config.json",
        ".\\resource\\icon\\app\\IguanaEye.png");  

    pollster::block_on(app.run());
}
