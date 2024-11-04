use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::widgets::ListState;

use crate::{
    category::category::{Category, AllCategory},
    db::operations::Operation,
    input::input::Input,
    task::task::TaskList
};

pub struct EventHandling {
    pub handling: Handling
}

pub enum Handling {
    HandleTask,
    HandleAddTask,
    HandleAddCategory,
    HandleDeleteAndChangeCategory
}

impl EventHandling {
    pub fn new() -> Self {
        Self {
            handling: Handling::HandleTask
        }
    }

    pub fn handle_task(&mut self, name: String, oper: &mut Operation, tasklist: &mut TaskList) -> Option<Result<(), std::io::Error>> {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => {
                        let _ = oper.update_db(name.clone(), TaskList {
                            tasks: tasklist.clone_tasks(),
                            state: ListState::default()
                        });
                        Some(Ok(()))
                    },
                    KeyCode::Char('a') => {self.handling = Handling::HandleAddTask; None},
                    KeyCode::Char('A') => {self.handling = Handling::HandleAddCategory; None},
                    KeyCode::Char('c') => {self.handling = Handling::HandleDeleteAndChangeCategory; None},
                    KeyCode::Char('j') => {tasklist.select_next(); None},
                    KeyCode::Char('l') => {tasklist.toggle_status(); None},
                    KeyCode::Char('k') => {tasklist.select_previous(); None},
                    KeyCode::Char('d') => {tasklist.delete_task(); None},
                    KeyCode::Char('g') => {tasklist.select_first(); None},
                    KeyCode::Char('G') => {tasklist.select_last(); None},
                    _ => None
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
                        self.handling = Handling::HandleTask;
                    },
                    KeyCode::Char(to_insert) => input.enter_char(to_insert),
                    KeyCode::Backspace => input.delete_char(),
                    KeyCode::Left => input.move_cursor_left(),
                    KeyCode::Right => input.move_cursor_right(),
                    KeyCode::Esc => self.handling = Handling::HandleTask,
                    _ => {}
                }
            }
        } 
    }

    pub fn handle_add_category(&mut self, input: &mut Input, all_category: &mut AllCategory, oper: &mut Operation) {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => {
                        let name = input.submit_message();
                        all_category.add_category(name.clone());
                        let _ = oper.add_category(name);
                        self.handling = Handling::HandleTask;
                    },
                    KeyCode::Char(to_insert) => input.enter_char(to_insert),
                    KeyCode::Backspace => input.delete_char(),
                    KeyCode::Left => input.move_cursor_left(),
                    KeyCode::Right => input.move_cursor_right(),
                    KeyCode::Esc => self.handling = Handling::HandleTask,
                    _ => {}
                }
            }
        } 
    }

    pub fn handle_delete_change_category(&mut self, category: &mut Category, all_category: &mut AllCategory, oper: &mut Operation) {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => self.handling = Handling::HandleTask,
                    KeyCode::Enter => {
                        let _ = oper.update_db(category.name.clone(), TaskList {
                            tasks: category.task_list.clone_tasks(),
                            state: ListState::default()
                        });
                        let _ = oper.update_state_variable(all_category.get_current_name(), category);
                        self.handling = Handling::HandleTask;
                    },
                    KeyCode::Char('j') => all_category.select_next(),
                    KeyCode::Char('k') => all_category.select_previous(),
                    KeyCode::Char('d') => {
                        let _ = oper.delete_category(all_category.get_current_name());
                        all_category.delete_category();
                    },
                    _ => {}
                }
            }
        }
    }
}
