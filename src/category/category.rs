use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, List, ListState, ListDirection, Paragraph},
    Frame
};

use crate::{
    event_handling::event_handling::{EventHandling, Handling},
    task::task::TaskList
};

pub struct AllCategory {
    pub names: Vec<String>,
    pub state: ListState
}

impl AllCategory {
    pub fn view(&mut self, eventhandling: &EventHandling, frame: &mut Frame, chunk: Rect) {
        let new_chunk = Rect::new( chunk.width / 4, chunk.height / 3, chunk.width / 2, chunk.height / 3);
        match eventhandling.handling {
            Handling::HandleAddTask => {},
            Handling::HandleAddCategory => {},
            Handling::HandleDeleteAndChangeCategory => self.render_all_category(frame, new_chunk),
            Handling::HandleTask => {},
        }
    }

    pub fn render_all_category(&mut self, frame: &mut Frame, chunk: Rect) {
        frame.render_stateful_widget(
            List::new(self.names.clone())
                .block(Block::default().title_alignment(Alignment::Center).title("All Category").borders(Borders::all()))
                .highlight_style(Style::new().bg(Color::Cyan).bold())
                .highlight_symbol(">")
                .direction(ListDirection::TopToBottom),
            chunk,
            &mut self.state
        );
    }

    pub fn get_current_name(&mut self) -> String {
        if let Some(i) = self.state.selected() {
            self.names[i].clone()
        } else {
            "".to_string()
        }
    }

    pub fn add_category(&mut self, message: String) {
        if message == "".to_string() {
            return;
        }
        self.names.push(message)
    }

    pub fn delete_category(&mut self) {
        if let Some(i) = self.state.selected() {
            let new_categorys: Vec<String> = self.clone_category()
                .into_iter()
                .filter(|name| *name != self.names[i])
                .collect();
            self.names = new_categorys;
        }
    }

    fn clone_category(&mut self) -> Vec<String> {
        let cself: Vec<String> = self.names
            .iter()
            .map(|t| t.clone())
            .collect();
        cself
    }

    pub fn select_next(&mut self) {
        self.state.select_next()
    }

    pub fn select_previous(&mut self) {
        self.state.select_previous()
    }
}

pub struct Category {
    pub name: String,
    pub task_list: TaskList
}

impl Category {
    pub fn view(&mut self, frame: &mut Frame, chunk: Rect) {
        frame.render_widget(
            Paragraph::new(format!("{}", self.name))
                .block(Block::default().title_alignment(Alignment::Center).title("Category").borders(Borders::all()))
                .centered(),
            chunk
        );
    }
}
