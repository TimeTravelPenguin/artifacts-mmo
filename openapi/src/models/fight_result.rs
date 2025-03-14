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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FightResult {
    #[serde(rename = "win")]
    Win,
    #[serde(rename = "loss")]
    Loss,
}

impl std::fmt::Display for FightResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Win => write!(f, "win"),
            Self::Loss => write!(f, "loss"),
        }
    }
}

impl Default for FightResult {
    fn default() -> FightResult {
        Self::Win
    }
}
