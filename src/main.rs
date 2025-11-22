use crate::configuration::model::ConfigurationEvent;
use crate::model::{ApplicationEvent, Command, GlobalEvent, ModelTrait};
use crossterm::event::EventStream;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::StreamExt;
use model::ApplicationModel;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::error::Error;
use std::io;
use std::time::Duration;
use tokio::time;

mod configuration;
mod model;
mod server;
mod stub;
mod wire_mock;
mod ui;

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
    let mut reader = EventStream::new();
    send_initial_events(app).await?;
    loop {
        tokio::select! {

            event_option = app.event_channel.1.recv() => {
                 if let Some(msg) = event_option {
                    let _ = match msg {
                        ApplicationEvent::Global(ev) => app.apply_event(ev).await,
                        ApplicationEvent::Configuration(ev) => app.config_model.apply_event(ev).await,
                        ApplicationEvent::Server(ev) => app.server_model.apply_event(ev).await,
                        ApplicationEvent::Stub(ev) => app.stub_model.apply_event(ev).await,
                        ApplicationEvent::QuitApplication => return Ok(()),
                    };
                }
            }

            command_option = app.command_channel.1.recv() => {
                if let Some(msg) = command_option {
                    match msg{
                            Command::Global(ev) => app.handle_command(ev).await?,
                            Command::Configuration(ev) => app.config_model.handle_command(ev).await?,
                            Command::Server(ev) => app.server_model.handle_command(ev).await?,
                            Command::Stub(ev) => app.stub_model.handle_command(ev).await?,
                    }
                }
            }

            maybe_event = reader.next() => {
                    match maybe_event{
                        Some(Ok(event))  => {
                            match &app.screen{
                                Some(screen) => screen.handle_key_event(&event).await?,
                                None => {}
                            }
                        }
                        _ => ()
                    }
                }

            _ = time::sleep(Duration::from_millis(70)) => {
                match &app.screen {
                    Some(screen) =>   {terminal.draw(|f| screen.draw(app, f))?;
                    },
                _ => {}
                }
            }
        }
    }
}

async fn send_initial_events(app: &mut ApplicationModel) -> Result<(), Box<dyn Error>> {
    app.event_channel
        .0
        .send(ApplicationEvent::Global(
            GlobalEvent::SwitchToServerSelectionScreen,
        ))
        .await?;
    app.event_channel
        .0
        .send(ApplicationEvent::Configuration(
            ConfigurationEvent::LoadConfigurationRequested,
        ))
        .await?;
    Ok(())
}
