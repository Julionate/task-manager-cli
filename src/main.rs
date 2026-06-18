mod app_state;
mod initialize;
mod input;
mod menus;

fn main() {
    println!("Welcome to Task Manager CLI, choose an option");
    initialize::init();
}
