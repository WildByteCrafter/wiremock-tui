use crate::ScreenTrait;
use ratatui::Frame;
use crate::application_model::App;

impl MainScreen {
    pub fn new() -> Self {
        MainScreen {}
    }
}

impl ScreenTrait for MainScreen {
    fn draw(&self, app: &App, f: &mut Frame) {
        println!("Main screen");
    }
}

pub struct MainScreen {}