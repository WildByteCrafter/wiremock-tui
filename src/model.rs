use crate::connection_screen::ConnectionScreen;
use crate::main_screen::MainScreen;
use crate::wire_mock_client::{get_all_stubs, StubMapping};
use crate::{AppError, ScreenTrait};
use std::error::Error;

pub struct App {
    pub screen: Box<dyn ScreenTrait>,
    pub server_list: Vec<&'static str>,
    pub current_selected_server: &'static str,
    pub current_selected_server_index: usize,
    pub stubs: Vec<StubMapping>,
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
        }
    }

    pub fn change_server_selection_up(self: &mut Self) {
        let next_index = self.current_selected_server_index.saturating_sub(1);
        self.current_selected_server_index = next_index;
        self.current_selected_server = self.server_list[next_index];
    }

    pub fn change_server_selection_down(self: &mut Self) {
        let next_index =
            (self.current_selected_server_index as usize + 1).min(self.server_list.len() - 1);
        self.current_selected_server_index = next_index;
        self.current_selected_server = self.server_list[next_index];
    }

    pub(crate) fn switch_to_main_screen(self: &mut Self) {
        self.screen = Box::new(MainScreen::new());
    }

    pub(crate) fn read_all_stubs(&mut self) -> Result<(), Box<dyn Error>> {
        let res = get_all_stubs(self.current_selected_server)?;
        self.stubs = res.mappings;
        Ok(())
    }


    pub fn select_next_stub(&mut self) {
        if !self.stubs.is_empty() {
            if let Some(screen) = self.screen.as_any_mut().downcast_mut::<crate::main_screen::MainScreen>() {
                screen.selected_stub_index = (screen.selected_stub_index + 1).min(self.stubs.len() - 1);
                screen.scroll_offset = 0; // Reset scroll when changing stub
            }
        }
    }

    pub fn select_previous_stub(&mut self) {
        if let Some(screen) = self.screen.as_any_mut().downcast_mut::<crate::main_screen::MainScreen>() {
            screen.selected_stub_index = screen.selected_stub_index.saturating_sub(1);
            screen.scroll_offset = 0; // Reset scroll when changing stub
        }
    }

    pub fn scroll_details_up(&mut self) {
        if let Some(screen) = self.screen.as_any_mut().downcast_mut::<crate::main_screen::MainScreen>() {
            screen.scroll_offset = screen.scroll_offset.saturating_sub(1);
        }
    }

    pub fn scroll_details_down(&mut self) {
        if let Some(screen) = self.screen.as_any_mut().downcast_mut::<crate::main_screen::MainScreen>() {
            screen.scroll_offset += 1;
        }
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
    ReadAllStubs,
    Quit,
    None,
}