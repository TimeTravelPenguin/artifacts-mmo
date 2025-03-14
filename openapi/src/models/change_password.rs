/*
 * Artifacts API
 *
 *  Artifacts is an API-based MMO game where you can manage 5 characters to explore, fight, gather resources, craft items and much more.  Website: https://artifactsmmo.com/  Documentation: https://docs.artifactsmmo.com/  OpenAPI Spec: https://api.artifactsmmo.com/openapi.json
 *
 * The version of the OpenAPI document: 4.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangePassword {
    /// Your password.
    #[serde(rename = "current_password")]
    pub current_password: String,
    /// New password.
    #[serde(rename = "new_password")]
    pub new_password: String,
}

impl ChangePassword {
    pub fn new(current_password: String, new_password: String) -> ChangePassword {
        ChangePassword {
            current_password,
            new_password,
        }
    }
}
