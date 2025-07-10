use reqwest::{Response, StatusCode};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::api::client::ArtifactsClient;

/// Artifacts API module that provides functionality to interact with Characters.
pub mod characters;
/// Provides a custom client for the Artifacts API.
pub mod client;
/// Artifacts API module that provides functionality to interact with Maps.
pub mod maps;
/// Artifacts API module that provides functionality to interact with Monsters.
pub mod monsters;
/// Artifacts API module that provides functionality to interact with the user's Characters.
pub mod my_characters;

/// Actions that can be executed against the Artifacts API.
pub trait Action {
    type Return;
    type Error: TryFrom<StatusCode, Error = ()>;

    fn execute(
        &self,
        api: &ArtifactsClient,
    ) -> impl Future<Output = Result<Self::Return, ArtifactsError<Self::Error>>> + Send;
}

/// Actions that can be executed against the Artifacts API with parameters.
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

/// Custom error type for the Artifacts API client.
#[derive(Debug, Error)]
pub enum ArtifactsError<E> {
    /// Represents various errors that can occur while interacting with the Artifacts API.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    /// Represents errors that can occur while serializing or deserializing JSON data.
    #[error("Deserialize error: {0}")]
    Json(#[from] serde_json::Error),
    /// Represents errors that can occur while executing middleware actions.
    #[error("Middleware error: {0}")]
    RetryError(#[from] reqwest_middleware::Error),
    /// Represents an error returned by the Artifacts API.
    #[error("API error: {status} - {error}")]
    ApiError {
        status: reqwest::StatusCode,
        error: E,
    },
    /// Represents an error when the API returns a status code that is not recognized.
    #[error("Unknown API error: {status} - {source}")]
    UnknownStatusError {
        status: reqwest::StatusCode,
        #[source]
        source: anyhow::Error,
    },
    /// A catch-all error for any other errors that may occur.
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// A generic response type for the Artifacts API.
///
/// Typically, when data is returned from the API, it is given in the `data`
/// field of the resulting JSON object. This struct is used to simplify
/// deserialization of such responses.
#[derive(Debug, Deserialize)]
pub struct ArtifactsApiResponse<T> {
    data: T,
}

impl<T: DeserializeOwned> ArtifactsApiResponse<T> {
    /// Parses the JSON response from the API into the specified type `T`.
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
