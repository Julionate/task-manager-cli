use crate::enums::AppState;
use crate::utils::input::read_input;

pub fn task_manager() -> AppState {
    println!("Select an action");
    println!("1. Create a new task");
    println!("2. Update a task");
    println!("3. Delete a task");
    println!("4. Go back");

    let input = read_input();

    match input.to_lowercase().trim() {
        "4" => AppState::Main,
        _ => task_manager(),
    }
}
