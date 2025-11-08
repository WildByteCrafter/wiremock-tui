use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use io::Error;
use ratatui::{Frame, Terminal, backend::CrosstermBackend};
use std::io;

use application_model::App;
use thiserror::Error;

mod application_model;
mod connection_screen;
mod main_screen;

enum Msg {
    SwitchToMainScreen,
    ChangeServerSelectionUp,
    ChangeServerSelectionDown,
    Quit,
    None,
}

fn main() -> Result<(), Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| {
            app.screen.draw(app, f);
        })?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            match event_handling(app) {
                Ok(new_app) => {}
                Err(e) => {
                    if let Some(app_error) = e.downcast_ref::<AppError>() {
                        match app_error {
                            AppError::UserExit => {
                                return Ok(());
                            }
                        }
                    }
                    return Err(e);
                }
            }
        }
    }
}

fn event_handling(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    if let Event::Key(key) = event::read()? {
        let msg = match key.code {
            KeyCode::Char('q') => Msg::Quit,
            KeyCode::Up | KeyCode::Char('k') => Msg::ChangeServerSelectionUp,
            KeyCode::Down | KeyCode::Char('j') => Msg::ChangeServerSelectionDown,
            _ => Msg::None,
        };
        return match msg {
            Msg::SwitchToMainScreen => Ok(()),
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
    Ok(())
}

trait ScreenTrait {
    fn draw(&self, app: &App, f: &mut Frame);
}

#[derive(Error, Debug)]
enum AppError {
    #[error("User exit")]
    UserExit,
}
