use crate::connection_edit_screen::ConnectionEditScreen;
use crate::connection_selection_screen::ConnectionSelectionScreen;
use crate::model::StubEvent::{
    DeleteSelected, ReadAllStubs, ScrollDetailsDown, ScrollDetailsUp, SelectNext, SelectPrevious,
    ToggleAutoRefresh,
};
use crate::stub_screen::StubScreen;
use crate::wire_mock_client::{delete_stub, get_all_stubs, StubMapping};
use crate::{AppError, ScreenTrait};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::{interval, Duration};

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
    pub server_selection: ServerConnectionModel,
    pub stub_model: StubModel,
    pub config: AppConfig,
    pub async_channel_receiver: (Sender<ApplicationEvent>, Receiver<ApplicationEvent>),
}

impl ApplicationModel {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let cfg: AppConfig = confy::load("wm-tui", None)?;
        let event_channel = mpsc::channel::<ApplicationEvent>(100);
        let application_model = ApplicationModel {
            screen: Box::new(ConnectionSelectionScreen::new()),
            server_selection: ServerConnectionModel::new(&cfg, event_channel.0.clone()),
            stub_model: StubModel::new(event_channel.0.clone()),
            config: cfg,
            async_channel_receiver: event_channel,
        };
        Ok(application_model)
    }

    fn save_configuration(&mut self) -> Result<(), AppError> {
        self.config.selected_server_index = self.server_selection.current_selected_server_index;
        self.config.server_list = self.server_selection.server_list.clone();
        confy::store("wm-tui", None, &self.config).map_err(|e| AppError::StoreConfigurationError(e.to_string()))
    }

    fn switch_to_main_screen(self: &mut Self) {
        self.screen = Box::new(StubScreen::new());
    }

    fn switch_to_server_selection_screen(self: &mut Self) {
        self.screen = Box::new(ConnectionSelectionScreen::new());
    }

    fn switch_to_server_edit_screen(self: &mut Self) {
        self.screen = Box::new(ConnectionEditScreen::new())
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

pub struct ServerConnectionModel {
    pub event_sender: Sender<ApplicationEvent>,
    pub server_list: Vec<String>,
    pub current_selected_server_index: Option<usize>,
}

impl ServerConnectionModel {
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

pub struct StubModel {
    pub selected_server_url: Option<String>,
    pub event_sender: Sender<ApplicationEvent>,
    pub stubs: Vec<StubMapping>,
    pub selected_stub_index: usize,
    pub scroll_offset: usize,
    pub refresh_task: Option<tokio::task::JoinHandle<()>>,
}

impl StubModel {
    fn new(event_sender: Sender<ApplicationEvent>) -> Self {
        Self {
            selected_server_url: None,
            event_sender,
            stubs: vec![],
            selected_stub_index: 0,
            scroll_offset: 0,
            refresh_task: None,
        }
    }

    fn handle_event(&mut self, ev: StubEvent) -> Result<(), Box<dyn Error>> {
        use StubEvent::*;
        match ev {
            SelectNext => {
                self.select_next_stub();
                Ok(())
            }
            SelectPrevious => {
                self.select_previous_stub();
                Ok(())
            }
            ScrollDetailsUp => {
                self.scroll_details_up();
                Ok(())
            }
            ScrollDetailsDown => {
                self.scroll_details_down();
                Ok(())
            }
            DeleteSelected => {
                self.delete_selected_stub()?;
                Ok(())
            }
            ReadAllStubs => self.read_all_stubs(),
            ToggleAutoRefresh => {
                self.toggle_auto_refresh_stubs();
                Ok(())
            }
        }
    }

    fn read_all_stubs(&mut self) -> Result<(), Box<dyn Error>> {
        if self.selected_server_url.is_none() {
            return Err(Box::new(AppError::NoServerSelected));
        }
        let res = get_all_stubs(&self.selected_server_url.as_ref().unwrap())?;
        self.stubs = res.mappings;
        Ok(())
    }

    fn select_next_stub(&mut self) {
        if self.stubs.is_empty() {
            return;
        }
        self.selected_stub_index = (self.selected_stub_index + 1).min(self.stubs.len() - 1);
        self.scroll_offset = 0;
    }

    fn select_previous_stub(&mut self) {
        self.selected_stub_index = self.selected_stub_index.saturating_sub(1);
        self.scroll_offset = 0; // Reset scroll when changing stub
    }

    fn scroll_details_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    fn scroll_details_down(&mut self) {
        self.scroll_offset += 1;
    }

    fn toggle_auto_refresh_stubs(&mut self) {
        if let Some(task) = self.refresh_task.take() {
            task.abort();
            return;
        }
        let sender = self.event_sender.clone();
        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(1));
            interval.tick().await;
            loop {
                interval.tick().await;
                if sender
                    .send(ApplicationEvent::Stub(ReadAllStubs))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        });
        self.refresh_task = Some(task);
    }

    fn delete_selected_stub(&mut self) -> Result<(), Box<dyn Error>> {
        if self.stubs.is_empty() || self.selected_server_url.is_none() {
            return Ok(());
        }
        let idx = self.selected_stub_index.min(self.stubs.len() - 1);
        if let Some(stub) = self.stubs.get(idx) {
            let id = stub.id.clone();
            // Perform delete on server
            delete_stub(self.selected_server_url.as_ref().unwrap(), &id)?;
            // Remove locally
            self.stubs.remove(idx);
            // Adjust selection
            if self.stubs.is_empty() {
                self.selected_stub_index = 0;
                self.scroll_offset = 0;
            } else {
                if idx >= self.stubs.len() {
                    self.selected_stub_index = self.stubs.len() - 1;
                }
                self.scroll_offset = 0;
            }
        }
        Ok(())
    }
}

pub enum GlobalEvent {
    SwitchToStubScreen,
    SwitchToServerSelectionScreen,
    SwitchToConnectionEditScreen,
    SaveConfiguration,
    Quit,
}

pub enum ServerEvent {
    ChangeSelectionUp,
    ChangeSelectionDown,
    StartNewServerRegistration,
    AddNewServer { server_url: String },
    DeleteSelectedServer,
}

pub enum StubEvent {
    SelectNext,
    SelectPrevious,
    ScrollDetailsUp,
    ScrollDetailsDown,
    DeleteSelected,
    ReadAllStubs,
    ToggleAutoRefresh,
}

pub enum ApplicationEvent {
    None,
    Global(GlobalEvent),
    Server(ServerEvent),
    Stub(StubEvent),
}
