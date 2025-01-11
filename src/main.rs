mod task;
mod storage;

use clap::{Parser, Subcommand};
use colored::*; 
use task::Task;
use storage::{load_tasks, save_tasks};

#[derive(Parser)]
#[command(
    name = "meowlist",
    about = "Meowlist - Manage your tasks, in a silly way :p",
    long_about = None,
    disable_help_flag = true, 
    disable_help_subcommand = true, 
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "mrrp mrrow mreow :)")]
    Add {
        description: String,
    },
    #[command(about = "mrrow mrrp ?")]
    List,
    #[command(about = "meow mrrp mrowww :D")]
    Done {
        id: usize,
    },
    #[command(about = "mrrow :c")]
    Delete {
        id: usize,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match &cli.command {
        Commands::Add { description } => {
            let id = tasks.len() + 1;
            let task = Task::new(id, description.clone());
            tasks.push(task);
            save_tasks(&tasks);
            println!("{}", "Task added!".green()); 
        }
        Commands::List => {
            for task in &tasks {
                let status = if task.completed { "[x]" } else { "[ ]" };
                let color = if task.completed { "green" } else { "red" };
                println!(
                    "{}. {} {}",
                    task.id,
                    status.color(color),
                    task.description
                );
            }
        }
        Commands::Done { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == *id) {
                task.completed = true;
                save_tasks(&tasks);
                println!("{}", "Task marked as done!".green());
            } else {
                println!("{}", "Task not found.".red()); 
            }
        }
        Commands::Delete { id } => {
            tasks.retain(|task| task.id != *id);
            save_tasks(&tasks);
            println!("{}", "Task deleted!".red()); 
        }
    }
}
