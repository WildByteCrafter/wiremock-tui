use crate::ScreenTrait;
use crate::main_screen::MainScreen;
use crate::model::{App, Msg};
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use std::error::Error;

pub struct ConnectionScreen {}

impl ScreenTrait for ConnectionScreen {
    fn draw(&self, mut app: &App, f: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
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

        // Server list display
        let items: Vec<ListItem> = app
            .server_list
            .iter()
            .enumerate()
            .map(|(i, &server)| {
                let style = if i == app.current_selected_server_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(server).style(style)
            })
            .collect();

        let server_list =
            List::new(items).block(Block::default().borders(Borders::ALL).title("Servers"));
        f.render_widget(server_list, main_layout[1]);

        // Commands
        let commands = vec!["↑ / k : Up", "↓ / j : Down", "Enter : Confirm"];

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

    fn event_handling(&self) -> Result<Option<Msg>, std::io::Error> {
        if let Event::Key(key) = event::read()? {
            let msg = match key.code {
                KeyCode::Char('q') => Msg::Quit,
                KeyCode::Up | KeyCode::Char('k') => Msg::ChangeServerSelectionUp,
                KeyCode::Down | KeyCode::Char('j') => Msg::ChangeServerSelectionDown,
                KeyCode::Enter => Msg::SwitchToMainScreen,
                _ => Msg::None,
            };
            return Ok(Some(msg));
        }
        Ok(None)
    }
}

impl ConnectionScreen {
    pub fn new() -> Self {
        ConnectionScreen {}
    }
}
