#![allow(dead_code)]

use anyhow::Context;
use dotenvy_macro::dotenv;
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tui_logger::TuiTracingSubscriberLayer;

use crate::{
    api::{QueryAction, client::ArtifactsClient, monsters::MonsterQuery},
    app::App,
};

mod api;
mod app;

async fn game_loop(api: ArtifactsClient) -> anyhow::Result<()> {
    let monster = GetMonstersAction
        .execute(&api, MonsterQuery::default().min_level(2))
        .await
        .context("Failed to execute GetMonstersAction")?;

    let monster = monster
        .iter()
        .min_by_key(|m| m.level)
        .context("No monsters found")?;

    let map_query = api::maps::MapQuery::default()
        .content_type(api::maps::MapContentType::Monster)
        .content_code(monster.code.clone());

    let map = GetMapsAction
        .execute(&api, map_query)
        .await
        .context("Failed to execute GetMapsAction")?;

    let map = map.first().context("No maps found for monster")?;

    info!(
        "Monster {:?} location: ({:?}, {:?})",
        monster.name, map.x, map.y
    );

    Ok(())
}

struct GetMonstersAction;

impl QueryAction for GetMonstersAction {
    type Query = api::monsters::MonsterQuery;
    type Return = Vec<api::monsters::Monster>;

    async fn execute(
        &self,
        api: &ArtifactsClient,
        query: Self::Query,
    ) -> anyhow::Result<Self::Return> {
        let monsters: Vec<api::monsters::Monster> = api
            .get_monsters(&query)
            .await
            .context("Failed to fetch monsters")?;

        Ok(monsters)
    }
}

pub struct GetMapsAction;

impl QueryAction for GetMapsAction {
    type Query = api::maps::MapQuery;
    type Return = Vec<api::maps::Map>;

    async fn execute(
        &self,
        api: &ArtifactsClient,
        query: Self::Query,
    ) -> anyhow::Result<Self::Return> {
        let maps: Vec<api::maps::Map> =
            api.get_maps(&query).await.context("Failed to fetch maps")?;

        Ok(maps)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    color_eyre::install().unwrap();
    tui_logger::init_logger(tui_logger::LevelFilter::Trace).unwrap();
    tui_logger::set_default_level(tui_logger::LevelFilter::Trace);

    tracing_subscriber::registry()
        .with(TuiTracingSubscriberLayer)
        .init();

    let api_token = dotenv!("ARTIFACTS_API_TOKEN");
    let api = ArtifactsClient::new(api_token);

    let mut terminal = ratatui::init();
    terminal.clear()?;
    terminal.hide_cursor()?;

    let game_loop = tokio::spawn(game_loop(api));
    let app = App::new().run(terminal).await;

    if !game_loop.is_finished() {
        warn!("Game loop is still running. Aborting...");
        game_loop.abort();
    }

    _ = game_loop.await;

    if let Err(e) = app {
        error!("Error during game loop: {}", e);
    }

    ratatui::restore();

    Ok(())
}
