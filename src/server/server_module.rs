use crate::contract::{Command, Event, Module, ProcessingResult, ProcessingResultPayload, Task};
use color_eyre::Report;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ProcessingResult::NothingDone;

#[derive(Clone)]
pub enum ServerEvents {
    ServerSelectionReadyForDisplay,
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
    fn name(&self) -> &'static str {
        "server"
    }

    fn can_process_command(&self, command: &Command) -> bool {
        match command {
            Command::ServerModule(_) => true,
            _ => false,
        }
    }

    fn process_command(&mut self, command: Command) -> Result<ProcessingResult, Report> {
        if !self.can_process_command(&command) {
            return Ok(NothingDone);
        }
        match command {
            Command::ServerModule(ServerCommands::ShowServerSelection) => {
                let task = Box::new(LoadServerTask::new());
                Ok(ProcessingResult::Processed(
                    ProcessingResultPayload::new().with_tasks(task),
                ))
            }
            Command::ServerModule(ServerCommands::ImportLoadedServerList { server_list }) => {
                self.server_list.extend(server_list.clone());
                Ok(ProcessingResult::Processed(ProcessingResultPayload::new().with_event(Event::ServerModule(ServerEvents::ServerSelectionReadyForDisplay))))
            }
            Command::ServerModule(ServerCommands::ServerSelectionUp) => Ok(NothingDone),
            Command::ServerModule(ServerCommands::ServerSelectionDown) => Ok(NothingDone),
            Command::ServerModule(ServerCommands::SelectServer) => Ok(ProcessingResult::Processed(
                ProcessingResultPayload::new().with_event(Event::ServerModule(
                    ServerEvents::ServerSelected {
                        server: self.server_list[0].clone(),
                    },
                )),
            )),
            _ => Ok(NothingDone),
        }
    }

    fn render(&self, frame: &mut Frame) {
        let paragraph = Paragraph::new("Server selection");
        frame.render_widget(paragraph, frame.area());
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
