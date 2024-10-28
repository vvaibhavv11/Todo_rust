use crossterm::event::{self, KeyCode, KeyEventKind};

use crate::{input::input::Input, task::task::TaskList};

pub struct EventHandling {
    pub handling: Handling,
}

pub enum Handling {
    ViewTask,
    ViewAddTask,
    ViewAddCategory,
}

impl EventHandling {
    pub fn new() -> Self {
        Self {
            handling: Handling::ViewTask,
        }
    }

    pub fn handle_task(&mut self, tasklist: &mut TaskList) -> Option<Result<(), std::io::Error>> {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Some(Ok(())),
                    KeyCode::Char('a') => {self.handling = Handling::ViewAddTask; None},
                    KeyCode::Char('A') => {self.handling = Handling::ViewAddCategory; None},
                    KeyCode::Char('j') => {tasklist.select_next(); None},
                    KeyCode::Char('l') => {tasklist.toggle_status(); None},
                    KeyCode::Char('k') => {tasklist.select_previous(); None},
                    KeyCode::Char('d') => {tasklist.delete_task(); None},
                    KeyCode::Char('g') => {tasklist.select_first(); None},
                    KeyCode::Char('G') => {tasklist.select_last(); None},
                    _ => None,
                }
            } else {None}
        } else {None}
    }
    
    pub fn handle_add_task(&mut self, input: &mut Input, tasklist: &mut TaskList) {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => {
                        let new_task = input.submit_message();
                        tasklist.update(new_task);
                        input.reset_message();
                        self.handling = Handling::ViewTask;
                    },
                    KeyCode::Char(to_insert) => input.enter_char(to_insert),
                    KeyCode::Backspace => input.delete_char(),
                    KeyCode::Left => input.move_cursor_left(),
                    KeyCode::Right => input.move_cursor_right(),
                    KeyCode::Esc => self.handling = Handling::ViewTask,
                    _ => {},
                }
            }
        } 
    }

    pub fn handle_add_category(&mut self, input: &mut Input) {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => {
                        input.reset_message();
                        self.handling = Handling::ViewTask;
                    },
                    KeyCode::Char(to_insert) => input.enter_char(to_insert),
                    KeyCode::Backspace => input.delete_char(),
                    KeyCode::Left => input.move_cursor_left(),
                    KeyCode::Right => input.move_cursor_right(),
                    KeyCode::Esc => self.handling = Handling::ViewTask,
                    _ => {},
                }
            }
        } 
    }
}
