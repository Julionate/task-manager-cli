use crate::enums::AppState;
use crate::menus;
use crate::models::tasks::Tasks;

pub struct App {
    tasks: Tasks,
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            tasks: Tasks::init(),
            state: AppState::Main,
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to Task Manager CLI");

        loop {
            self.state = match self.state {
                AppState::Main => menus::main_menu(),
                AppState::TaskManager => menus::task_manager(&mut self.tasks),
                AppState::TasksList => menus::list_tasks(&self.tasks),
                AppState::Exit => {
                    menus::exit();
                    break;
                }
                _ => {
                    menus::exit();
                    break;
                }
            };
        }
    }
}
