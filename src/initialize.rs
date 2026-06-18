use crate::app_state::AppState;
use crate::menus::*;

pub fn init() {
    let mut state = AppState::MainMenu;
    // This function manages the main loop of the program and displays
    // orchestates the flow
    loop {
        state = match state {
            AppState::MainMenu => main_menu(),
            AppState::ManageTasks => manage_task_menu(),
            AppState::ListTasks => list_task_menu(),
            AppState::SaveTasks => save_task_menu(),
            AppState::Exit => {
                exit_program();
                break;
            }
            _ => {
                break;
            }
        }
    }
}
