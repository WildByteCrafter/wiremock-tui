use crate::connection_screen::ConnectionScreen;
use crate::main_screen::MainScreen;
use crate::{AppError, ScreenTrait};

pub struct App {
    pub screen: Box<dyn ScreenTrait>,
    pub server_list: Vec<&'static str>,
    pub current_selected_server: &'static str,
    pub current_selected_server_index: usize,
}

impl App {
    pub fn new() -> Self {
        let servers = vec!["localhost:8080", "localhost:8081", "localhost:8082"];
        App {
            screen: Box::new(ConnectionScreen::new()),
            server_list: servers,
            current_selected_server: "localhost:8080",
            current_selected_server_index: 0,
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
        Msg::Quit => Err(Box::new(AppError::UserExit)),
        Msg::None => Ok(()),
    };
}

pub enum Msg {
    SwitchToMainScreen,
    ChangeServerSelectionUp,
    ChangeServerSelectionDown,
    Quit,
    None,
}
