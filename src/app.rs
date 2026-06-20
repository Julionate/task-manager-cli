use crate::enums::AppState;
use crate::menus;

pub struct App {
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Main,
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to Task Manager CLI");

        loop {
            self.state = match self.state {
                AppState::Main => menus::main_menu(),
                AppState::TaskManager => menus::task_manager(),
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
