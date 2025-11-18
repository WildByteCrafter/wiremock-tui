use crate::model::AppError;
use crate::model::{ApplicationEvent, Command, ModelTrait};
use serde::{Deserialize, Serialize};
use std::error::Error;
use async_trait::async_trait;
use thiserror::Error;
use tokio::sync::mpsc::Sender;

#[derive(Serialize, Deserialize, Debug)]
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

#[async_trait]
impl ModelTrait<ConfigurationEvent, ConfigurationCommand> for ConfigurationModel {
    async fn apply_event(&mut self, _: ConfigurationEvent) -> Option<Command> {
        None
    }

    async fn handle_command(&mut self, _: ConfigurationCommand) -> Result<(), Box<dyn Error>> {
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

pub enum ConfigurationCommand {
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
