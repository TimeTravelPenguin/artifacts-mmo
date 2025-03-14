use std::{fmt::Debug, time::Duration};

use tokio::time::sleep;
use tracing::{error, info};

use openapi::{
    apis::{Api, ApiClient, configuration::Configuration},
    models::DestinationSchema,
};

mod api;

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
                    delay *= 2; // Exponential backoff: double the delay after each attempt.
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .pretty()
        .with_writer(non_blocking)
        .init();

    dotenv::dotenv().ok();
    let Ok(artifacts_api_key) = dotenv::var("ARTIFACTS_API_KEY") else {
        error!("ARTIFACTS_API_KEY is not set");
        std::process::exit(1);
    };

    info!("ARTIFACTS_API_KEY is set.");

    let mut cfg = Configuration::new();
    cfg.bearer_access_token = Some(artifacts_api_key);

    let client = ApiClient::new(cfg.into());

    let response = retry(
        || async {
            client
                .my_account_api()
                .get_account_details_my_details_get()
                .await
        },
        3,
        Duration::from_secs(1),
    )
    .await;

    match response {
        Ok(itm) => info!("Account details: {:#?}", itm.data),
        Err(e) => error!("Failed to get account details: {:?}", e),
    }
}
