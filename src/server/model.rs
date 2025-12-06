use crate::model::{Command, Message, ModelTrait};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::sync::mpsc::Sender;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfiguration {
    pub server_list: Vec<String>,
    pub selected_server_index: Option<usize>,
}

impl Default for ServerConfiguration {
    fn default() -> Self {
        Self {
            server_list: vec!["http://localhost:8080".to_string()],
            selected_server_index: Some(0),
        }
    }
}

pub struct ServerModel {
    pub msg_sender: Sender<Message>,
    pub command_sender: Sender<Command>,
    pub server_list: Vec<String>,
    pub current_selected_server_index: Option<usize>,
}

#[async_trait]
impl ModelTrait<ServerMsg, ServerCommand> for ServerModel {
    async fn apply_event(&mut self, event: ServerMsg) -> Result<(), Box<dyn Error>> {
        match event {
            ServerMsg::ChangeSelectionUp => {
                self.change_server_selection_up();
                Ok(())
            }
            ServerMsg::ChangeSelectionDown => {
                self.change_server_selection_down();
                Ok(())
            }
            ServerMsg::StartNewServerRegistration => {
                self.start_new_server_registration();
                Ok(())
            }
            ServerMsg::DeleteSelectedServer => {
                self.delete_selected_server();
                Ok(())
            }
            ServerMsg::LoadConfigurationRequested => {
                self.command_sender
                    .send(Command::Server(ServerCommand::LoadConfiguration))
                    .await?;
                Ok(())
            }
            ServerMsg::ConfigurationLoaded(configuration    ) => {
                self.server_list = configuration.server_list;
                self.current_selected_server_index = configuration.selected_server_index;
                Ok(())
            }
        }
    }

    async fn handle_command(&mut self, command: ServerCommand) -> Result<(), Box<dyn Error>> {
        match command {
            ServerCommand::LoadConfiguration => {
                let server_configuration: ServerConfiguration = confy::load("wiremock-tui", "servers")?;
                self.msg_sender.send(Message::Server(ServerMsg::ConfigurationLoaded(server_configuration))).await?;
                Ok(())
            }
        }
    }
}

impl ServerModel {
    pub fn new(event_sender: Sender<Message>, command_sender: Sender<Command>) -> Self {
        Self {
            msg_sender: event_sender,
            command_sender,
            server_list: vec![],
            current_selected_server_index: None,
        }
    }

    fn update_server_list(self: &mut Self, server_list: Vec<String>) {
        self.server_list = server_list;
        if self.server_list.len() > 0 {
            self.current_selected_server_index = Some(0);
        }
    }

    fn change_server_selection_up(self: &mut Self) {
        if self.current_selected_server_index.is_none() {
            return;
        }
        let next_index = self
            .current_selected_server_index
            .unwrap()
            .saturating_sub(1);
        self.current_selected_server_index = Some(next_index);
    }

    pub fn current_selected_server(&self) -> Option<&String> {
        self.current_selected_server_index
            .and_then(|i| self.server_list.get(i))
    }

    fn change_server_selection_down(self: &mut Self) {
        if self.current_selected_server_index.is_none() {
            return;
        }
        let next_index =
            (self.current_selected_server_index.unwrap() + 1).min(self.server_list.len() - 1);
        self.current_selected_server_index = Some(next_index);
    }

    fn add_new_server(&self, server_url: String) {
        println!("Adding new server: {server_url}");
    }
    fn start_new_server_registration(&self) {}

    fn delete_selected_server(&self) {
        if self.current_selected_server_index.is_none() {
            return;
        }
    }
}

pub enum ServerCommand {
    LoadConfiguration,
}

pub enum ServerMsg {
    LoadConfigurationRequested,
    ConfigurationLoaded(ServerConfiguration),
    ChangeSelectionUp,
    ChangeSelectionDown,
    StartNewServerRegistration,
    DeleteSelectedServer,
}
