use ratatui::{layout::{Rect, Alignment}, widgets::{Block, Borders, Paragraph}, Frame};

use crate::event_handling::event_handling::{EventHandling, Handling};

const TASK: &'static str = "q: Quit | a/A: Add Task/Add Category | j/k: Down/Up | g/G: to go Top/Bottom | d/l: Delete Task/Change Status";
const ADDTASK: &'static str = "esc: Close | enter: Submit the task";
const ADDCATEGORY: &'static str = "esc: Close | enter: Submit the Category";

pub struct Help;

impl Help {
    pub fn view(eventhandling: &EventHandling, frame: &mut Frame, chunk: Rect) {
        let newchunk = Rect::new(chunk.x, chunk.y, chunk.width, chunk.height);
        match eventhandling.handling {
            Handling::ViewTask => Help::render_help(frame, newchunk, TASK),
            Handling::ViewAddTask => Help::render_help(frame, newchunk, ADDTASK),
            Handling::ViewAddCategory => Help::render_help(frame, newchunk, ADDCATEGORY)
        }
    }

    fn render_help(frame: &mut Frame, chunk: Rect, help: &str) {
        frame.render_widget(
            Paragraph::new(format!("{}", help))
                .block(Block::default().title_alignment(Alignment::Center).title("Commands").borders(Borders::all()))
                .centered(),
            chunk
        )
    }

    // fn for_add_task(frame: &mut Frame, chunk: Rect) {
    //     frame.render_widget(
    //         Paragraph::new(format!("esc: Close | enter: Submit the task"))
    //             .block(Block::default().title_alignment(Alignment::Center).title("Commands").borders(Borders::all()))
    //             .centered(),
    //         chunk
    //     )
    // }
}
