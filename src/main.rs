#![allow(dead_code)]

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use dotenvy_macro::dotenv;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};
use tui_logger::TuiTracingSubscriberLayer;

use crate::{
    actions::{FightAction, GetCharactersAction, RestAction},
    api::{Action, ParamatarisedAction, client::ArtifactsClient, maps::MapQuery},
    app::App,
};

mod actions;
mod api;
mod app;
mod macros;
mod models;

async fn periodically_update_characters(
    api: ArtifactsClient,
    character_widget: Arc<Mutex<app::CharacterWidgetState>>,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(3));

    loop {
        match GetCharactersAction.execute(&api).await {
            Ok(chars) => {
                if let Ok(mut state) = character_widget.lock() {
                    state.characters = chars;
                } else {
                    error!("Failed to lock character widget state");
                    return;
                }
            }
            Err(e) => {
                error!("Failed to fetch characters: {}", e);
            }
        }

        interval.tick().await;
    }
}

async fn sleep_until_cooldown_expired(cooldown_expiration: chrono::DateTime<chrono::Utc>) {
    let now = chrono::Utc::now();
    if cooldown_expiration > now {
        let duration = cooldown_expiration - now;
        info!(
            target: "fight-loop", "Sleeping for {} seconds until cooldown expires",
            duration.num_seconds()
        );

        tokio::time::sleep(Duration::from_secs_f64(duration.as_seconds_f64())).await;
    }
}

async fn move_to_chicken(
    api: ArtifactsClient,
    character: &models::character::Character,
) -> Option<models::character::Character> {
    let Ok(chicken) = api.get_monster("chicken").await else {
        error!(target: "fight-loop", "Failed to fetch monster 'chicken'");
        return None;
    };

    let Ok(maps) = api
        .get_maps(
            &MapQuery::default()
                .content_type(models::map::MapContentType::Monster)
                .content_code(chicken.code.clone()),
        )
        .await
    else {
        error!(target: "fight-loop", "Failed to fetch maps for monster 'chicken'");
        return None;
    };

    let Some(map) = maps.first() else {
        error!(target: "fight-loop", "No maps found for monster 'chicken'");
        return None;
    };

    let (x, y) = (map.x, map.y);

    if (character.x, character.y) != (x, y) {
        info!(target: "fight-loop", "Character {} is not at the correct location. Moving to ({}, {}).", character.name, x, y);
        match api.move_character(&character.name, x, y).await {
            Ok(data) => {
                info!(target: "fight-loop", "Character {} moved to ({}, {}).", character.name, x, y);
                return Some(data.character);
            }
            Err(e) => {
                error!(target: "fight-loop", "Failed to move character {}: {}", character.name, e);
                return None;
            }
        }
    }

    Some(character.clone())
}

async fn character_fight_loop(api: ArtifactsClient, name: String) {
    info!(target: "fight-loop", "Starting fight loop for character: {}", name);

    let mut character = match api.get_character(&name).await {
        Ok(c) => c,
        Err(e) => {
            error!(target: "fight-loop", "failed to fetch character {}: {}", name, e);
            return;
        }
    };

    if character.hp < character.max_hp {
        info!(target: "fight-loop", "Character {} needs to rest before starting the fight loop.", name);

        match RestAction.execute(&api, &character).await {
            Ok(rest_data) => {
                info!(
                    "Rest result for {}: Recovered {} HP",
                    name, rest_data.hp_restored
                );
                character = rest_data.character;

                sleep_until_cooldown_expired(rest_data.cooldown.expiration).await;
            }
            Err(e) => {
                error!("Failed to rest character {}: {}", name, e);
                return;
            }
        }
    }

    info!(target: "fight-loop", "Fetched character: {:?}", character);

    #[allow(unused_assignments)]
    let mut lost_hp = 0;

    if let Some(updated_character) = move_to_chicken(api.clone(), &character).await {
        character = updated_character;
    } else {
        error!(target: "fight-loop", "Failed to move character {} to chicken location.", name);
        return;
    }

    loop {
        if character.cooldown_expiration > chrono::Utc::now() {
            info!(target: "fight-loop", "Character {} is on cooldown. Waiting for cooldown to expire.", name);
            sleep_until_cooldown_expired(character.cooldown_expiration).await;
        }

        info!(target: "fight-loop", "Preparing to fight with character: {}", name);
        match FightAction.execute(&api, &character).await {
            Ok(fight_data) => {
                info!("Fight result for {}: {:?}", name, fight_data);
                lost_hp = character.hp - fight_data.character.hp;
                character = fight_data.character;

                sleep_until_cooldown_expired(fight_data.cooldown.expiration).await;
            }
            Err(e) => {
                error!("Failed to fight with character {}: {}", name, e);
                return;
            }
        }

        if lost_hp < character.hp {
            info!(target: "fight-loop", "Character {} can continue fighting. Skipping rest.", name);
            continue;
        }

        info!(target: "fight-loop", "Character {} needs to rest. Lost HP: {} ({}/{})",
            name, lost_hp, character.hp, character.max_hp);

        match RestAction.execute(&api, &character).await {
            Ok(rest_data) => {
                info!("Rest result for {}: {:?}", name, rest_data);
                character = rest_data.character;

                sleep_until_cooldown_expired(rest_data.cooldown.expiration).await;
            }
            Err(e) => {
                error!("Failed to rest character {}: {}", name, e);
                return;
            }
        }

        info!(target: "fight-loop", "Character {} rested and is ready for the next fight.", name);
    }
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        ratatui::restore();
        hook(info);
    }));
}

fn configure_logging() {
    let log_path = "./artifacts.log";
    let log_file = std::fs::File::create(log_path).expect("Failed to create log file");

    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(EnvFilter::from_default_env().add_directive(LevelFilter::DEBUG.into()));

    tui_logger::init_logger(tui_logger::LevelFilter::Trace).unwrap();
    tui_logger::set_default_level(tui_logger::LevelFilter::Trace);

    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .with(TuiTracingSubscriberLayer)
        .init();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    color_eyre::install().unwrap();
    configure_logging();

    let api_token = dotenv!("ARTIFACTS_API_TOKEN");
    let client = ArtifactsClient::new(api_token);

    let mut terminal = ratatui::init();
    terminal.clear()?;
    terminal.hide_cursor()?;

    let token = CancellationToken::new();
    let app = App::new(client.clone());

    let tok = token.clone();
    let client_clone = client.clone();
    let char_fight_handle = tokio::spawn(async move {
        tokio::select! {
            _ = character_fight_loop(client_clone, "Penguin".to_string()) => {
                info!("Character fight loop completed");
            }
            _ = tok.cancelled() => {
                info!("Character fight loop cancelled");
            }
        }
    });

    let tok = token.clone();
    let client_clone = client.clone();
    let character_widget_state = app.character_widget_state();
    let update_char_handle = tokio::spawn(async move {
        tokio::select! {
            _ = periodically_update_characters(client_clone, character_widget_state) => {
                info!("Character update loop completed");
            }
            _ = tok.cancelled() => {
                info!("Character update loop cancelled");
            }
        }
    });

    _ = app.run(terminal).await;

    token.cancel();
    _ = tokio::join!(char_fight_handle, update_char_handle);

    ratatui::restore();

    Ok(())
}
