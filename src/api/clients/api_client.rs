use reqwest::Client;

use crate::api::endpoint::APIEndpoint;

pub(crate) struct Config {
    pub client: Client,
    pub base_url: String,
    pub token: String,
}

impl Config {
    fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.artifactsmmo.com".to_string(),
            token,
        }
    }
}

pub struct APIClient {
    config: Config,
}

impl APIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            config: Config::new(api_key),
        }
    }

    pub async fn request<E: APIEndpoint>(
        &self,
        request: E::Request,
    ) -> Result<E::Response, reqwest::Error> {
        let url = format!("{}/{}", self.config.base_url, E::path());
        let response = self
            .config
            .client
            .request(E::method(), &url)
            .json(&request)
            .header("Authorization", format!("Bearer {}", self.config.token))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}
