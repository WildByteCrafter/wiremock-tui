use crate::contract::contract_module::{ApplicationCommands, Command, Event, Module, Task};
use crate::contract::contract_processing::ProcessingResult::NothingDone;
use crate::contract::contract_processing::{ProcessingResult, ProcessingResultPayload};
use crate::contract::contract_trigger::{ActiveForMode, CommandTrigger, CommandTriggerPayload};
use color_eyre::Report;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

#[derive(Clone, Debug)]
pub enum ServerEvents {
    ServerSelectionReadyForDisplay,
    ServerSelected { server: String },
}

#[derive(Clone, Debug)]
pub enum ServerCommands {
    ShowServerSelection,
    ImportLoadedServerList { server_list: Vec<String> },
    ServerSelectionUp,
    ServerSelectionDown,
    SelectServer,
}

#[derive(Clone, Debug)]
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

const MODULE_NAME: &'static str = "server";

impl Module for ServerModule {
    fn name(&self) -> &'static str {
        MODULE_NAME
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
                Ok(ProcessingResult::Processed(
                    ProcessingResultPayload::new()
                        .with_event(Event::ServerModule(
                            ServerEvents::ServerSelectionReadyForDisplay,
                        ))
                        .with_command(Command::Application(
                            ApplicationCommands::SetCommandTriggers {
                                command_trigger_payload: CommandTriggerPayload {
                                    module_name: MODULE_NAME,
                                    command_triggers: vec![
                                        CommandTrigger {
                                            module_name: MODULE_NAME,
                                            command_name: "up",
                                            active_for_mode: ActiveForMode::Navigation,
                                            command: Command::ServerModule(
                                                ServerCommands::ServerSelectionUp,
                                            ),
                                            triggers: vec![KeyEvent {
                                                code: KeyCode::Char('j'),
                                                modifiers: KeyModifiers::NONE,
                                                kind: KeyEventKind::Press,
                                                state: KeyEventState::NONE,
                                            }],
                                        },
                                        CommandTrigger {
                                            module_name: MODULE_NAME,
                                            command_name: "down",
                                            active_for_mode: ActiveForMode::Navigation,
                                            command: Command::ServerModule(
                                                ServerCommands::ServerSelectionDown,
                                            ),
                                            triggers: vec![KeyEvent {
                                                code: KeyCode::Char('k'),
                                                modifiers: KeyModifiers::NONE,
                                                kind: KeyEventKind::Press,
                                                state: KeyEventState::NONE,
                                            }],
                                        },
                                    ],
                                },
                            },
                        )),
                ))
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

    fn render(&self, frame: &mut Frame, rect: Rect) {
        let paragraph = Paragraph::new("Server selection");
        frame.render_widget(paragraph, rect);
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
