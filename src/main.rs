mod category;
mod task;
mod help;
mod input;
mod event_handling;
mod db;
use std::time::Duration;
use std::io;
use std::thread::sleep;
use category::category::AllCategory;
use ratatui::prelude::*;
use ratatui::widgets::ListState;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    DefaultTerminal,
};

use crate::{
    category::category::Category,
    task::task::{TaskList, Task, Status}, help::help::Help,
    input::input::Input,
    event_handling::event_handling::{EventHandling, Handling},
    db::db::connection,
    db::operations::Operation
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let conn = connection();
    let mut db = Operation::new(conn.unwrap());
    let _ = db.create_table();
    let data = db.get_first_entry().unwrap().unwrap();
    let mut category = Category {
        name: data.1,
        task_list: TaskList {
            tasks: serde_json::from_str(&data.2).unwrap_or(vec![]),
            state: ListState::default()
        }
    };
    terminal.clear()?;
    let app_result = run(terminal, &mut db, &mut category);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal, oper: &mut Operation, category: &mut Category) -> io::Result<()> {
    let mut input = Input::new();
    let mut event_handle = EventHandling::new();
    let mut all_categorys = AllCategory {
        names: oper.get_all_categorys().unwrap(),
        state: ListState::default()
    };
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .split(f.area());

            category.view(f, chunks[0]);
            all_categorys.view(&event_handle, f, chunks[1]);
            category.task_list.view(f, chunks[1]);
            input.view(&event_handle, f, chunks[1]);
            Help::view(&event_handle, f, chunks[2]);
        })?;

        match event_handle.handling {
            Handling::HandleTask => {
                if let Some(result) = event_handle.handle_task(category.name.clone(), oper, &mut category.task_list) {
                    return Ok(result?);
                }
            } 
            Handling::HandleAddTask => event_handle.handle_add_task(&mut input, &mut category.task_list),
            Handling::HandleAddCategory => event_handle.handle_add_category(&mut input, &mut all_categorys, oper),
            Handling::HandleDeleteAndChangeCategory => event_handle.handle_delete_change_category(category, &mut all_categorys, oper),
        }
        // sleep(Duration::from_millis(16));
    }
}
