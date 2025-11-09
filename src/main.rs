use crate::model::handle_event;
use crossterm::{
    event::{self},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use io::Error;
use model::App;
use model::Msg;
use ratatui::{backend::CrosstermBackend, Frame, Terminal};
use std::io;
use thiserror::Error;

mod connection_screen;
mod main_screen;
mod model;
mod wire_mock_client;

#[tokio::main]
async fn main() -> Result<(), Error> {

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        if let Ok(msg) = app.async_channel_receiver.1.try_recv() {
            handle_event(msg, app)?;
        }

        // Draw current screen
        terminal.draw(|f| app.screen.draw(app, f))?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Some(msg) = app.screen.event_handling()? {
                if let Err(e) = handle_event(msg, app) {
                    if let Some(AppError::UserExit) = e.downcast_ref::<AppError>() {
                        return Ok(());
                    }
                    return Err(e);
                }
            }
        }
    }
}

trait ScreenTrait {
    fn draw(&self, app: &App, f: &mut Frame);
    fn event_handling(&self) -> Result<Option<Msg>, Error>;
}

#[derive(Error, Debug)]
enum AppError {
    #[error("User exit")]
    UserExit,
}
