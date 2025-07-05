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
        Ok(char)
    }

    pub async fn delete_character(&self, name: &str) -> Result<Character, ArtifactsError> {
        info!("Deleting character: {}", name);

        let body = serde_json::json!({
            "name": name,
        });

        let url = format!("{}/characters/delete", self.base_url);
        let resp = self
            .client
            .post(url)
            .json(&body)
            .bearer_auth(self.api_token.clone())
            .send()
            .await?;

        let char = ArtifactsApiResponse::<Character>::parse_json(resp).await?;
        Ok(char)
    }
}
