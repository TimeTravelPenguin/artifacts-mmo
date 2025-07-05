use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::api::client::ArtifactsClient;

pub mod characters;
pub mod client;
pub mod maps;
pub mod monsters;

pub trait Action {
    type Return;

    fn execute(
        &self,
        api: &ArtifactsClient,
    ) -> impl Future<Output = anyhow::Result<Self::Return>> + Send;
}

pub trait QueryAction {
    type Query;
    type Return;

    fn execute(
        &self,
        api: &ArtifactsClient,
        query: Self::Query,
    ) -> impl Future<Output = anyhow::Result<Self::Return>> + Send;
}

#[derive(Debug, Error)]
pub enum ArtifactsError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Deserialize error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Middleware error: {0}")]
    RetryError(#[from] reqwest_middleware::Error),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtifactsApiResponse<T> {
    data: T,
}

impl<T: DeserializeOwned> ArtifactsApiResponse<T> {
    async fn parse_json(resp: Response) -> Result<T, ArtifactsError> {
        let status = resp.status();
        let text = resp.text().await?.to_string();

        if !status.is_success() {
            return Err(ArtifactsError::Unknown(format!(
                "HTTP error {}: {}",
                status, text
            )));
        }

        let data: ArtifactsApiResponse<T> =
            serde_json::from_str(&text).map_err(ArtifactsError::Json)?;

        Ok(data.data)
    }
}
