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
}

fn main() {
    let mut manager = TaskManager::new();
    manager.add_task("Learn Rust".to_string());
    manager.add_task("Touch Grass".to_string());
    manager.list_tasks();
    manager.complete_task(1);
    manager.list_tasks();
}
