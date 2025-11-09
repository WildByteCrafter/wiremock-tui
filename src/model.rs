use crate::connection_screen::ConnectionScreen;
use crate::main_screen::MainScreen;
use crate::wire_mock_client::{delete_stub, get_all_stubs, StubMapping};
use crate::{AppError, ScreenTrait};
use std::error::Error;

pub struct App {
    pub screen: Box<dyn ScreenTrait>,
    pub server_list: Vec<&'static str>,
    pub current_selected_server: &'static str,
    pub current_selected_server_index: usize,
    pub stubs: Vec<StubMapping>,
    pub selected_stub_index: usize,
    pub scroll_offset: usize,
}

impl App {
    pub fn new() -> Self {
        let servers = vec![
            "http://localhost:9191",
            "http://localhost:8080",
            "http://localhost:8181",
        ];
        let current_selected_server = servers[0];
        App {
            screen: Box::new(ConnectionScreen::new()),
            server_list: servers,
            current_selected_server: current_selected_server,
            current_selected_server_index: 0,
            stubs: vec![],
            selected_stub_index: 0,
            scroll_offset: 0,
        }
    }

    fn change_server_selection_up(self: &mut Self) {
        let next_index = self.current_selected_server_index.saturating_sub(1);
        self.current_selected_server_index = next_index;
        self.current_selected_server = self.server_list[next_index];
    }

    fn change_server_selection_down(self: &mut Self) {
        let next_index =
            (self.current_selected_server_index as usize + 1).min(self.server_list.len() - 1);
        self.current_selected_server_index = next_index;
        self.current_selected_server = self.server_list[next_index];
    }

    fn switch_to_main_screen(self: &mut Self) {
        self.screen = Box::new(MainScreen::new());
    }

    fn read_all_stubs(&mut self) -> Result<(), Box<dyn Error>> {
        let res = get_all_stubs(self.current_selected_server)?;
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

    fn delete_selected_stub(&mut self) -> Result<(), Box<dyn Error>> {
        if self.stubs.is_empty() {
            return Ok(());
        }
        let idx = self.selected_stub_index.min(self.stubs.len() - 1);
        if let Some(stub) = self.stubs.get(idx) {
            let id = stub.id.clone();
            // Perform delete on server
            delete_stub(self.current_selected_server, &id)?;
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
            app.change_server_selection_up();
            return Ok(());
        }
        Msg::ChangeServerSelectionDown => {
            app.change_server_selection_down();
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
    };
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
    Quit,
    None,
}
