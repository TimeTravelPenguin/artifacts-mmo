use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

pub trait APIEndpoint {
    type Request: Serialize;
    type Response: for<'de> Deserialize<'de>;

    fn path() -> &'static str;
    fn method() -> Method;
}

pub struct APIClient {
    client: Client,
    base_url: String,
    token: String,
}

impl APIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.artifactsmmo.com".to_string(),
            token: api_key,
        }
    }

    pub async fn request<E: APIEndpoint>(
        &self,
        request: E::Request,
    ) -> Result<E::Response, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, E::path());
        let response = self
            .client
            .request(E::method(), &url)
            .json(&request)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}
