use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

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
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task::new(self.next_id, description);
        self.tasks.push(task);
        self.next_id += 1;
    }

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
}

fn main() {
    let mut manager = TaskManager::new();
    manager.add_task("Learn Rust".to_string());
    manager.add_task("Build a CLI app".to_string());
    manager.list_tasks();
    manager.complete_task(1);
    manager.list_tasks();
}
