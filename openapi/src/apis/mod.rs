use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    ReqwestMiddleware(reqwest_middleware::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::ReqwestMiddleware(e) => ("reqwest-middleware", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::ReqwestMiddleware(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<reqwest_middleware::Error> for Error<T> {
    fn from(e: reqwest_middleware::Error) -> Self {
        Error::ReqwestMiddleware(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            Self::Json
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod accounts_api;
pub mod achievements_api;
pub mod badges_api;
pub mod characters_api;
pub mod default_api;
pub mod effects_api;
pub mod errors;
pub mod events_api;
pub mod grand_exchange_api;
pub mod items_api;
pub mod leaderboard_api;
pub mod maps_api;
pub mod monsters_api;
pub mod my_account_api;
pub mod my_characters_api;
pub mod npcs_api;
pub mod resources_api;
pub mod tasks_api;
pub mod token_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn accounts_api(&self) -> &dyn accounts_api::AccountsApi;
    fn achievements_api(&self) -> &dyn achievements_api::AchievementsApi;
    fn badges_api(&self) -> &dyn badges_api::BadgesApi;
    fn characters_api(&self) -> &dyn characters_api::CharactersApi;
    fn default_api(&self) -> &dyn default_api::DefaultApi;
    fn effects_api(&self) -> &dyn effects_api::EffectsApi;
    fn events_api(&self) -> &dyn events_api::EventsApi;
    fn grand_exchange_api(&self) -> &dyn grand_exchange_api::GrandExchangeApi;
    fn items_api(&self) -> &dyn items_api::ItemsApi;
    fn leaderboard_api(&self) -> &dyn leaderboard_api::LeaderboardApi;
    fn maps_api(&self) -> &dyn maps_api::MapsApi;
    fn monsters_api(&self) -> &dyn monsters_api::MonstersApi;
    fn my_account_api(&self) -> &dyn my_account_api::MyAccountApi;
    fn my_characters_api(&self) -> &dyn my_characters_api::MyCharactersApi;
    fn npcs_api(&self) -> &dyn npcs_api::NpcsApi;
    fn resources_api(&self) -> &dyn resources_api::ResourcesApi;
    fn tasks_api(&self) -> &dyn tasks_api::TasksApi;
    fn token_api(&self) -> &dyn token_api::TokenApi;
}

pub struct ApiClient {
    accounts_api: Box<dyn accounts_api::AccountsApi>,
    achievements_api: Box<dyn achievements_api::AchievementsApi>,
    badges_api: Box<dyn badges_api::BadgesApi>,
    characters_api: Box<dyn characters_api::CharactersApi>,
    default_api: Box<dyn default_api::DefaultApi>,
    effects_api: Box<dyn effects_api::EffectsApi>,
    events_api: Box<dyn events_api::EventsApi>,
    grand_exchange_api: Box<dyn grand_exchange_api::GrandExchangeApi>,
    items_api: Box<dyn items_api::ItemsApi>,
    leaderboard_api: Box<dyn leaderboard_api::LeaderboardApi>,
    maps_api: Box<dyn maps_api::MapsApi>,
    monsters_api: Box<dyn monsters_api::MonstersApi>,
    my_account_api: Box<dyn my_account_api::MyAccountApi>,
    my_characters_api: Box<dyn my_characters_api::MyCharactersApi>,
    npcs_api: Box<dyn npcs_api::NpcsApi>,
    resources_api: Box<dyn resources_api::ResourcesApi>,
    tasks_api: Box<dyn tasks_api::TasksApi>,
    token_api: Box<dyn token_api::TokenApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            accounts_api: Box::new(accounts_api::AccountsApiClient::new(configuration.clone())),
            achievements_api: Box::new(achievements_api::AchievementsApiClient::new(
                configuration.clone(),
            )),
            badges_api: Box::new(badges_api::BadgesApiClient::new(configuration.clone())),
            characters_api: Box::new(characters_api::CharactersApiClient::new(
                configuration.clone(),
            )),
            default_api: Box::new(default_api::DefaultApiClient::new(configuration.clone())),
            effects_api: Box::new(effects_api::EffectsApiClient::new(configuration.clone())),
            events_api: Box::new(events_api::EventsApiClient::new(configuration.clone())),
            grand_exchange_api: Box::new(grand_exchange_api::GrandExchangeApiClient::new(
                configuration.clone(),
            )),
            items_api: Box::new(items_api::ItemsApiClient::new(configuration.clone())),
            leaderboard_api: Box::new(leaderboard_api::LeaderboardApiClient::new(
                configuration.clone(),
            )),
            maps_api: Box::new(maps_api::MapsApiClient::new(configuration.clone())),
            monsters_api: Box::new(monsters_api::MonstersApiClient::new(configuration.clone())),
            my_account_api: Box::new(my_account_api::MyAccountApiClient::new(
                configuration.clone(),
            )),
            my_characters_api: Box::new(my_characters_api::MyCharactersApiClient::new(
                configuration.clone(),
            )),
            npcs_api: Box::new(npcs_api::NpcsApiClient::new(configuration.clone())),
            resources_api: Box::new(resources_api::ResourcesApiClient::new(
                configuration.clone(),
            )),
            tasks_api: Box::new(tasks_api::TasksApiClient::new(configuration.clone())),
            token_api: Box::new(token_api::TokenApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn accounts_api(&self) -> &dyn accounts_api::AccountsApi {
        self.accounts_api.as_ref()
    }
    fn achievements_api(&self) -> &dyn achievements_api::AchievementsApi {
        self.achievements_api.as_ref()
    }
    fn badges_api(&self) -> &dyn badges_api::BadgesApi {
        self.badges_api.as_ref()
    }
    fn characters_api(&self) -> &dyn characters_api::CharactersApi {
        self.characters_api.as_ref()
    }
    fn default_api(&self) -> &dyn default_api::DefaultApi {
        self.default_api.as_ref()
    }
    fn effects_api(&self) -> &dyn effects_api::EffectsApi {
        self.effects_api.as_ref()
    }
    fn events_api(&self) -> &dyn events_api::EventsApi {
        self.events_api.as_ref()
    }
    fn grand_exchange_api(&self) -> &dyn grand_exchange_api::GrandExchangeApi {
        self.grand_exchange_api.as_ref()
    }
    fn items_api(&self) -> &dyn items_api::ItemsApi {
        self.items_api.as_ref()
    }
    fn leaderboard_api(&self) -> &dyn leaderboard_api::LeaderboardApi {
        self.leaderboard_api.as_ref()
    }
    fn maps_api(&self) -> &dyn maps_api::MapsApi {
        self.maps_api.as_ref()
    }
    fn monsters_api(&self) -> &dyn monsters_api::MonstersApi {
        self.monsters_api.as_ref()
    }
    fn my_account_api(&self) -> &dyn my_account_api::MyAccountApi {
        self.my_account_api.as_ref()
    }
    fn my_characters_api(&self) -> &dyn my_characters_api::MyCharactersApi {
        self.my_characters_api.as_ref()
    }
    fn npcs_api(&self) -> &dyn npcs_api::NpcsApi {
        self.npcs_api.as_ref()
    }
    fn resources_api(&self) -> &dyn resources_api::ResourcesApi {
        self.resources_api.as_ref()
    }
    fn tasks_api(&self) -> &dyn tasks_api::TasksApi {
        self.tasks_api.as_ref()
    }
    fn token_api(&self) -> &dyn token_api::TokenApi {
        self.token_api.as_ref()
    }
}
