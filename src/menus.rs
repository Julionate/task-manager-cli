use crate::input::read_input;

pub enum MenuActions {
    ManageTasks,
    ListTasks,
    SaveTasks,
    Exit,
}

pub fn main_menu() -> MenuActions {
    println!("1. Manage Tasks\n2. List Tasks\n3. Save Tasks\n'Exit' to leave");
    let input = read_input();

    match input.to_lowercase().trim() {
        "1" => MenuActions::ManageTasks,
        "2" => MenuActions::ListTasks,
        "3" => MenuActions::SaveTasks,
        "exit" => MenuActions::Exit,
        _ => {
            println!("Invalid Operation");
            main_menu()
        }
    }
}

pub fn manage_task_menu() {
    println!("Choose a new option...")
}
pub fn list_task_menu() {
    println!("Showing tasks...")
}
pub fn save_task_menu() {
    println!("Are you sure to save?")
}
pub fn exit_program() {
    println!("Closing Task Manager CLI, goodbye!")
}
