use crossterm::event::Event;
use dotenvy_macro::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tui_logger::TuiTracingSubscriberLayer;

use crate::{api::client::ArtifactsClient, app::App};

mod api;
mod app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    color_eyre::install().unwrap();
    tui_logger::init_logger(tui_logger::LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(tui_logger::LevelFilter::Debug);

    tracing_subscriber::registry()
        .with(TuiTracingSubscriberLayer)
        .init();

    let api_token = dotenv!("ARTIFACTS_API_TOKEN");
    let api = ArtifactsClient::new(api_token);

    let mut terminal = ratatui::init();
    terminal.clear()?;
    terminal.hide_cursor()?;

    let result = App::new(api).run(terminal).await;

    ratatui::restore();
    result
}
