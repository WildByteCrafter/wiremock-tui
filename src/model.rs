use crate::configuration::model::{ConfigurationCommand, ConfigurationEvent, ConfigurationModel};
use crate::server::model::{ServerCommand, ServerEvent, ServerModel};
use crate::server::server_edit_screen::ServerEditScreen;
use crate::server::server_selection_screen::ServerSelectionScreen;
use crate::stub::model::StubCommand;
use crate::stub::stub_screen::StubScreen;
use crate::{configuration, stub};
use async_trait::async_trait;
use crossterm::event::Event;
use ratatui::Frame;
use std::error::Error;
use stub::model::StubEvent;
use stub::model::StubModel;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct ApplicationModel {
    pub screen: Option<Box<dyn ScreenTrait + Send>>,
    pub config_model: ConfigurationModel,
    pub server_model: ServerModel,
    pub stub_model: StubModel,
    pub async_channel_receiver: (Sender<ApplicationEvent>, Receiver<ApplicationEvent>),
}

#[async_trait]
impl ModelTrait<GlobalEvent, GlobalCommand> for ApplicationModel {
    async fn apply_event(&mut self, event: GlobalEvent) -> Option<Command> {
        match event {
            GlobalEvent::SwitchToStubScreen => {
                let selected_server = self.server_model.current_selected_server();
                self.stub_model.selected_server_url = selected_server.cloned();
                self.switch_to_main_screen();
            }
            GlobalEvent::SwitchToServerSelectionScreen => {
                self.switch_to_server_selection_screen();
            }
            GlobalEvent::SwitchToConnectionEditScreen => {
                self.switch_to_server_edit_screen();
            }
        }
        None
    }

    async fn handle_command(&mut self, _: GlobalCommand) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl ApplicationModel {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let event_channel = mpsc::channel::<ApplicationEvent>(100);
        let application_model = ApplicationModel {
            screen: None,
            config_model: ConfigurationModel::new(event_channel.0.clone()),
            server_model: ServerModel::new(event_channel.0.clone()),
            stub_model: StubModel::new(event_channel.0.clone()),
            async_channel_receiver: event_channel,
        };
        Ok(application_model)
    }

    fn switch_to_main_screen(self: &mut Self) {
        self.screen = Some(Box::new(StubScreen::new(
            self.async_channel_receiver.0.clone(),
        )));
    }

    fn switch_to_server_selection_screen(self: &mut Self) {
        self.screen = Some(Box::new(ServerSelectionScreen::new(
            self.async_channel_receiver.0.clone(),
        )));
    }

    fn switch_to_server_edit_screen(self: &mut Self) {
        self.screen = Some(Box::new(ServerEditScreen::new(
            self.async_channel_receiver.0.clone(),
        )));
    }
}

pub enum Command {
    Configuration(ConfigurationCommand),
    Server(ServerCommand),
    Stub(StubCommand),
    Global(GlobalCommand),
}

pub enum ApplicationEvent {
    QuitApplication,
    Config(ConfigurationEvent),
    Global(GlobalEvent),
    Server(ServerEvent),
    Stub(StubEvent),
}

pub enum GlobalEvent {
    SwitchToStubScreen,
    SwitchToServerSelectionScreen,
    SwitchToConnectionEditScreen,
}

pub enum GlobalCommand {}

#[async_trait]
pub trait ModelTrait<E, C> {
    async fn apply_event(&mut self, event: E) -> Option<Command>;
    async fn handle_command(&mut self, command: C) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait ScreenTrait {
    fn draw(&self, app: &ApplicationModel, f: &mut Frame);
    async fn handle_key_event(&self, key_event: &Event) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Global error occurred: {0}")]
    Global(#[from] GlobalError),

    #[error("Stub error occurred: {0}")]
    Stub(#[from] stub::model::StubError),

    #[error("Configuration error occurred: {0}")]
    Config(#[from] configuration::model::ConfigurationError),
}

#[derive(Error, Debug)]
pub enum GlobalError {
    #[error("User exit requested")]
    UserRequestedExit,
}
