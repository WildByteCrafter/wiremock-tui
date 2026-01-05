mod app;
mod command_manager;
mod contract;
mod server;
mod stub;
mod cmd_input;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = app::App::new().run(terminal).await;
    ratatui::restore();
    result
}
