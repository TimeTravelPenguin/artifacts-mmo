use reqwest::{Response, StatusCode};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::api::client::ArtifactsClient;

pub mod characters;
pub mod client;
pub mod maps;
pub mod monsters;
pub mod my_characters;

pub trait Action {
    type Return;
    type Error: TryFrom<StatusCode, Error = ()>;

    fn execute(
        &self,
        api: &ArtifactsClient,
    ) -> impl Future<Output = Result<Self::Return, ArtifactsError<Self::Error>>> + Send;
}

pub trait ParamatarisedAction {
    type Return;
    type Error: TryFrom<StatusCode, Error = ()>;
    type Param;

    fn execute(
        &self,
        api: &ArtifactsClient,
        character: &Self::Param,
    ) -> impl Future<Output = Result<Self::Return, ArtifactsError<Self::Error>>> + Send;
}

#[derive(Debug, Error)]
pub enum ArtifactsError<E> {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Deserialize error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Middleware error: {0}")]
    RetryError(#[from] reqwest_middleware::Error),
    #[error("API error: {status} - {error}")]
    ApiError {
        status: reqwest::StatusCode,
        error: E,
    },
    #[error("Unknown API error: {status} - {source}")]
    UnknownStatusError {
        status: reqwest::StatusCode,
        #[source]
        source: anyhow::Error,
    },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Deserialize)]
pub struct ArtifactsApiResponse<T> {
    data: T,
}

impl<T: DeserializeOwned> ArtifactsApiResponse<T> {
    async fn parse_json<E>(resp: Response) -> std::result::Result<T, ArtifactsError<E>>
    where
        E: TryFrom<StatusCode, Error = ()>,
    {
        let status = resp.status();
        let text = resp.text().await?;

        if status.is_success() {
            let data: ArtifactsApiResponse<T> = serde_json::from_str(&text)?;
            Ok(data.data)
        } else if let Ok(e) = E::try_from(status) {
            Err(ArtifactsError::ApiError { status, error: e })
        } else {
            Err(ArtifactsError::UnknownStatusError {
                status,
                source: anyhow::anyhow!("Failed to parse response: {}", text),
            })
        }
    }
}
