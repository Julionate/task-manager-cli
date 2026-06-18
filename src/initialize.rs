use crate::menus::*;

pub fn init() {
    // This function manages the main loop of the program and displays
    // orchestates the flow
    loop {
        match main_menu() {
            MenuActions::ManageTasks => manage_task_menu(),
            MenuActions::ListTasks => list_task_menu(),
            MenuActions::SaveTasks => save_task_menu(),
            MenuActions::Exit => {
                exit_program();
                break;
            }
        }
    }
}
