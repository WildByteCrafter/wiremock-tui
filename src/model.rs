use crate::server::model::{ServerCommand, ServerModel, ServerMsg};
use crate::server::server_edit_screen::ServerEditScreen;
use crate::server::server_selection_screen::ServerSelectionScreen;
use crate::stub;
use crate::stub::model::StubCommand;
use crate::stub::stub_screen::StubScreen;
use async_trait::async_trait;
use crossterm::event::Event;
use ratatui::Frame;
use std::error::Error;
use stub::model::StubModel;
use stub::model::StubMsg;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct ApplicationModel {
    pub screen: Option<Box<dyn ScreenTrait + Send>>,
    pub server_model: ServerModel,
    pub stub_model: StubModel,
    pub event_channel: (Sender<Message>, Receiver<Message>),
    pub command_channel: (Sender<Command>, Receiver<Command>),
}

#[async_trait]
impl ModelTrait<GlobalMsg, GlobalCommand> for ApplicationModel {
    async fn apply_event(&mut self, event: GlobalMsg) -> Result<(), Box<dyn Error>> {
        match event {
            GlobalMsg::SwitchToStubScreen => {
                let selected_server = self.server_model.current_selected_server();
                self.stub_model.selected_server_url = selected_server.cloned();
                self.switch_to_main_screen();
                Ok(())
            }
            GlobalMsg::SwitchToServerSelectionScreen => {
                self.switch_to_server_selection_screen();
                Ok(())
            }
            GlobalMsg::SwitchToConnectionEditScreen => {
                self.switch_to_server_edit_screen();
                Ok(())
            }
        }
    }

    async fn handle_command(&mut self, _: GlobalCommand) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl ApplicationModel {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let event_channel = mpsc::channel::<Message>(100);
        let command_channel = mpsc::channel::<Command>(100);
        let application_model = ApplicationModel {
            screen: None,
            server_model: ServerModel::new(event_channel.0.clone(), command_channel.0.clone()),
            stub_model: StubModel::new(event_channel.0.clone(), command_channel.0.clone()),
            event_channel,
            command_channel,
        };
        Ok(application_model)
    }

    fn switch_to_main_screen(self: &mut Self) {
        self.screen = Some(Box::new(StubScreen::new(self.event_channel.0.clone())));
    }

    fn switch_to_server_selection_screen(self: &mut Self) {
        self.screen = Some(Box::new(ServerSelectionScreen::new(
            self.event_channel.0.clone(),
        )));
    }

    fn switch_to_server_edit_screen(self: &mut Self) {
        self.screen = Some(Box::new(ServerEditScreen::new(
            self.event_channel.0.clone(),
        )));
    }
}

pub enum Command {
    Server(ServerCommand),
    Stub(StubCommand),
    Global(GlobalCommand),
}

pub enum Message {
    QuitRequested,
    Global(GlobalMsg),
    Server(ServerMsg),
    Stub(StubMsg),
}

pub enum GlobalMsg {
    SwitchToStubScreen,
    SwitchToServerSelectionScreen,
    SwitchToConnectionEditScreen,
}

pub enum GlobalCommand {}

#[async_trait]
pub trait ModelTrait<E, C> {
    async fn apply_event(&mut self, event: E) -> Result<(), Box<dyn Error>>;
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
}

#[derive(Error, Debug)]
pub enum GlobalError {
    #[error("User exit requested")]
    UserRequestedExit,
}
