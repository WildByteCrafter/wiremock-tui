use crate::connection_screen::ConnectionScreen;
use crate::main_screen::MainScreen;
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

impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        Self {
            server_list: vec!["localhost:8080".to_string()],
            selected_server_index: Some(0),
        }
    }
}

pub struct App {
    pub screen: Box<dyn ScreenTrait>,
    pub server_selection: ServerSelection,
    pub stubs: Vec<StubMapping>,
    pub selected_stub_index: usize,
    pub scroll_offset: usize,
    pub async_channel_receiver: (Sender<Msg>, Receiver<Msg>),
    pub refresh_task: Option<tokio::task::JoinHandle<()>>,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let cfg: AppConfig = confy::load("wm-tui", None)?;
        Ok(App {
            screen: Box::new(ConnectionScreen::new()),
            server_selection: ServerSelection::new(&cfg),
            stubs: vec![],
            selected_stub_index: 0,
            scroll_offset: 0,
            async_channel_receiver: mpsc::channel::<Msg>(100),
            refresh_task: None,
        })
    }

    fn switch_to_main_screen(self: &mut Self) {
        self.screen = Box::new(MainScreen::new());
    }

    fn read_all_stubs(&mut self) -> Result<(), Box<dyn Error>> {
        if self.server_selection.current_selected_server().is_none() {
            return Err(Box::new(AppError::NoServerSelected));
        }
        let res = get_all_stubs(self.server_selection.current_selected_server().unwrap())?;
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
        let send = self.async_channel_receiver.0.clone();
        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(1));
            interval.tick().await;
            loop {
                interval.tick().await;
                if send.send(Msg::ReadAllStubs).await.is_err() {
                    break;
                }
            }
        });
        self.refresh_task = Some(task);
    }

    fn delete_selected_stub(&mut self) -> Result<(), Box<dyn Error>> {
        if self.stubs.is_empty() || self.server_selection.current_selected_server().is_none() {
            return Ok(());
        }
        let idx = self.selected_stub_index.min(self.stubs.len() - 1);
        if let Some(stub) = self.stubs.get(idx) {
            let id = stub.id.clone();
            // Perform delete on server
            delete_stub(
                self.server_selection.current_selected_server().unwrap(),
                &id,
            )?;
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

pub fn handle_event(msg: Msg, app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    return match msg {
        Msg::SwitchToMainScreen => {
            app.switch_to_main_screen();
            return Ok(());
        }
        Msg::ChangeServerSelectionUp => {
            app.server_selection.change_server_selection_up();
            return Ok(());
        }
        Msg::ChangeServerSelectionDown => {
            app.server_selection.change_server_selection_down();
            Ok(())
        }
        Msg::SelectNextStub => {
            app.select_next_stub();
            Ok(())
        }
        Msg::SelectPreviousStub => {
            app.select_previous_stub();
            Ok(())
        }
        Msg::ScrollDetailsUp => {
            app.scroll_details_up();
            Ok(())
        }
        Msg::ScrollDetailsDown => {
            app.scroll_details_down();
            Ok(())
        }
        Msg::DeleteSelectedStub => {
            app.delete_selected_stub()?;
            Ok(())
        }
        Msg::Quit => Err(Box::new(AppError::UserExit)),
        Msg::None => Ok(()),
        Msg::ReadAllStubs => app.read_all_stubs(),
        Msg::ToggleAutoRefreshStubs => {
            app.toggle_auto_refresh_stubs();
            Ok(())
        }
    };
}

pub struct ServerSelection {
    pub server_list: Vec<String>,
    pub current_selected_server_index: Option<usize>,
}

impl ServerSelection {
    pub fn new(app_config: &AppConfig) -> Self {
        Self {
            server_list: app_config.server_list.clone(),
            current_selected_server_index: app_config.selected_server_index,
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
}

pub enum Msg {
    SwitchToMainScreen,
    ChangeServerSelectionUp,
    ChangeServerSelectionDown,
    SelectNextStub,
    SelectPreviousStub,
    ScrollDetailsUp,
    ScrollDetailsDown,
    DeleteSelectedStub,
    ReadAllStubs,
    ToggleAutoRefreshStubs,
    Quit,
    None,
}
