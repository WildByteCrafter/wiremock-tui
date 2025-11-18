use crate::model::AppError;
use crate::model::{ApplicationEvent, Command, ModelTrait};
use serde::{Deserialize, Serialize};
use std::error::Error;
use thiserror::Error;
use tokio::sync::mpsc::Sender;

#[derive(Serialize, Deserialize,Debug)]
pub struct RootConfiguration {
    pub server_list: Vec<String>,
    pub selected_server_index: Option<usize>,
}

impl Default for RootConfiguration {
    fn default() -> Self {
        Self {
            server_list: vec!["http://localhost:9393".to_string()],
            selected_server_index: Some(0),
        }
    }
}

pub struct ConfigurationModel {
    event_sender: Sender<ApplicationEvent>,
    app_config: RootConfiguration,
}

impl ModelTrait<ConfigurationEvent> for ConfigurationModel {
    async fn handle_event(&mut self, event: ConfigurationEvent) {
        println!("The origin is: {event:?}")
    }

    fn handle_command(&mut self, command: Command) -> Result<(), Box<dyn Error>> {
        print!("Command {command:#?}");
        Ok(())
    }
}

impl ConfigurationModel {
    pub fn new(event_sender: Sender<ApplicationEvent>) -> Result<Self, ConfigurationError> {
        let app_config: RootConfiguration = confy::load("wm-tui", None)
            .map_err(|e| ConfigurationError::StoreConfigurationError(e))?;
        Ok(ConfigurationModel {
            event_sender,
            app_config,
        })
    }

    fn save_configuration(&mut self) -> Result<(), AppError> {
        // self.app_config.selected_server_index = self.server_selection.current_selected_server_index;
        // self.app_config.server_list = self.server_selection.server_list.clone();
        // confy::store("wm-tui", None, &self.config)
        //     .map_err(|e| AppError::StoreConfigurationError(e.to_string()))
        Ok(())
    }
}

pub enum ConfigurationCmd {
    LoadConfiguration,
}

#[derive(Debug)]
pub enum ConfigurationEvent {
    ConfigurationLoaded(RootConfiguration),
}

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("Failed to store configuration")]
    StoreConfigurationError(#[source] confy::ConfyError),
}
