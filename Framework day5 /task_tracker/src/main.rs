
use std::io::{self, Write};

// Define Task struct
#[derive(Clone)]
struct Task {
    id: usize,
    title: String,
    completed: bool,
}

// Task Manager System
struct TaskManager {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskManager {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
        };
        self.next_id += 1;
        println!("[LOG] Task '{}' added as Task #{}", task.title, task.id);
        self.tasks.push(task);
    }

    fn complete_task(&mut self, task_id: usize) -> Result<(), String> {
        for task in &mut self.tasks {
            if task.id == task_id {
                if task.completed {
                    return Err("Task is already completed.".to_string());
                }
                task.completed = true;
                println!("[LOG] Task #{} marked as completed.", task.id);
                return Ok(());
            }
        }
        Err("Task ID not found.".to_string())
    }

    fn remove_task(&mut self, task_id: usize) -> Result<(), String> {
        let index = self.tasks.iter().position(|t| t.id == task_id);
        if let Some(i) = index {
            let removed = self.tasks.remove(i);
            println!("[LOG] Task #{} '{}' removed.", removed.id, removed.title);
            Ok(())
        } else {
            Err("Task ID not found.".to_string())
        }
    }

    fn list_tasks(&self) {
        println!("--- Task List ---");
        if self.tasks.is_empty() {
            println!("No tasks found.");
            return;
        }

        for task in &self.tasks {
            println!(
                "#{} [{}] - {}",
                task.id,
                if task.completed { "✔️ Done" } else { "⏳ Pending" },
                task.title
            );
        }
    }
}

fn main() {
    let mut manager = TaskManager::new();

    loop {
        println!("\n1. Add Task\n2. Complete Task\n3. Remove Task\n4. List Tasks\n5. Exit");
        print!("Choose: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                print!("Enter Task Title: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                manager.add_task(title.trim().to_string());
            }

            "2" => {
                print!("Enter Task ID to Complete: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();

                let task_id: usize = match id_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid Task ID. Please enter a number.");
                        continue;
                    }
                };

                if let Err(e) = manager.complete_task(task_id) {
                    println!("Error: {}", e);
                }
            }

            "3" => {
                print!("Enter Task ID to Remove: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();

                let task_id: usize = match id_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid Task ID. Please enter a number.");
                        continue;
                    }
                };

                if let Err(e) = manager.remove_task(task_id) {
                    println!("Error: {}", e);
                }
            }

            "4" => {
                manager.list_tasks();
            }

            "5" => {
                println!("Exiting... Bye!");
                break;
            }

            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}
