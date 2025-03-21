use std::sync::Arc;

use crate::api::characters::models::Character;
use crate::api::endpoint::APIEndpoint;
use anyhow::Result;
use reqwest::Method;

use super::api_client::Config;

pub(crate) struct CharacterClient {
    pub config: Arc<Config>,
}

impl APIEndpoint for CharacterClient {
    type Request = ();
    type Response = Vec<Character>;

    fn path() -> &'static str {
        "/my/characters"
    }

    fn method() -> Method {
        Method::GET
    }
}

impl CharacterClient {
    async fn get_characters(&self) -> Result<Vec<Character>> {
        let config = self.config.clone();
        let client = &self.config.client;
        let uri = format!("{}/my/characters", config.base_url);

        let request = client.get(&uri).bearer_auth(&config.token).build()?;
        let response = client.execute(request).await?;

        todo!()
    }
}
