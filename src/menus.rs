use crate::app_state::AppState;
use crate::input::read_input;

pub enum MenuActions {
    ManageTasks,
    ListTasks,
    SaveTasks,
    Exit,
}

pub fn main_menu() -> AppState {
    println!("1. Manage Tasks");
    println!("2. List Tasks");
    println!("3. Save Tasks");
    println!("'Exit' to leave");
    let input = read_input();

    match input.to_lowercase().trim() {
        "1" => AppState::ManageTasks,
        "2" => AppState::ListTasks,
        "3" => AppState::SaveTasks,
        "exit" => AppState::Exit,
        _ => {
            println!("Invalid Operation");
            main_menu()
        }
    }
}

pub fn manage_task_menu() -> AppState {
    println!("1. Create");
    println!("2. Edit");
    println!("3. Delete");
    println!("'Back' to go back");
    let input = read_input();

    match input.to_lowercase().trim() {
        "1" => AppState::CreateTask,
        "2" => AppState::EditTask,
        "3" => AppState::DeleteTask,
        "back" => AppState::MainMenu,
        _ => AppState::ManageTasks,
    }
}
pub fn list_task_menu() -> AppState {
    AppState::ListTasks
}
pub fn save_task_menu() -> AppState {
    AppState::SaveTasks
}
pub fn exit_program() -> AppState {
    println!("Closing Task Manager CLI, goodbye friend!");
    AppState::Exit
}
