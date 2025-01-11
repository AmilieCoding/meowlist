use std::fs::{OpenOptions};
use std::io::{Read, Write};
use crate::task::Task;

const FILE_PATH: &str = "tasks.json";

pub fn load_tasks() -> Vec<Task> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE_PATH)
        .unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

pub fn save_tasks(tasks: &[Task]) {
    let data = serde_json::to_string(tasks).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .unwrap();
    file.write_all(data.as_bytes()).unwrap();
}
