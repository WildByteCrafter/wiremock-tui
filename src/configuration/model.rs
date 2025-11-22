use std::error::Error;
use crate::model::{ApplicationEvent, Command, ModelTrait};
use crate::server::model::ServerEvent;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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
    command_sender: Sender<Command>,
    app_config: Option<RootConfiguration>,
}

impl ConfigurationModel {
    pub fn new(event_sender: Sender<ApplicationEvent>, command_sender: Sender<Command>) -> Self {
        ConfigurationModel {
            event_sender,
            command_sender,
            app_config: None,
        }
    }
}

#[async_trait]
impl ModelTrait<ConfigurationEvent, ConfigurationCommand> for ConfigurationModel {
    async fn apply_event(&mut self, event: ConfigurationEvent) -> Result<(), Box<dyn Error>> {
        match event {
            ConfigurationEvent::LoadConfigurationRequested => {
                self.command_sender
                    .send(Command::Configuration(
                        ConfigurationCommand::LoadConfiguration,
                    ))
                    .await?;
                Ok(())
            }
            ConfigurationEvent::ConfigurationLoaded(ev) => {
                self.app_config = Some(ev);
                let vec = self
                    .app_config
                    .as_ref()
                    .ok_or("Configuration is missing")?
                    .server_list
                    .clone();
                self.event_sender
                    .send(ApplicationEvent::Server(ServerEvent::ServerListUpdated(
                        vec,
                    )))
                    .await?;
                Ok(())
            }
        }
    }

    async fn handle_command(
        &mut self,
        command: ConfigurationCommand,
    ) -> Result<(), Box<dyn Error>> {
        match command {
            ConfigurationCommand::LoadConfiguration => {
                let app_config: RootConfiguration = confy::load("wm-tui", None)
                    .map_err(|e| ConfigurationError::StoreConfigurationError(e))?;
                self.event_sender
                    .send(ApplicationEvent::Configuration(
                        ConfigurationEvent::ConfigurationLoaded(app_config),
                    ))
                    .await?;
            }
            ConfigurationCommand::SaveConfiguration => {
                // self.app_config.selected_server_index = self.server_selection.current_selected_server_index;
                // self.app_config.server_list = self.server_selection.server_list.clone();
                // confy::store("wm-tui", None, &self.config)
                //     .map_err(|e| AppError::StoreConfigurationError(e.to_string()))
            }
        }
        Ok(())
    }
}

pub enum ConfigurationCommand {
    LoadConfiguration,
    SaveConfiguration,
}

#[derive(Debug)]
pub enum ConfigurationEvent {
    LoadConfigurationRequested,
    ConfigurationLoaded(RootConfiguration),
}

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("Failed to store configuration")]
    StoreConfigurationError(#[source] confy::ConfyError),
}
