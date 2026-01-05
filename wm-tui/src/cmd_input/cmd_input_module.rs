use crate::contract::contract_module::{Command, Module};
use crate::contract::contract_processing::ProcessingResult::NothingDone;
use crate::contract::contract_processing::{ProcessingResult, ProcessingResultPayload};
use crate::contract::contract_trigger::CommandTriggerPayload;
use color_eyre::Report;
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;

#[derive(Clone, Debug)]
pub enum CmdInputCommands {
    RegisterCommandTriggers {
        command_trigger_payload: CommandTriggerPayload,
    },
    ProcessInput {
        input: KeyEvent,
    },
}

#[derive(Clone, Debug)]
pub enum CmdInputEvents {}

#[derive(Clone, Debug)]
pub struct CmdInputModule {}

impl CmdInputModule {
    pub fn new() -> Self {
        Self {}
    }
}

const MODULE_NAME: &'static str = "cmd_input";

impl Module for CmdInputModule {
    fn name(&self) -> &'static str {
        MODULE_NAME
    }

    fn can_process_command(&self, command: &Command) -> bool {
        match command {
            Command::CmdInputModule(_) => true,
            _ => false,
        }
    }

    fn process_command(&mut self, command: Command) -> Result<ProcessingResult, Report> {
        if !self.can_process_command(&command) {
            return Ok(NothingDone);
        }
        match command {
            Command::CmdInputModule(CmdInputCommands::RegisterCommandTriggers {
                command_trigger_payload,
            }) => {
                println!(
                    "Registering command triggers: {:?}",
                    command_trigger_payload
                );
                Ok(ProcessingResult::Processed(ProcessingResultPayload::new()))
            }
            Command::CmdInputModule(CmdInputCommands::ProcessInput { input }) => {
                println!("Input: {:?}", input);
                Ok(ProcessingResult::Processed(ProcessingResultPayload::new()))
            }
            _ => Ok(NothingDone),
        }
    }

    fn render(&self, frame: &mut Frame, rect: Rect) {
        todo!()
    }
}
