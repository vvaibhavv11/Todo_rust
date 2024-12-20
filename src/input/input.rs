use ratatui::{
    layout::{Position, Rect},
    style::{Color, Style},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use crate::event_handling::event_handling::{EventHandling, Handling};

pub struct Input {
    pub input: String,
    pub character_index: usize,
    pub messages: String
}


impl Input {
    pub fn view(&mut self, eventhandling: &EventHandling, frame: &mut Frame, chunk: Rect) {
        let new_chunk = Rect::new( chunk.width / 4, chunk.height / 3, chunk.width / 2, chunk.height / 3);
        match eventhandling.handling {
            Handling::HandleAddTask => self.render_input(frame, new_chunk, "Add Task".to_string()),
            Handling::HandleAddCategory => self.render_input(frame, new_chunk, "Add Category".to_string()),
            Handling::HandleDeleteAndChangeCategory => {},
            Handling::HandleTask => {}
        }
    }

    fn render_input(&self, frame: &mut Frame, new_chunk: Rect, title: String) {
        frame.render_widget(Clear, new_chunk);
        frame.render_widget(
            Paragraph::new(self.input.as_str())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::bordered().title(title)),
            new_chunk,
        );
        #[allow(clippy::cast_possible_truncation)]
        frame.set_cursor_position(Position::new(
            new_chunk.x + self.character_index as u16 + 1,
            new_chunk.y + 1,
        ));
    }

    pub fn new() -> Self {
        Self {
            input: String::new(),
            character_index: 0,
            messages: String::new(),
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn reset_message(&mut self) {
        self.messages = "".to_string();
    }

    pub fn submit_message(&mut self) -> String{
        self.messages = self.input.clone();
        self.input.clear();
        self.reset_cursor();
        return self.messages.clone()
    }
}
