use crate::app::App;

mod app;
mod enums;
mod menus;
mod models;
mod utils;

fn main() {
    let mut app = App::new();
    app.run();
}
