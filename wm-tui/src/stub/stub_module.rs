use crate::contract::contract_module::{Command, Module};
use crate::contract::contract_processing::ProcessingResult;
use color_eyre::Report;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;

#[derive(Debug, Clone)]
pub struct StubCommands {}

#[derive(Debug, Clone)]
pub struct StubEvents {}

#[derive(Debug, Clone)]
pub struct StubModule {}

impl StubModule {
    pub fn new() -> Self {
        StubModule {}
    }
}

const MODULE_NAME: &'static str = "stub";

impl Module for StubModule {
    fn name(&self) -> &'static str {
        MODULE_NAME
    }

    fn can_process_command(&self, command: &Command) -> bool {
        match command {
            Command::StubModule(_) => true,
            _ => false,
        }
    }

    fn process_command(&mut self, _: Command) -> Result<ProcessingResult, Report> {
        todo!()
    }

    fn render(&self, frame: &mut Frame, rect: Rect) {
        todo!()
    }
}
