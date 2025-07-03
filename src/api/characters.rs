use serde::{Deserialize, Serialize};
use tracing::info;

use crate::api::{ArtifactsApiResponse, ArtifactsError, client::ArtifactsClient};

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub gold: u64,
}

impl ArtifactsClient {
    pub async fn get_character(&self, name: &str) -> Result<Character, ArtifactsError> {
        info!("Fetching character: {}", name);

        let url = format!("{}/characters/{}", self.base_url, name);
        let resp = self
            .client
            .get(url)
            .bearer_auth(self.api_token.clone())
            .send()
            .await?;

        let char = ArtifactsApiResponse::<Character>::parse_json(resp).await?;

        info!("Character fetched successfully: {:?}", char);
        Ok(char)
    }
}
