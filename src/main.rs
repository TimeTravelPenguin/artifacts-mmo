use std::{fmt::Debug, sync::Arc, time::Duration};

use anyhow::Result;
use api::characters::models::Character;
use api::compatibility::FromLegacy;
use openapi::{
    apis::{Api, ApiClient, configuration::Configuration},
    models::MyAccountDetails,
};

use iced::{Element, Task};
use iced::{Theme, widget::text};
use tokio::{sync::Mutex, time::sleep};
use tracing::{error, info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;
//use widgets::character_info::CharacterInfo;
mod api;
//mod widgets;

/// A generic retry helper that retries an async operation with exponential backoff.
///
/// - `operation`: A closure that returns a Future which outputs a `Result<T, E>`.
/// - `max_retries`: Maximum number of attempts before giving up.
/// - `initial_delay`: The starting delay duration between attempts.
///
/// Returns the successful value or the last error.
async fn retry<T, E, Fut, F>(
    mut operation: F,
    max_retries: usize,
    initial_delay: Duration,
) -> Result<T, E>
where
    E: Debug,
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut attempt = 0;
    let mut delay = initial_delay;

    loop {
        match operation().await {
            Ok(value) => return Ok(value),
            Err(e) => {
                attempt += 1;
                if attempt > max_retries {
                    error!("Operation failed after {} attempts: {:?}", attempt, e);
                    return Err(e);
                } else {
                    error!(
                        "Attempt {} failed: {:?}. Retrying in {} seconds...",
                        attempt,
                        e,
                        delay.as_secs()
                    );
                    sleep(delay).await;
                    delay *= 2;
                }
            }
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install().map_err(|e| anyhow::anyhow!(e))?;

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::WARN.into())
        .from_env()?
        .add_directive("artifacts=info".parse()?);

    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .pretty()
        .with_line_number(true)
        .with_writer(non_blocking)
        .init();

    dotenv::dotenv().ok();
    let Ok(artifacts_api_key) = dotenv::var("ARTIFACTS_API_KEY") else {
        error!("ARTIFACTS_API_KEY is not set");
        return Err(anyhow::anyhow!("ARTIFACTS_API_KEY is not set"));
    };

    info!("ARTIFACTS_API_KEY is set.");

    let mut cfg = Configuration::new();
    cfg.bearer_access_token = Some(artifacts_api_key);

    let client = ApiClient::new(cfg.into());

    iced::application("Artifacts Client", App::update, App::view)
        .theme(|_| Theme::CatppuccinMocha)
        .run_with(|| (App::new(client), Task::done(Message::Startup)))
        .map_err(|e| anyhow::anyhow!(e))
}

async fn get_account(client: &ApiClient) -> Result<MyAccountDetails> {
    retry(
        || async {
            client
                .my_account_api()
                .get_account_details_my_details_get()
                .await
        },
        3,
        Duration::from_secs(1),
    )
    .await
    .map_or_else(|e| Err(anyhow::anyhow!(e)), |itm| Ok(*itm.data))
}

async fn get_characters(client: &ApiClient) -> Result<Vec<Character>> {
    retry(
        || async {
            client
                .my_characters_api()
                .get_my_characters_my_characters_get()
                .await
        },
        3,
        Duration::from_secs(1),
    )
    .await
    .map(|itm| itm.data.into_iter().map(Character::from_legacy).collect())
    .map_err(|e| anyhow::anyhow!(e))
}

pub struct App {
    client: Arc<Mutex<ApiClient>>,
    account: Option<MyAccountDetails>,
}

#[derive(Debug)]
enum Message {
    Startup,
    FetchAccount(Result<MyAccountDetails>),
    Error(anyhow::Error),
}

impl App {
    fn new(client: ApiClient) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
            account: None,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Startup => {
                if self.account.is_none() {
                    let client = Arc::clone(&self.client);
                    Task::perform(
                        async move {
                            let client = client.lock().await;
                            get_account(&client).await
                        },
                        Message::FetchAccount,
                    )
                } else {
                    Task::none()
                }
            }
            Message::FetchAccount(Ok(account)) => {
                self.account = Some(account);
                info!("Account: {:?}", self.account);
                Task::none()
            }
            Message::FetchAccount(Err(e)) => {
                error!("Error: {:?}", e);
                Task::done(Message::Error(e))
            }
            Message::Error(e) => {
                error!("Error: {:?}", e);
                iced::exit()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = if self.account.is_none() {
            text::Text::new("Loading...")
        } else {
            text::Text::new(format!(
                "Account: {}",
                self.account.as_ref().unwrap().username
            ))
        };
        content.into()
    }
}
