use crate::model::{ApplicationEvent, Command, ModelTrait};
use async_trait::async_trait;
use std::error::Error;
use tokio::sync::mpsc::Sender;

pub struct ServerModel {
    pub event_sender: Sender<ApplicationEvent>,
    pub server_list: Vec<String>,
    pub current_selected_server_index: Option<usize>,
}

#[async_trait]
impl ModelTrait<ServerEvent, ServerCommand> for ServerModel {
    async fn apply_event(&mut self, event: ServerEvent) -> Option<Command> {
        match event {
            ServerEvent::ChangeSelectionUp => {
                self.change_server_selection_up();
            }
            ServerEvent::ChangeSelectionDown => {
                self.change_server_selection_down();
            }
            ServerEvent::StartNewServerRegistration => {
                self.start_new_server_registration();
            }
            ServerEvent::AddNewServer { server_url } => {
                self.add_new_server(server_url);
            }
            ServerEvent::DeleteSelectedServer => {
                self.delete_selected_server();
            }
            ServerEvent::ServerListUpdated(server_list) => self.update_server_list(server_list),
        }
        None
    }

    async fn handle_command(&mut self, command: ServerCommand) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl ServerModel {
    pub fn new(event_sender: Sender<ApplicationEvent>) -> Self {
        Self {
            event_sender,
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

pub enum ServerCommand {}

pub enum ServerEvent {
    ServerListUpdated(Vec<String>),
    ChangeSelectionUp,
    ChangeSelectionDown,
    StartNewServerRegistration,
    AddNewServer { server_url: String },
    DeleteSelectedServer,
}
