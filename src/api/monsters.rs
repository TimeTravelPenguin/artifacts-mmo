use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tracing::debug;

use crate::api::{ArtifactsApiResponse, ArtifactsError, client::ArtifactsClient};

#[derive(Debug, Serialize, Deserialize)]
pub struct Monster {
    pub name: String,
    pub code: String,
    pub level: u32,
    pub hp: u32,
}

#[skip_serializing_none]
#[derive(Default, Debug, Setters, Serialize, Deserialize)]
#[setters(strip_option)]
pub struct MonsterQuery {
    pub name: Option<String>,
    pub drop: Option<String>,
    pub min_level: Option<u32>,
    pub max_level: Option<u32>,
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
}

impl ArtifactsClient {
    pub async fn get_monsters(&self, query: &MonsterQuery) -> Result<Vec<Monster>, ArtifactsError> {
        debug!(
            "Fetching monsters with query: {}",
            serde_json::to_string(query)
                .unwrap_or_else(|_| "Failed to serialize query".to_string())
        );

        let url = format!("{}/monsters", self.base_url);
        let resp = self
            .client
            .get(url)
            .query(&query)
            .bearer_auth(self.api_token.clone())
            .send()
            .await?;

        let char = ArtifactsApiResponse::<Vec<Monster>>::parse_json(resp).await?;
        Ok(char)
    }
}
