use crate::model::ScreenTrait;
use crate::model::{ApplicationEvent, ApplicationModel, GlobalEvent};
use crate::ui;
use async_trait::async_trait;
use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use tokio::sync::mpsc::Sender;

pub struct ServerEditScreen {
    sender: Sender<ApplicationEvent>,
}

impl ServerEditScreen {
    pub fn new(sender: Sender<ApplicationEvent>) -> Self {
        ServerEditScreen { sender }
    }
}

#[async_trait]
impl ScreenTrait for ServerEditScreen {
    fn draw(&self, _: &ApplicationModel, frame: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(frame.area());

        // Title
        let title = ui::widgets::title_paragraph("Wire Mock  - Edit Server Connection");
        frame.render_widget(title, main_layout[0]);

        // Commands
        let commands = vec!["q : Quit", "Enter : Confirm"];

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
                    self.sender.send(ApplicationEvent::QuitApplication).await?;
                    Ok(())
                }
                KeyCode::Enter | KeyCode::Char('k') => {
                    self.sender
                        .send(ApplicationEvent::Global(
                            GlobalEvent::SwitchToServerSelectionScreen,
                        ))
                        .await?;
                    Ok(())
                }
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }
}
