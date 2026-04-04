mod app;
mod events;
mod nuget;
mod projects;
mod transformations;
mod types;
mod ui;

use tracing::info;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file_appender = tracing_appender::rolling::daily("logs", "nuget-tui.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();

    info!("Starting nuget-tui");

    let mut terminal = ratatui::init();
    app::App::default().run(&mut terminal).await?;
    ratatui::restore();
    Ok(())
}
