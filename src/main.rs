use crate::model::{ApplicationEvent, Command, ModelTrait};
use crossterm::{
    event::{self},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use model::ApplicationModel;
use model::{AppError, GlobalError, ScreenTrait};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::error::Error;
use std::io;

mod configuration;
mod model;
mod server;
mod stub;
mod wire_mock;

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
) -> Result<(), Box<dyn Error>> {
    loop {
        if let Ok(msg) = app.async_channel_receiver.1.try_recv() {
            handle_event(app, msg)?;
        }

        // Draw current screen
        terminal.draw(|f| app.screen.draw(app, f))?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Some(msg) = app.screen.event_handling()? {
                if let Err(e) = handle_event(app, msg) {
                    if let Some(app_error) = e.downcast_ref::<AppError>() {
                        if let AppError::Global(GlobalError::UserRequestedExit) = app_error {
                            return Ok(());
                        }
                    }
                    return Err(e);
                }
            }
        }
    }
}

pub fn handle_event(
    application_model: &mut ApplicationModel,
    msg: ApplicationEvent,
) -> Result<Option<Command>, Box<dyn Error>> {
    match msg {
        ApplicationEvent::Global(ev) => application_model.handle_event(ev),
        ApplicationEvent::Server(ev) => application_model.server_selection.handle_event(ev),
        ApplicationEvent::Stub(ev) => application_model.stub_model.handle_event(ev),
    }
}
