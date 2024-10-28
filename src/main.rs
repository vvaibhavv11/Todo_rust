mod category;
mod task;
mod help;
mod input;
mod event_handling;
use std::time::Duration;
use std::io;
use std::thread::sleep;
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
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let ts = vec![
        Task {
            message: String::from("Task 1"),
            status: Status::Todo,
        },
        Task {
            message: String::from("Task 2"),
            status: Status::Todo,
        },
        Task {
            message: String::from("Task 3"),
            status: Status::Todo,
        },
    ];
    let mut tasks = TaskList { tasks: ts, state: ListState::default() };
    terminal.clear()?;
    let app_result = run(terminal, &mut tasks);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal, tasklist: &mut TaskList) -> io::Result<()> {
    let mut input = Input::new();
    let mut event_handle = EventHandling::new();
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

            Category::view(f, chunks[0]);
            tasklist.view(f, chunks[1]);
            input.view(&event_handle, f, chunks[1]);
            Help::view(&event_handle, f, chunks[2]);
        })?;

        match event_handle.handling {
            // Handling::ViewTask => EventHandling::view_task(&mut input, tasklist),
            Handling::ViewTask => {
                if let Some(result) = event_handle.handle_task(tasklist) {
                    return Ok(result?);
                }
            } 
            Handling::ViewAddTask => event_handle.handle_add_task(&mut input, tasklist),
            Handling::ViewAddCategory => event_handle.handle_add_category(&mut input),
        }

        // match input.is_visible {
        //     Visible::No => {
        //         if let event::Event::Key(key) = event::read()? {
        //             if key.kind == KeyEventKind::Press {
        //                 match key.code {
        //                     KeyCode::Char('q') => return Ok(()),
        //                     KeyCode::Char('a') => input.is_visible = Visible::Yes,
        //                     KeyCode::Char('j') => tasklist.select_next(),
        //                     KeyCode::Char('k') => tasklist.select_previous(),
        //                     KeyCode::Char('g') => tasklist.select_first(),
        //                     KeyCode::Char('G') => tasklist.select_last(),
        //                     _ => {}
        //                 }
        //             }
        //         }
        //     },
        //     Visible::Yes => {
        //         if let event::Event::Key(key) = event::read().unwrap() {
        //             if key.kind == KeyEventKind::Press {
        //                 match key.code {
        //                     KeyCode::Enter => input.submit_message(),
        //                     KeyCode::Char(to_insert) => input.enter_char(to_insert),
        //                     KeyCode::Backspace => input.delete_char(),
        //                     KeyCode::Left => input.move_cursor_left(),
        //                     KeyCode::Right => input.move_cursor_right(),
        //                     KeyCode::Esc => input.is_visible = Visible::No,
        //                     _ => {},
        //                 }
        //             }
        //         } 
        //     }
        // }


        // sleep(Duration::from_millis(16));
    }
}
