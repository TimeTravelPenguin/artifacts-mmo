use serde::{Deserialize, Serialize};

use super::action::Action;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cooldown {
    /// The total seconds of the cooldown.
    #[serde(rename = "total_seconds")]
    pub total_seconds: i32,
    /// The remaining seconds of the cooldown.
    #[serde(rename = "remaining_seconds")]
    pub remaining_seconds: i32,
    /// The start of the cooldown.
    #[serde(rename = "started_at")]
    pub started_at: String,
    /// The expiration of the cooldown.
    #[serde(rename = "expiration")]
    pub expiration: String,
    /// The reason of the cooldown.
    #[serde(rename = "reason")]
    pub reason: Action,
}

impl Cooldown {
    pub fn new(
        total_seconds: i32,
        remaining_seconds: i32,
        started_at: String,
        expiration: String,
        reason: Action,
    ) -> Cooldown {
        Cooldown {
            total_seconds,
            remaining_seconds,
            started_at,
            expiration,
            reason,
        }
    }
}
