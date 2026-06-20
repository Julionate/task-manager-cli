use crate::enums::AppState;
use crate::utils::input::read_input;

pub fn main_menu() -> AppState {
    println!("What you want to do...?");
    println!("1. Manage tasks");
    println!("2. List tasks");
    println!("3. Save tasks");
    println!("4. Exit");

    let input = read_input();

    match input.to_lowercase().trim() {
        "1" => AppState::TaskManager,
        "2" => AppState::TasksList,
        "3" => AppState::TasksSave,
        "4" => AppState::Exit,
        _ => main_menu(),
    }
}
