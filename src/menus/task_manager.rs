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
        "2" => update(tasks),
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

fn update(tasks: &mut Tasks) -> AppState {
    let number_of_tasks = tasks.number_of_tasks();

    if number_of_tasks == 0 {
        println!("There's no tasks, create one a try again...");
        return AppState::TaskManager;
    }

    tasks.list_tasks();
    println!("Please select an ID: ");

    loop {
        let input = read_input();
        match input.trim().parse::<u32>() {
            Ok(id) => {
                if id >= number_of_tasks as u32 {
                    println!("ID doesn't exist, try again");
                    continue;
                }
                println!("Write the new input: ");
                let new_description = read_input();
                tasks.update_task(id, new_description);
                break;
            }
            Err(_) => {
                println!("Invalid ID number, try again");
                continue;
            }
        }
    }

    AppState::TaskManager
}
