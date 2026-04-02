mod app;
mod events;
mod nuget;
mod types;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    app::App::default().run(&mut terminal).await?;
    ratatui::restore();
    Ok(())
}
