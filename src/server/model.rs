use std::error::Error;
use tokio::sync::mpsc::Sender;
use crate::model::{AppConfig, ApplicationEvent};

pub struct ServerModel {
    pub event_sender: Sender<ApplicationEvent>,
    pub server_list: Vec<String>,
    pub current_selected_server_index: Option<usize>,
}

impl ServerModel {
    pub fn new(app_config: &AppConfig, event_sender: Sender<ApplicationEvent>) -> Self {
        Self {
            event_sender,
            server_list: app_config.server_list.clone(),
            current_selected_server_index: app_config.selected_server_index,
        }
    }

    pub fn handle_event(&mut self, ev: ServerEvent) -> Result<(), Box<dyn Error>> {
        match ev {
            ServerEvent::ChangeSelectionUp => {
                self.change_server_selection_up();
                Ok(())
            }
            ServerEvent::ChangeSelectionDown => {
                self.change_server_selection_down();
                Ok(())
            }
            ServerEvent::StartNewServerRegistration => {
                self.start_new_server_registration();
                Ok(())
            }
            ServerEvent::AddNewServer { server_url } => {
                self.add_new_server(server_url);
                Ok(())
            }
            ServerEvent::DeleteSelectedServer => {
                self.delete_selected_server();
                Ok(())
            }
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

    fn add_new_server(&self, p0: String) {}
    fn start_new_server_registration(&self) {}

    fn delete_selected_server(&self) {
        if self.current_selected_server_index.is_none() {
            return;
        }
    }
}

pub enum ServerEvent {
    ChangeSelectionUp,
    ChangeSelectionDown,
    StartNewServerRegistration,
    AddNewServer { server_url: String },
    DeleteSelectedServer,
}