use crate::model::ScreenTrait;
use crate::model::{ApplicationModel, GlobalMsg, Message};
use crate::server::model::ServerMsg;
use crate::ui;
use async_trait::async_trait;
use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;
use tokio::sync::broadcast::Sender;

pub struct ServerSelectionScreen {
    sender: Sender<Message>,
}

impl ServerSelectionScreen {
    pub fn new(sender: Sender<Message>) -> Self {
        ServerSelectionScreen { sender }
    }
}

#[async_trait]
impl ScreenTrait for ServerSelectionScreen {
    fn draw(&self, app: &ApplicationModel, frame: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(frame.area());

        // Title
        let title = ui::widgets::title_paragraph("Wire Mock - Select Server Connection");
        frame.render_widget(title, main_layout[0]);

        // Server list display
        let items: Vec<ListItem> = app
            .server_model
            .server_list
            .iter()
            .enumerate()
            .map(|(i, server)| {
                let style = if i
                    == app
                        .server_model
                        .current_selected_server_index
                        .unwrap_or(999)
                {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(format!("▶ {}", server)).style(style)
            })
            .collect();

        let server_list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Server selection"),
        );
        frame.render_widget(server_list, main_layout[1]);

        // Commands
        let commands = vec![
            "↑ / k : Up",
            "↓ / j : Down",
            "e : Edit server connection",
            "Enter : Confirm",
        ];

        let control_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Ratio(1, commands.len() as u32);
                commands.len()
            ])
            .split(main_layout[2]);

        for (index, command) in commands.iter().enumerate() {
            let paragraph = Paragraph::new(*command);
            frame.render_widget(paragraph, control_layout[index]);
        }
    }

    async fn handle_key_event(&self, event: &Event) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => {
                    self.sender.send(Message::QuitRequested)?;
                    Ok(())
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.sender
                        .send(Message::Server(ServerMsg::ChangeSelectionUp))?;
                    Ok(())
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.sender
                        .send(Message::Server(ServerMsg::ChangeSelectionDown))?;
                    Ok(())
                }
                KeyCode::Char('e') => {
                    self.sender
                        .send(Message::Global(GlobalMsg::SwitchToConnectionEditScreen))?;
                    Ok(())
                }
                KeyCode::Enter => {
                    self.sender
                        .send(Message::Global(GlobalMsg::SwitchToStubScreen))?;
                    Ok(())
                }
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }
}
