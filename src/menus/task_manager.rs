use crate::enums::AppState;
use crate::models::tasks::Tasks;
use crate::utils::input::read_input;

pub fn task_manager(tasks: &mut Tasks) -> AppState {
    println!("Select an action");
    println!("1. Create a new task");
    println!("2. Update a task");
    println!("3. Delete a task");
    println!("4. Go back");

    let input = read_input();

    match input.to_lowercase().trim() {
        "1" => create(tasks),
        "4" => AppState::Main,
        _ => task_manager(tasks),
    }
}

fn create(tasks: &mut Tasks) -> AppState {
    println!("### Creating a new task ###");
    println!("Task description: ");
    let input = read_input();
    tasks.new_task(input.trim().to_string());

    AppState::TaskManager
}
