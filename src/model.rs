use crate::server::model::{ServerEvent, ServerModel};
use crate::server::server_edit_screen::ServerEditScreen;
use crate::server::server_selection_screen::ServerSelectionScreen;
use crate::stub::stub_screen::StubScreen;
use crate::{stub, AppError, ScreenTrait};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use stub::model::StubModel;
use stub::model::StubEvent;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub server_list: Vec<String>,
    pub selected_server_index: Option<usize>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_list: vec!["http://localhost:9393".to_string()],
            selected_server_index: Some(0),
        }
    }
}

pub struct ApplicationModel {
    pub screen: Box<dyn ScreenTrait>,
    pub server_selection: ServerModel,
    pub stub_model: StubModel,
    pub config: AppConfig,
    pub async_channel_receiver: (Sender<ApplicationEvent>, Receiver<ApplicationEvent>),
}

impl ApplicationModel {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let cfg: AppConfig = confy::load("wm-tui", None)?;
        let event_channel = mpsc::channel::<ApplicationEvent>(100);
        let application_model = ApplicationModel {
            screen: Box::new(ServerSelectionScreen::new()),
            server_selection: ServerModel::new(&cfg, event_channel.0.clone()),
            stub_model: StubModel::new(event_channel.0.clone()),
            config: cfg,
            async_channel_receiver: event_channel,
        };
        Ok(application_model)
    }

    fn save_configuration(&mut self) -> Result<(), AppError> {
        self.config.selected_server_index = self.server_selection.current_selected_server_index;
        self.config.server_list = self.server_selection.server_list.clone();
        confy::store("wm-tui", None, &self.config)
            .map_err(|e| AppError::StoreConfigurationError(e.to_string()))
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

    pub fn handle_event(&mut self, msg: ApplicationEvent) -> Result<(), Box<dyn Error>> {
        match msg {
            ApplicationEvent::None => Ok(()),
            ApplicationEvent::Global(ev) => self.handle_global_event(ev),
            ApplicationEvent::Server(ev) => self.server_selection.handle_event(ev),
            ApplicationEvent::Stub(ev) => self.stub_model.handle_event(ev),
        }
    }

    fn handle_global_event(&mut self, ev: GlobalEvent) -> Result<(), Box<dyn Error>> {
        match ev {
            GlobalEvent::SwitchToStubScreen => {
                let selected_server = self.server_selection.current_selected_server();
                self.stub_model.selected_server_url = selected_server.cloned();
                self.switch_to_main_screen();
                Ok(())
            }
            GlobalEvent::SwitchToServerSelectionScreen => {
                self.switch_to_server_selection_screen();
                Ok(())
            }
            GlobalEvent::SwitchToConnectionEditScreen => {
                self.switch_to_server_edit_screen();
                Ok(())
            }
            GlobalEvent::SaveConfiguration => {
                self.save_configuration()?;
                Ok(())
            }
            GlobalEvent::Quit => Err(Box::new(AppError::UserExit)),
        }
    }
}

pub enum GlobalEvent {
    SwitchToStubScreen,
    SwitchToServerSelectionScreen,
    SwitchToConnectionEditScreen,
    SaveConfiguration,
    Quit,
}

pub enum ApplicationEvent {
    None,
    Global(GlobalEvent),
    Server(ServerEvent),
    Stub(StubEvent),
}
