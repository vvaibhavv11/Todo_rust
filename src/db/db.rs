use rusqlite::{Connection, Result, Error};
use std::{fs, path::PathBuf};

pub fn connection() -> Result<Connection, Error> {
    let mut storage = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    storage.push(".local/share/todo_rust/");
    fs::create_dir_all(&storage).expect("Failed to create directories");
    storage.push("todos.db3");
    Connection::open(storage)
}
