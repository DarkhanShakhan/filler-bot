mod app;
mod bot;
mod util;
fn main() {
    let mut app = app::App::default();
    app.start();
}
