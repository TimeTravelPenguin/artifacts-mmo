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
pub trait MapsApi: Send + Sync {
    /// GET /maps
    ///
    /// Fetch maps details.
    async fn get_all_maps_maps_get<'content_type, 'content_code, 'page, 'size>(
        &self,
        content_type: Option<models::MapContentType>,
        content_code: Option<&'content_code str>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<models::DataPageMapSchema, Error<GetAllMapsMapsGetError>>;

    /// GET /maps/{x}/{y}
    ///
    /// Retrieve the details of a map.
    async fn get_map_maps_xy_get<'x, 'y>(
        &self,
        x: i32,
        y: i32,
    ) -> Result<models::MapResponseSchema, Error<GetMapMapsXyGetError>>;
}

pub struct MapsApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl MapsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

#[async_trait]
impl MapsApi for MapsApiClient {
    /// Fetch maps details.
    async fn get_all_maps_maps_get<'content_type, 'content_code, 'page, 'size>(
        &self,
        content_type: Option<models::MapContentType>,
        content_code: Option<&'content_code str>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<models::DataPageMapSchema, Error<GetAllMapsMapsGetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/maps", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = content_type {
            local_var_req_builder =
                local_var_req_builder.query(&[("content_type", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = content_code {
            local_var_req_builder =
                local_var_req_builder.query(&[("content_code", &local_var_str.to_string())]);
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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DataPageMapSchema`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `models::DataPageMapSchema`")))),
            }
        } else {
            let local_var_entity: Option<GetAllMapsMapsGetError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Retrieve the details of a map.
    async fn get_map_maps_xy_get<'x, 'y>(
        &self,
        x: i32,
        y: i32,
    ) -> Result<models::MapResponseSchema, Error<GetMapMapsXyGetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/maps/{x}/{y}",
            local_var_configuration.base_path,
            x = x,
            y = y
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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MapResponseSchema`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `models::MapResponseSchema`")))),
            }
        } else {
            let local_var_entity: Option<GetMapMapsXyGetError> =
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

/// struct for typed errors of method [`get_all_maps_maps_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllMapsMapsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_map_maps_xy_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMapMapsXyGetError {
    Status404(),
    UnknownValue(serde_json::Value),
}
