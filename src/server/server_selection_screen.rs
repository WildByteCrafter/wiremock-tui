use crate::model::ScreenTrait;
use crate::model::{ApplicationEvent, ApplicationModel, GlobalEvent};
use crate::server::model::ServerEvent;
use async_trait::async_trait;
use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;
use tokio::sync::mpsc::Sender;

pub struct ServerSelectionScreen {
    sender: Sender<ApplicationEvent>,
}

impl ServerSelectionScreen {
    pub fn new(sender: Sender<ApplicationEvent>) -> Self {
        ServerSelectionScreen { sender }
    }
}

#[async_trait]
impl ScreenTrait for ServerSelectionScreen {
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
        let title = Paragraph::new("Wire Mock - Select Server Connection")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, main_layout[0]);

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
        f.render_widget(server_list, main_layout[1]);

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
            f.render_widget(paragraph, control_layout[index]);
        }
    }

    async fn handle_key_event(&self, event: &Event) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => {
                    self.sender.send(ApplicationEvent::QuitApplication).await?;
                    Ok(())
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.sender
                        .send(ApplicationEvent::Server(ServerEvent::ChangeSelectionUp))
                        .await?;
                    Ok(())
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.sender
                        .send(ApplicationEvent::Server(ServerEvent::ChangeSelectionDown))
                        .await?;
                    Ok(())
                }
                KeyCode::Char('e') => {
                    self.sender
                        .send(ApplicationEvent::Global(
                            GlobalEvent::SwitchToConnectionEditScreen,
                        ))
                        .await?;
                    Ok(())
                }
                KeyCode::Enter => {
                    self.sender
                        .send(ApplicationEvent::Global(GlobalEvent::SwitchToStubScreen))
                        .await?;
                    Ok(())
                }
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }
}
