use crate::ScreenTrait;
use crate::model::{App, Msg};
use ratatui::Frame;
use std::error::Error;

pub struct MainScreen {}

impl MainScreen {
    pub fn new() -> Self {
        MainScreen {}
    }
}

impl ScreenTrait for MainScreen {
    fn draw(&self, app: &App, f: &mut Frame) {}

    fn event_handling(&self) -> Result<Option<Msg>, std::io::Error> {
        Ok(None)
    }
}
