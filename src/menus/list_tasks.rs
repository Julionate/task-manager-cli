use crate::{enums::AppState, models::tasks::Tasks, utils::input::read_input};

pub fn list_tasks(tasks: &Tasks) -> AppState {
    tasks.list_tasks();
    println!("Press 'enter' to continue...");
    read_input();
    AppState::Main
}
