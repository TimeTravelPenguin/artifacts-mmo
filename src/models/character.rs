use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// TODO: Finish adding model fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub gold: u32,
    pub level: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub xp: u32,
    pub max_xp: u32,
    pub x: i32,
    pub y: i32,
    pub cooldown: u32,
    pub cooldown_expiration: DateTime<Utc>,
}
