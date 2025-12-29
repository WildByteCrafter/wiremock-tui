use crate::contract::{Command, Module, ProcessingResult, ProcessingResultPayload, Task};
use color_eyre::Report;
use ratatui::prelude::Widget;
use ProcessingResult::NothingDone;

#[derive(Clone)]
pub enum ServerEvents {
    ServerSelected { server: String },
}

#[derive(Clone)]
pub enum ServerCommands {
    ShowServerSelection,
    ImportLoadedServerList { server_list: Vec<String> },
    ServerSelectionUp,
    ServerSelectionDown,
    SelectServer,
}

pub struct ServerModule {
    server_list: Vec<String>,
}

enum ModuleState {
    NotInitializes,
    LoadConfiguration,
    Initializes,
    ShowServerSelection,
}

impl ServerModule {
    pub fn new() -> Self {
        Self {
            server_list: Vec::new(),
        }
    }
}

impl Module for ServerModule {
    fn process_command(&mut self, command: &Command) -> Result<ProcessingResult, Report> {
        let server_command = match command {
            Command::ServerModule(server_commands) => server_commands,
            _ => return Ok(NothingDone),
        };
        match server_command {
            ServerCommands::ShowServerSelection => {
                let task = Box::new(LoadServerTask::new());
                Ok(ProcessingResult::Processed(ProcessingResultPayload::new().with_tasks(task)))
            }
            ServerCommands::ImportLoadedServerList { server_list } => {
                self.server_list.extend(server_list.clone());
                Ok(ProcessingResult::Processed(ProcessingResultPayload::new()))
            }
            _ => Ok(NothingDone),
        }
    }

    fn main_widget(&self) -> Option<Box<dyn Widget>> {
        todo!()
    }
}

struct LoadServerTask;

impl LoadServerTask {
    pub fn new() -> Self {
        Self {}
    }
}

impl Task for LoadServerTask {
    fn execute(&self) -> Result<Command, Report> {
        Ok(Command::ServerModule(
            ServerCommands::ImportLoadedServerList {
                server_list: vec!["http://localhost:8080".to_string()],
            },
        ))
    }
}
