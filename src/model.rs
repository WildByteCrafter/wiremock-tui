use crate::server::model::{ServerEvent, ServerModel};
use crate::server::server_edit_screen::ServerEditScreen;
use crate::server::server_selection_screen::ServerSelectionScreen;
use crate::stub::stub_screen::StubScreen;
use crate::{configuration, stub};
use ratatui::Frame;
use std::error::Error;
use std::io;
use stub::model::StubEvent;
use stub::model::StubModel;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct ApplicationModel {
    pub screen: Box<dyn ScreenTrait>,
    pub server_selection: ServerModel,
    pub stub_model: StubModel,
    pub async_channel_receiver: (Sender<ApplicationEvent>, Receiver<ApplicationEvent>),
}

impl ModelTrait<GlobalEvent> for ApplicationModel {
    fn handle_event(&mut self, event: GlobalEvent) -> Result<Option<Command>, Box<dyn Error>> {
        match event {
            GlobalEvent::SwitchToStubScreen => {
                let selected_server = self.server_selection.current_selected_server();
                self.stub_model.selected_server_url = selected_server.cloned();
                self.switch_to_main_screen();
                Ok(None)
            }
            GlobalEvent::SwitchToServerSelectionScreen => {
                self.switch_to_server_selection_screen();
                Ok(None)
            }
            GlobalEvent::SwitchToConnectionEditScreen => {
                self.switch_to_server_edit_screen();
                Ok(None)
            }
            GlobalEvent::Quit => Err(Box::new(GlobalError::UserRequestedExit)),
        }
    }
}

impl ApplicationModel {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let event_channel = mpsc::channel::<ApplicationEvent>(100);
        let application_model = ApplicationModel {
            screen: Box::new(ServerSelectionScreen::new()),
            server_selection: ServerModel::new(event_channel.0.clone()),
            stub_model: StubModel::new(event_channel.0.clone()),
            async_channel_receiver: event_channel,
        };
        Ok(application_model)
    }

    fn switch_to_main_screen(self: &mut Self) {
        self.screen = Box::new(StubScreen::new());
    }

    fn switch_to_server_selection_screen(self: &mut Self) {
        self.screen = Box::new(ServerSelectionScreen::new());
    }

    fn switch_to_server_edit_screen(self: &mut Self) {
        self.screen = Box::new(ServerEditScreen::new())
    }
}

pub enum Command {
    None,
}

pub enum ApplicationEvent {
    Global(GlobalEvent),
    Server(ServerEvent),
    Stub(StubEvent),
}

pub enum GlobalEvent {
    SwitchToStubScreen,
    SwitchToServerSelectionScreen,
    SwitchToConnectionEditScreen,
    Quit,
}

pub trait ModelTrait<T> {
    fn handle_event(&mut self, event: T) -> Result<Option<Command>, Box<dyn Error>>;
}

pub trait ScreenTrait {
    fn draw(&self, app: &ApplicationModel, f: &mut Frame);
    fn event_handling(&self) -> Result<Option<ApplicationEvent>, io::Error>;
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
