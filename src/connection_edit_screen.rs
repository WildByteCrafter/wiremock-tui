use crate::model::{ApplicationEvent, ApplicationModel, GlobalEvent};
use crate::ScreenTrait;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use std::io::Error;

pub struct ConnectionEditScreen {}

impl ConnectionEditScreen {
    pub fn new() -> Self {
        ConnectionEditScreen {}
    }
}
impl ScreenTrait for ConnectionEditScreen {
    fn draw(&self, app: &ApplicationModel, f: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.area());

        // Title
        let title = Paragraph::new("Wire Mock Inspector  - Connection")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, main_layout[0]);

        // Commands
        let commands = vec!["q : Quit", "Enter : Confirm"];

        let control_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(main_layout[2]);

        for (index, command) in commands.iter().enumerate() {
            let paragraph = Paragraph::new(*command);
            f.render_widget(paragraph, control_layout[index]);
        }
    }

    fn event_handling(&self) -> Result<Option<ApplicationEvent>, Error> {
        if let Event::Key(key) = event::read()? {
            let msg = match key.code {
                KeyCode::Char('q') => ApplicationEvent::Global(GlobalEvent::Quit),
                KeyCode::Enter => ApplicationEvent::Global(GlobalEvent::SwitchToStubScreen),
                _ => ApplicationEvent::None,
            };
            return Ok(Some(msg));
        }
        Ok(None)
    }
}
