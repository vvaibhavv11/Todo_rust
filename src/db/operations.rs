use rusqlite::{params, Connection, OptionalExtension, Result};

use crate::{
    category::category::Category,
    task::task::TaskList
};

pub struct Operation {
    pub conn: Connection
}

impl Operation {
    pub fn new(con: Connection) -> Self {
        Self { conn: con }
    }

    pub fn create_table(&mut self) -> Result<()> {
        self.conn.execute("CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            category TEXT NOT NULL DEFAULT 'default',
            tasklist TEXT
        )", ())?;
        Ok(())
    }

    pub fn get_first_entry(&mut self) -> Result<Option<(i32, String, String)>> {
        let mut stmt = self.conn.prepare("SELECT * FROM todos ORDER BY id ASC LIMIT 1")?;
        let entry = stmt.query_row([], |row| {
            let id: i32 = row.get(0)?;
            let category: String = row.get(1)?;
            let tasklist: String = row.get(2)?;
            Ok((id, category, tasklist))
        })?;
        Ok(Some(entry))
    }

    pub fn add_category(&mut self, name: String) -> Result<()> {
        self.conn.execute(
            "INSERT INTO todos (category) VALUES (?1)",
            params![name],
        )?;
        Ok(())
    }

    pub fn delete_category(&mut self, name: String) -> Result<()> {
        self.conn.execute(
            "DELETE FROM todos WHERE category = ?1",
            params![name],
        )?;
        Ok(())
    }

    pub fn update_db(&mut self,name: String, task_list: TaskList) -> Result<()> {
        let tasklist = serde_json::to_string(&task_list);
        self.conn.execute(
            "UPDATE todos SET tasklist = ?1 WHERE category = ?2",
            params![tasklist.unwrap(), name],
        )?;
        Ok(())
    }

    pub fn get_all_categorys(&mut self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT DISTINCT category FROM todos")?;
        let category_iter = stmt.query_map([], |row| {
            let category: String = row.get(0)?;
            Ok(category)
        })?;

        let categories: Result<Vec<String>, _> = category_iter.collect();
        categories
    }

    pub fn update_state_variable(&mut self, name: String, category: &mut Category) -> Result<()> {
        let mut stmt = self.conn.prepare("SELECT tasklist FROM todos WHERE category = ?1")?;
        let tasklist: Option<String> = stmt.query_row(params![name], |row| {
            row.get(0)
        }).optional()?;
        let ctasklist = serde_json::from_str(&tasklist.unwrap()).unwrap();
        category.name = name;
        category.task_list = ctasklist;
        Ok(())
    }
}
