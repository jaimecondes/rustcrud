use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufReader};
use std::path::Path;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: String,
    description: String,
    done: bool,
}

const FILE_NAME: &str = "tasks.json";

fn load_tasks() -> Vec<Task> {
    if !Path::new(FILE_NAME).exists() {
        return Vec::new();
    }

    let file = File::open(FILE_NAME).expect("Unable to open file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
}

fn save_tasks(tasks: &Vec<Task>) {
    let file = File::create(FILE_NAME).expect("Unable to create file");
    serde_json::to_writer_pretty(file, tasks).expect("Unable to write data");
}

fn add_task(description: &str) {
    let mut tasks = load_tasks();
    let new_task = Task {
        id: Uuid::new_v4().to_string(),
        description: description.to_string(),
        done: false,
    };
    tasks.push(new_task);
    save_tasks(&tasks);
    println!("Task added.");
}

fn list_tasks() {
    let tasks = load_tasks();
    for (i, task) in tasks.iter().enumerate() {
        println!(
            "{}. [{}] {}",
            i + 1,
            if task.done { "x" } else { " " },
            task.description
        );
    }
}

fn mark_done(index: usize) {
    let mut tasks = load_tasks();
    if let Some(task) = tasks.get_mut(index) {
        task.done = true;
        save_tasks(&tasks);
        println!("Task marked as done.");
    } else {
        println!("Invalid task number.");
    }
}

fn delete_task(index: usize) {
    let mut tasks = load_tasks();
    if index < tasks.len() {
        tasks.remove(index);
        save_tasks(&tasks);
        println!("Task deleted.");
    } else {
        println!("Invalid task number.");
    }
}

fn print_help() {
    println!("Commands:");
    println!(" add <description>   - Add a new task");
    println!(" list                - List all tasks");
    println!(" done <task_no>      - Mark a task as done");
    println!(" delete <task_no>    - Delete a task");
    println!(" help                - Show this help message");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("add") => {
            if let Some(desc) = args.get(2) {
                add_task(desc);
            } else {
                println!("Description missing.");
            }
        }
        Some("list") => list_tasks(),
        Some("done") => {
            if let Some(index) = args.get(2).and_then(|s| s.parse::<usize>().ok()) {
                mark_done(index - 1);
            } else {
                println!("Please provide a valid task number.");
            }
        }
        Some("delete") => {
            if let Some(index) = args.get(2).and_then(|s| s.parse::<usize>().ok()) {
                delete_task(index - 1);
            } else {
                println!("Please provide a valid task number.");
            }
        }
        Some("help") | _ => print_help(),
    }
}
