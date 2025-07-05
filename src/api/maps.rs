use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tracing::debug;

use crate::api::{ArtifactsApiResponse, ArtifactsError, client::ArtifactsClient};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MapContentType {
    Monster,
    Resource,
    Workshop,
    Bank,
    GrandExchange,
    TasksMaster,
    Npc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapContent {
    pub content_type: MapContentType,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub name: String,
    pub skin: String,
    pub x: i32,
    pub y: i32,
    #[serde(flatten)]
    pub content: Option<MapContent>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Setters, Serialize, Deserialize)]
#[setters(strip_option)]
pub struct MapQuery {
    pub content_type: Option<MapContentType>,
    pub content_code: Option<String>,
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
}

impl ArtifactsClient {
    pub async fn get_maps(&self, query: &MapQuery) -> Result<Vec<Map>, ArtifactsError> {
        debug!(
            "Fetching maps with query: {}",
            serde_json::to_string(query)
                .unwrap_or_else(|_| "Failed to serialize query".to_string())
        );

        let url = format!("{}/maps", self.base_url);
        let resp = self
            .client
            .get(url)
            .query(&query)
            .bearer_auth(self.api_token.clone())
            .send()
            .await?;

        let char = ArtifactsApiResponse::<Vec<Map>>::parse_json(resp).await?;
        Ok(char)
    }
}
