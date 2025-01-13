use rusqlite::{params, Connection};
use dirs_next::data_local_dir;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}

// Get the database path.
fn get_database_path() -> PathBuf {
    let mut dir = data_local_dir().expect("Can't determine data directory :c");
    dir.push("meowlist");
    dir.push("tasks.db");
    if let Some(parent) = dir.parent() {
        std::fs::create_dir_all(parent).expect("Can't create data directory :c");
    }
    dir
}

// Get a connection to the database and ensure the tasks table exists.
fn get_connection() -> Connection {
    let db_path = get_database_path();
    let conn = Connection::open(db_path).expect("Can't open database :c");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            completed INTEGER NOT NULL
        )",
        [],
    )
    .expect("Can't create tasks table :c");
    conn
}

// Load tasks from the database.
pub fn load_tasks() -> Vec<Task> {
    let conn = get_connection();
    let mut stmt = conn.prepare("SELECT id, description, completed FROM tasks").unwrap();
    let task_iter = stmt
        .query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                completed: row.get::<_, i32>(2)? != 0,
            })
        })
        .unwrap();
    task_iter.filter_map(Result::ok).collect()
}

// Save a task to the database.
pub fn save_task(task: &Task) -> i32 {
    let conn = get_connection();
    let next_id: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(id), 0) + 1 FROM tasks",
            [],
            |row| row.get(0),
        )
        .unwrap_or(1);

    conn.execute(
        "INSERT INTO tasks (id, description, completed) VALUES (?1, ?2, ?3)",
        params![next_id, task.description, task.completed as i32],
    )
    .expect("Can't save task :c");

    next_id
}

// Delete a task from the database.
pub fn delete_task(task_id: i32) {
    let conn = get_connection();
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![task_id])
        .expect("Can't delete task :c");
}