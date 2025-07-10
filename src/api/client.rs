use std::time::Duration;

use derivative::Derivative;
use reqwest_middleware::{ClientBuilder, Extension};
use reqwest_retry::{Jitter, RetryTransientMiddleware, policies::ExponentialBackoff};
use reqwest_tracing::{OtelName, TracingMiddleware};

/// Custom client for interacting with the Artifacts API.
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct ArtifactsClient {
    pub(crate) client: reqwest_middleware::ClientWithMiddleware,
    pub(crate) user_agent: String,
    pub(crate) base_url: String,
    #[derivative(Debug = "ignore")]
    pub(crate) api_token: String,
}

impl ArtifactsClient {
    pub fn new(api_token: impl Into<String>) -> Self {
        let user_agent = format!("ArtifactsClient/{}", env!("CARGO_PKG_VERSION"));

        let client = reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                );
                headers
            })
            .user_agent(&user_agent)
            .build()
            .expect("Failed to create HTTP client");

        let retry_policy = ExponentialBackoff::builder()
            .retry_bounds(Duration::from_secs(1), Duration::from_secs(60))
            .jitter(Jitter::Bounded)
            .base(2)
            .build_with_total_retry_duration(Duration::from_secs(60));

        let client = ClientBuilder::new(client)
            .with_init(Extension(OtelName("artifacts-client".into())))
            .with(TracingMiddleware::default())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Self {
            client,
            user_agent,
            base_url: "https://api.artifactsmmo.com".to_string(),
            api_token: api_token.into(),
        }
    }
}
