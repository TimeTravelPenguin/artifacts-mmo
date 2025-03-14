/*
 * Artifacts API
 *
 *  Artifacts is an API-based MMO game where you can manage 5 characters to explore, fight, gather resources, craft items and much more.  Website: https://artifactsmmo.com/  Documentation: https://docs.artifactsmmo.com/  OpenAPI Spec: https://api.artifactsmmo.com/openapi.json
 *
 * The version of the OpenAPI document: 4.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::apis::ContentType;
use crate::{apis::ResponseContent, models};
use async_trait::async_trait;
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};
use std::sync::Arc;

#[async_trait]
pub trait LeaderboardApi: Send + Sync {
    /// GET /leaderboard/accounts
    ///
    /// Fetch leaderboard details.
    async fn get_accounts_leaderboard_leaderboard_accounts_get<'sort, 'page, 'size>(
        &self,
        sort: Option<models::AccountLeaderboardType>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<
        models::DataPageAccountLeaderboardSchema,
        Error<GetAccountsLeaderboardLeaderboardAccountsGetError>,
    >;

    /// GET /leaderboard/characters
    ///
    /// Fetch leaderboard details.
    async fn get_characters_leaderboard_leaderboard_characters_get<'sort, 'page, 'size>(
        &self,
        sort: Option<models::CharacterLeaderboardType>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<
        models::DataPageCharacterLeaderboardSchema,
        Error<GetCharactersLeaderboardLeaderboardCharactersGetError>,
    >;
}

pub struct LeaderboardApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl LeaderboardApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

#[async_trait]
impl LeaderboardApi for LeaderboardApiClient {
    /// Fetch leaderboard details.
    async fn get_accounts_leaderboard_leaderboard_accounts_get<'sort, 'page, 'size>(
        &self,
        sort: Option<models::AccountLeaderboardType>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<
        models::DataPageAccountLeaderboardSchema,
        Error<GetAccountsLeaderboardLeaderboardAccountsGetError>,
    > {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str =
            format!("{}/leaderboard/accounts", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = sort {
            local_var_req_builder =
                local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = page {
            local_var_req_builder =
                local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = size {
            local_var_req_builder =
                local_var_req_builder.query(&[("size", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DataPageAccountLeaderboardSchema`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `models::DataPageAccountLeaderboardSchema`")))),
            }
        } else {
            let local_var_entity: Option<GetAccountsLeaderboardLeaderboardAccountsGetError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Fetch leaderboard details.
    async fn get_characters_leaderboard_leaderboard_characters_get<'sort, 'page, 'size>(
        &self,
        sort: Option<models::CharacterLeaderboardType>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<
        models::DataPageCharacterLeaderboardSchema,
        Error<GetCharactersLeaderboardLeaderboardCharactersGetError>,
    > {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/leaderboard/characters",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = sort {
            local_var_req_builder =
                local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = page {
            local_var_req_builder =
                local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = size {
            local_var_req_builder =
                local_var_req_builder.query(&[("size", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DataPageCharacterLeaderboardSchema`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `models::DataPageCharacterLeaderboardSchema`")))),
            }
        } else {
            let local_var_entity: Option<GetCharactersLeaderboardLeaderboardCharactersGetError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }
}

/// struct for typed errors of method [`get_accounts_leaderboard_leaderboard_accounts_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountsLeaderboardLeaderboardAccountsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_characters_leaderboard_leaderboard_characters_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersLeaderboardLeaderboardCharactersGetError {
    UnknownValue(serde_json::Value),
}
