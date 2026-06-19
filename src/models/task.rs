use crate::enums::TaskStatus;

pub struct Task {
    id: u32,
    description: String,
    status: TaskStatus,
}

impl Task {
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_status_string(&self) -> String {
        match self.status {
            TaskStatus::Done => String::from("Done"),
            TaskStatus::InProgress => String::from("In Progress"),
            TaskStatus::ToDo => String::from("To-Do"),
        }
    }

    pub fn new(id: u32, description: String, status: TaskStatus) -> Self {
        Self {
            id,
            description,
            status,
        }
    }
}
