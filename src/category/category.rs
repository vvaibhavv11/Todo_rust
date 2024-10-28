use ratatui::{layout::Rect, widgets::Paragraph, Frame};


pub struct Category;

impl Category {
    pub fn view(frame: &mut Frame, chunk: Rect) {
        frame.render_widget(
            Paragraph::new(format!("Category")).centered(),
            chunk
        );
    }
}
