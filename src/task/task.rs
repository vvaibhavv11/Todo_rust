use serde::{Serialize, Deserialize};
use ratatui::{
    layout::{Alignment, Rect},
    text::Line,
    style::{palette::tailwind::{SLATE, GREEN}, Color, Style, Stylize},
    widgets::{Block, Borders, List, ListDirection, ListItem, ListState},
    Frame,
};

const TODO_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TODO_FG_COLOR: Color = GREEN.c500;

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Status {
    Todo,
    Completed
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub message: String,
    pub status: Status
}

#[derive(Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
    #[serde(skip)]
    pub state: ListState
}

impl Task {
    fn new_task(todo: String) -> Self {
        Self {
            message: todo,
            status: Status::Todo
        }
    }

    fn new_completed_task(todo: String) -> Self {
        Self {
            message: todo,
            status: Status::Completed
        }
    }
}

impl TaskList {
    pub fn view(&mut self, frame: &mut Frame, chunk: Rect) {
        self.format_tasks();
        let task: Vec<ListItem> = self.tasks.iter().map(|t| {
            let line = match t.status {
                Status::Todo => Line::styled(format!(" ☐ {}", t.message.clone()), TODO_FG_COLOR),
                Status::Completed =>  Line::styled(format!(" ✓ {}", t.message.clone()), COMPLETED_TODO_FG_COLOR)
            };

            ListItem::new(line)
        }).collect();
        frame.render_stateful_widget(
            List::new(task)
                .block(Block::default().title_alignment(Alignment::Center).title("Todos").borders(Borders::all()))
                .highlight_style(Style::new().bg(Color::Cyan).bold())
                .highlight_symbol(">")
                .direction(ListDirection::TopToBottom),
            chunk,
            &mut self.state
        );
    }

    pub fn toggle_status(&mut self) {
        if let Some(i) = self.state.selected() {
            self.tasks[i].status = match self.tasks[i].status {
                Status::Completed => Status::Todo,
                Status::Todo => Status::Completed
            }
        }
    }

    fn format_tasks(&mut self) {
        let mut non_completed_task: Vec<Task> = self.tasks
            .iter()
            .filter(|t| t.status == Status::Todo)
            .map(|t| Task::new_task(t.message.clone()))
            .collect();

        let completed_task: Vec<Task> = self.tasks
            .iter()
            .filter(|t| t.status == Status::Completed)
            .map(|t| Task::new_completed_task(t.message.clone()))
            .collect();

        completed_task.into_iter().for_each(|v| non_completed_task.push(v));
        self.tasks = non_completed_task;
    }

    pub fn delete_task(&mut self) {
        if let Some(i) = self.state.selected() {
            let new_tasks: Vec<Task> = self.clone_tasks()
                .into_iter()
                .filter(|t| t.message != self.tasks[i].message)
                .collect();
            self.tasks = new_tasks;
        }
    }

    pub fn clone_tasks(&mut self) -> Vec<Task> {
        let cself: Vec<Task> = self.tasks.iter().map(|t| {
            match t.status {
                Status::Todo => Task::new_task(t.message.clone()),
                Status::Completed => Task::new_completed_task(t.message.clone())
            }
        }).collect();
        cself
    }

    pub fn select_next(&mut self) {
        self.state.select_next()
    }

    pub fn select_previous(&mut self) {
        self.state.select_previous()
    }

    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.state.select_last();
    } 

    pub fn update(&mut self, message: String) {
        if message == "".to_string() {
            return;
        }
        let new_task = Task::new_task(message);
        self.tasks.push(new_task)
    }

}
