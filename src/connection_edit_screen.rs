use crate::model::{ApplicationEvent, ApplicationModel};
use crate::ScreenTrait;
use ratatui::Frame;
use std::io::Error;
use crossterm::event;
use crossterm::event::{Event, KeyCode};

pub struct ConnectionEditScreen {}

impl ConnectionEditScreen {
    pub fn new() -> Self {
        ConnectionEditScreen {}
    }
}
impl ScreenTrait for ConnectionEditScreen {
    fn draw(&self, app: &ApplicationModel, f: &mut Frame) {
        todo!()
    }

    fn event_handling(&self) -> Result<Option<ApplicationEvent>, Error> {
        if let Event::Key(key) = event::read()? {
            let msg = match key.code {
                KeyCode::Char('q') => ApplicationEvent::Quit,
                KeyCode::Up | KeyCode::Char('k') => ApplicationEvent::ChangeServerSelectionUp,
                KeyCode::Down | KeyCode::Char('j') => ApplicationEvent::ChangeServerSelectionDown,
                KeyCode::Enter => ApplicationEvent::SwitchToMainScreen,
                _ => ApplicationEvent::None,
            };
            return Ok(Some(msg));
        }
        Ok(None)
    }
}
