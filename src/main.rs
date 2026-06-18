use std::io::stdin;
fn main() {
    loop {
        println!("Welcome to Task Manager CLI, choose an option");
        println!("1. Manage Tasks\n2. List Tasks\n3. Save Tasks\n'Exit' to leave");
        let input = read_input();

        match input.to_lowercase().trim() {
            "1" => {
                println!("You choose Manage tasks")
            }
            "2" => {
                println!("You choose list tasks")
            }
            "3" => {
                println!("You choose save tasks")
            }
            "exit" => {
                println!("Closing Task Manager CLI...");
                break;
            }
            _ => {}
        }
    }
}

fn read_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    println!("You entered: {}", buffer);
    buffer
}
