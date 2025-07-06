use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tracing::debug;

use crate::{
    api::{ArtifactsApiResponse, ArtifactsError, client::ArtifactsClient},
    make_error,
    models::map::{Map, MapContentType},
};

make_error!(GetAllMapsError);

make_error!(GetMapError,
    404 => MapNotFound
        => "Map not found",
);

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
    pub async fn get_maps(
        &self,
        query: &MapQuery,
    ) -> Result<Vec<Map>, ArtifactsError<GetAllMapsError>> {
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

    pub async fn get_map(&self, x: i32, y: i32) -> Result<Map, ArtifactsError<GetMapError>> {
        debug!("Fetching map at coordinates: ({}, {})", x, y);

        let url = format!("{}/maps/{}/{}", self.base_url, x, y);
        let resp = self
            .client
            .get(url)
            .bearer_auth(self.api_token.clone())
            .send()
            .await?;

        let map = ArtifactsApiResponse::<Map>::parse_json(resp).await?;
        Ok(map)
    }
}
