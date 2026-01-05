use crossterm::event::KeyEvent;
use crate::contract::contract_processing::ProcessingResult;
use crate::contract::contract_trigger::CommandTriggerPayload;
use crate::cmd_input::cmd_input_module::CmdInputCommands;
use crate::server::server_module::{ServerCommands, ServerEvents};
use crate::stub::stub_module::{StubCommands, StubEvents};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;

pub trait Module {
    fn name(&self) -> &'static str;

    fn can_process_command(&self, command: &Command) -> bool;

    fn process_command(&mut self, command: Command)
    -> Result<ProcessingResult, color_eyre::Report>;

    fn render(&self, frame: &mut Frame, rect: Rect);
}

#[derive(Clone,Debug)]
pub enum Command {
    Application(ApplicationCommands),
    ServerModule(ServerCommands),
    StubModule(StubCommands),
    CmdInputModule(CmdInputCommands),
}

#[derive(Clone,Debug)]
pub enum ApplicationCommands {
    Tick,
    Quit,
    SetCommandTriggers {
        command_trigger_payload: CommandTriggerPayload,
    },
    ProcessInput {
        input: KeyEvent,
    },
}

#[derive(Clone,Debug)]
pub enum Event {
    ServerModule(ServerEvents),
    StubModule(StubEvents),
}

pub trait Task: Send {
    fn execute(&self) -> Result<Command, color_eyre::Report>;
}
