mod task;
mod storage;

use clap::{Parser, Subcommand};
use colored::*; 
use task::Task;
use storage::{load_tasks, save_task, update_task, delete_task};

// -> Setting up commands, disabling default help flags.
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

// -> Setting up all commands and their descriptors.
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
        id: i32,
    },
    #[command(about = "mrrow :c")]
    Delete {
        id: i32,
    },
}

// -> Main function to allow for the actual task feedback.
fn main() {
    let cli = Cli::parse();
    let tasks = load_tasks();

    match &cli.command {
        Commands::Add { description } => {
            let task = Task::new(0, description.clone());
            save_task(&task);
            println!("{}", "Task added!".green()); 
        }
        Commands::List => {
            for task in tasks {
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
            let tasks = load_tasks();
            if let Some(task) = tasks.into_iter().find(|t| t.id == *id as usize) {
                let mut task = task;
                task.completed = true;
                update_task(&task);
                println!("{}", "Task marked as done!".green());
            } else {
                println!("{}", "Task not found.".red()); 
            }
        }
        Commands::Delete { id } => {
            delete_task(*id);
            println!("{}", "Task deleted!".red()); 
        }
    }
}
