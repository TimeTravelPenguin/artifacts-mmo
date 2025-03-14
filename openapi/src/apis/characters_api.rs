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
pub trait CharactersApi: Send + Sync {
    /// POST /characters/create
    ///
    /// Create new character on your account. You can create up to 5 characters.
    async fn create_character_characters_create_post<'add_character_schema>(
        &self,
        add_character_schema: models::AddCharacterSchema,
    ) -> Result<models::CharacterResponseSchema, Error<CreateCharacterCharactersCreatePostError>>;

    /// POST /characters/delete
    ///
    /// Delete character on your account.
    async fn delete_character_characters_delete_post<'delete_character_schema>(
        &self,
        delete_character_schema: models::DeleteCharacterSchema,
    ) -> Result<models::CharacterResponseSchema, Error<DeleteCharacterCharactersDeletePostError>>;

    /// GET /characters/{name}
    ///
    /// Retrieve the details of a character.
    async fn get_character_characters_name_get<'name>(
        &self,
        name: &'name str,
    ) -> Result<models::CharacterResponseSchema, Error<GetCharacterCharactersNameGetError>>;
}

pub struct CharactersApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl CharactersApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

#[async_trait]
impl CharactersApi for CharactersApiClient {
    /// Create new character on your account. You can create up to 5 characters.
    async fn create_character_characters_create_post<'add_character_schema>(
        &self,
        add_character_schema: models::AddCharacterSchema,
    ) -> Result<models::CharacterResponseSchema, Error<CreateCharacterCharactersCreatePostError>>
    {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/characters/create", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        local_var_req_builder = local_var_req_builder.json(&add_character_schema);

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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CharacterResponseSchema`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `models::CharacterResponseSchema`")))),
            }
        } else {
            let local_var_entity: Option<CreateCharacterCharactersCreatePostError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Delete character on your account.
    async fn delete_character_characters_delete_post<'delete_character_schema>(
        &self,
        delete_character_schema: models::DeleteCharacterSchema,
    ) -> Result<models::CharacterResponseSchema, Error<DeleteCharacterCharactersDeletePostError>>
    {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/characters/delete", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        local_var_req_builder = local_var_req_builder.json(&delete_character_schema);

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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CharacterResponseSchema`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `models::CharacterResponseSchema`")))),
            }
        } else {
            let local_var_entity: Option<DeleteCharacterCharactersDeletePostError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Retrieve the details of a character.
    async fn get_character_characters_name_get<'name>(
        &self,
        name: &'name str,
    ) -> Result<models::CharacterResponseSchema, Error<GetCharacterCharactersNameGetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/characters/{name}",
            local_var_configuration.base_path,
            name = crate::apis::urlencode(name)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CharacterResponseSchema`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `models::CharacterResponseSchema`")))),
            }
        } else {
            let local_var_entity: Option<GetCharacterCharactersNameGetError> =
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

/// struct for typed errors of method [`create_character_characters_create_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCharacterCharactersCreatePostError {
    Status494(),
    Status495(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_character_characters_delete_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCharacterCharactersDeletePostError {
    Status498(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_character_characters_name_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharacterCharactersNameGetError {
    Status404(),
    UnknownValue(serde_json::Value),
}
