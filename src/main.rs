use crossterm::{
    event::{self},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use io::Error;
use model::ApplicationModel;
use model::ApplicationEvent;
use ratatui::{backend::CrosstermBackend, Frame, Terminal};
use std::io;
use thiserror::Error;

mod connection_selection_screen;
mod main_screen;
mod model;
mod wire_mock_client;
mod connection_edit_screen;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = ApplicationModel::new()?;
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
    app: &mut ApplicationModel,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        if let Ok(msg) = app.async_channel_receiver.1.try_recv() {
            app.handle_event(msg)?;
        }

        // Draw current screen
        terminal.draw(|f| app.screen.draw(app, f))?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Some(msg) = app.screen.event_handling()? {
                if let Err(e) = app.handle_event(msg) {
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
    fn draw(&self, app: &ApplicationModel, f: &mut Frame);
    fn event_handling(&self) -> Result<Option<ApplicationEvent>, Error>;
}

#[derive(Error, Debug)]
enum AppError {
    #[error("User exit")]
    UserExit,
    #[error("No wire mock server selected")]
    NoServerSelected,
}
