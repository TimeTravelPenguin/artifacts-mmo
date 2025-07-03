use derivative::Derivative;

#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct ArtifactsClient {
    pub(crate) client: reqwest::Client,
    pub(crate) user_agent: String,
    pub(crate) base_url: String,
    #[derivative(Debug = "ignore")]
    pub(crate) api_token: String,
}

impl ArtifactsClient {
    pub fn new(api_token: impl Into<String>) -> Self {
        let client = reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                );
                headers
            })
            .user_agent(format!("ArtifactsClient/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            user_agent: format!("ArtifactsClient/{}", env!("CARGO_PKG_VERSION")),
            base_url: "https://api.artifactsmmo.com".to_string(),
            api_token: api_token.into(),
        }
    }
}
