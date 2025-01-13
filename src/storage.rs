use rusqlite::{params, Connection, Result};
use std::path::PathBuf;
use dirs_next::data_local_dir;
use crate::task::Task;

fn get_database_path() -> PathBuf {
    let mut dir = data_local_dir().expect("Can't determine data directory :c");

    dir.push("meowlist");
    dir.push("tasks.db");

    if let Some(parent) = dir.parent() {
        std::fs::create_dir_all(parent).expect("Can't create data directory :c");
    }

    dir
}

fn get_connection() -> Connection {
    let db_path = get_database_path();
    let conn = Connection::open(db_path).expect("Can't open database :c");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            completed INTEGER NOT NULL
        [],
    ).expect("Can't create tasks table :c");

    conn
}

pub fn load_tasks() -> Vec<Task> {
    let conn = get_connection();
    let mut stmt = conn.prepare("SELECT id, description, completed FROM tasks").unwrap();

    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            completed: row.get::<_, i32>(2)? != 0,
        })
    }).unwrap();

    task_iter.filter_map(Result::ok).collect()
}

pub fn save_task(task: &Task) {
    let conn = get_connection();

    let next_id: i32 = conn.query_row(
        "SELECT COALESCE(MIN(id) + 1, 1)
         FROM tasks
         WHERE (MIN(id) + 1) NOT IN (SELECT id FROM tasks)",
        [],
        |row| row.get(0),
    ).unwrap_or_else(|_| {
        conn.query_row(
            "SELECT COALESCE(MAX(id), 0) + 1 FROM tasks",
            [],
            |row| row.get(0),
        ).unwrap_or(1)
    });

    conn.execute(
        "INSERT INTO tasks (description, completed) VALUES (?1, ?2)",
        params![task.description, task.completed as i32],
    ).expect("Can't save task :c");
}

pub fn update_task(task: &Task) {
    let conn = get_connection();
    conn.execute(
        "UPDATE tasks SET description = ?1, completed = ?2 WHERE id = ?3",
        params![task.description, task.completed as i32, task.id],
    ).expect("Can't update task :c");
}

pub fn delete_task(task_id: i32) {
    let conn = get_connection();
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![task_id])
        .expect("Can't delete task :c");

    let mut stmt = conn.prepare("SELECT id FROM tasks ORDER BY id").unwrap();
    let task_ids: Vec<i32> = stmt.query_map([], |row| row.get(0)).unwrap()
        .filter_map(Result::ok)
        .collect();

    for (new_id, old_id) in task_ids.iter().enumerate() {
        conn.execute(
            "UPDATE tasks SET id = ?1 WHERE id = ?2",
            params![(new_id + 1) as i32, old_id],
        ).expect("Can't renumber task IDs :c");
    }
}
