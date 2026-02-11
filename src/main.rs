use clap::{Parser, Subcommand};
mod todo;
mod storage;
//use todo::{TodoList, Todo};
use storage::{save_todos};
//use storage
#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple command-line todo application")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Add a new todo item
    Add {
        /// The description of the todo item
        description: String},
    /// List all todo items
    List,
    /// Mark a todo item as completed
    Done {
        /// The ID of the todo item to mark as completed
        id: usize},
    Remove {
        /// The ID of the todo item to remove
        id: usize},
}
fn main() {
    let cli = Cli::parse();
    let filename = "todos.json";
    let mut todos = storage::load_todos(filename);
    match cli.command {
        Commands::Add { description } => {
            let id = todos.keys().max().unwrap_or(&0) + 1;
            todos.insert(id, todo::Todo::new(description));
            println!("Added todo {}.", id);}
    
    Commands::List => {
        todo::list_todos(&todos);
        }
    Commands::Done { id } => {
        todo::mark_done(&mut todos, id);
        }
    Commands::Remove { id } => {
        todo::remove_todo(&mut todos, id);
        }
    }
    save_todos(&todos, filename).expect("Failed to save todos.");
}
