use std::fs::{self, File};
use std::io::{Write};
use crate::todo::TodoList;
use std::collections::HashMap;
pub fn save_todos(todos: &TodoList, filename: &str)  -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(todos)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
pub fn load_todos(filename: &str) -> TodoList {
   if let Ok(contents) = fs::read_to_string(filename) {
     serde_json::from_str(&contents).unwrap_or_else(|_| HashMap::new())
    } else {
        HashMap::new()
    }
}