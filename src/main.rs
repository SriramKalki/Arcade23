use std::env;
use std::fs::{OpenOptions};
use std::io::{BufRead, BufReader, Write};

#[derive(Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: u32, description: String) -> Task {
        Task {
            id,
            description,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    //create task manager
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    //add task
    fn add_task(&mut self, description: String) {
        let task = Task::new(self.next_id, description);
        self.tasks.push(task);
        self.next_id += 1;
    }

    //list task
    fn list_tasks(&self) {
        for task in &self.tasks {
            println!("{:?}", task);
        }
    }

    fn complete_task(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.complete();
        } else {
            println!("Task with id {} not found", id);
        }
    }

    fn load_from_file(filename: &str) -> TaskManager {
        let file = OpenOptions::new().read(true).open(filename);
        let mut manager = TaskManager::new();

        if let Ok(file) = file {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let parts: Vec<&str> = line.split(",").collect();
                    if parts.len() == 3 {
                        let id = parts[0].parse::<u32>().unwrap_or(0);
                        let description = parts[1].to_string();
                        let completed = parts[2].parse::<bool>().unwrap_or(false);
                        let task = Task { id, description, completed };
                        manager.tasks.push(task);
                        if id >= manager.next_id {
                            manager.next_id = id + 1;
                        }
                    }
                }
            }
        }

        manager
    }

    fn save_to_file(&self, filename: &str) {
        let file = OpenOptions::new().write(true).truncate(true).create(true).open(filename);
        
        if let Ok(mut file) = file {
            for task in &self.tasks {
                let line = format!("{},{},{}\n", task.id, task.description, task.completed);
                file.write_all(line.as_bytes()).unwrap();
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut manager = TaskManager::load_from_file("tasks.txt");

    if args.len() > 1 {
        match args[1].as_str() {
            "add" => {
                if args.len() > 2 {
                    let description = args[2..].join(" ");
                    manager.add_task(description);
                }
            }
            "list" => {
                manager.list_tasks();
            }
            "complete" => {
                if args.len() > 2 {
                    if let Ok(id) = args[2].parse::<u32>() {
                        manager.complete_task(id);
                    }
                }
            }
            _ => {
                println!("Unknown command");
            }
        }
    } else {
        println!("Usage: todo_list <command> [arguments]");
        println!("Commands:");
        println!("  add <task description>");
        println!("  list");
        println!("  complete <task id>");
    }

    manager.save_to_file("tasks.txt");
}
