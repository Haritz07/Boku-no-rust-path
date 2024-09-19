use std::io::{self}; 
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    title: String,
    status: Status,
    id: u64,
}

fn save_tasks(tasks: &Vec<Task>, file_path: &str) {
    // Convert the tasks to a JSON string
    let json = serde_json::to_string(tasks).expect("Failed to serialize tasks");

    // Write the JSON string to a file
    fs::write(file_path, json).expect("Unable to write file");
}

fn load_tasks(file_path: &str) -> Vec<Task> {
    if let Ok(json) = fs::read_to_string(file_path) {
        serde_json::from_str(&json).expect("Failed to deserialize tasks")
    } else {
        Vec::new() // Return an empty list if the file doesn't exist
    }
}

fn add_task(title: String, id: u64) -> Task {
    Task {
        title,
        status: Status::Pending,
        id,
    }
}

fn display_tasks(tasks: &[Task]) {
    for task in tasks {
        println!("ID: {}, Title: {}, Status: {:?}", task.id, task.title, task.status);
    }
}

fn update_task_status(task: &mut Task) {
    println!("Choose new status for task '{}':", task.title);
    println!("1. Pending");
    println!("2. In Progress");
    println!("3. Completed");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Couldn't read that, sorry!");

    match input.trim() {
        "1" => task.status = Status::Pending,
        "2" => task.status = Status::InProgress,
        "3" => task.status = Status::Completed,
        _ => println!("Invalid status"),
    }
}

fn main() {
    // Load tasks from file
    let file_path = "tasks.json";
    let mut todo: Vec<Task> = load_tasks(file_path); // Load tasks here

    let mut next_id = todo.len() as u64 + 1; // Set next ID based on the number of loaded tasks

    // Menu loop for adding tasks and interacting
    loop {
        println!("\n1. Add a task");
        println!("2. Display all tasks");
        println!("3. Update task status");
        println!("4. Delete task");
        println!("5. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Invalid choice");

        match choice.trim() {
            "1" => {
                // Add a task
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Invalid title");

                let task = add_task(title.trim().to_string(), next_id);
                todo.push(task); // Add task to the list
                next_id += 1;    // Increment the task ID for the next task

                println!("Task added!");
                continue; // Go back to menu after adding a task
            }

            "2" => {
                // Display all tasks
                if todo.is_empty() {
                    println!("No tasks available.");
                } else {
                    display_tasks(&todo);
                }
                continue;
            }

            "3" => {
                // Update task status
                println!("Enter task ID to update:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Error reading input");

                let id: u64 = match id_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID");
                        continue;
                    }
                };

                // Find and update the task
                if let Some(task) = todo.iter_mut().find(|t| t.id == id) {
                    update_task_status(task);
                    println!("Task status updated to '{:?}'", task.status);
                } else {
                    println!("Task with ID {} not found", id);
                }
                continue;
            }

            "4" => {
                println!("Enter task ID to delete");
                let mut id_del = String::new();
                io::stdin().read_line(&mut id_del).expect("Get serious bruv!");

                let id: u64 = match id_del.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID");
                        continue;
                    }
                };

                if let Some(index) = todo.iter().position(|t| t.id == id) {
                    todo.remove(index);
                    println!("Task deleted!");
                } else {
                    println!("ID not found");
                }
                continue;
            }

            "5" => {
                // Quit the program
                println!("Saving tasks...");
                save_tasks(&todo, file_path); // Save tasks before quitting
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid choice, please try again.");
                continue;
            }
        }
    }
}
