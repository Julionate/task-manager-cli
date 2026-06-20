use super::task::Task;
use crate::enums::TaskStatus;

pub struct Tasks {
    tasks: Vec<Task>,
    last_id: u32,
}

impl Tasks {
    pub fn init() -> Self {
        // To-Do: Read from saved JSON File

        Self {
            tasks: vec![],
            last_id: 0,
        }
    }

    pub fn new_task(&mut self, description: String) {
        let task = Task::new(self.last_id, description, TaskStatus::ToDo);
        self.tasks.push(task);
        self.last_id += 1
    }

    pub fn list_tasks(&self) {
        for task in &self.tasks {
            println!("#######################");
            println!("ID: {}", task.get_id());
            println!("Description: {}", task.get_description());
            println!("Status: {}", task.get_status_string());
            println!("#######################");
        }
    }
}
