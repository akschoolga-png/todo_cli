use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    description: String,
    completed: bool,
}
pub type TodoList = HashMap<usize, Todo>;
    impl Todo {
        pub fn new(description: String) -> Self {
            Todo {
                description,
                completed: false,
            }
        }
}
pub fn list_todos(todos: &TodoList) {
    if todos.is_empty() {
        println!("No todo items found.");
        return;}
        
    let mut todol: Vec<_> = todos.keys().collect();
    todol.sort();
    for id in todol {
        let todo = &todos[id];
        let status = if todo.completed { "✓" } else { " " };
        println!("{} [{}] {}", id, status, todo.description);
    }
}
pub fn mark_done(todos: &mut TodoList, id: usize) {
    if let Some(todo) = todos.get_mut(&id) {
        todo.completed = true;
        println!("Marked todo {} as completed.", id);
    } else {
        println!("Todo with ID {} not found.", id);
    }
}
pub fn remove_todo(todos: &mut TodoList, id: usize) {
    if todos.remove(&id).is_some() {
        println!("Removed todo {}.", id);
    } else {
        println!("Todo with ID {} not found.", id);
    }
}