use serde::{Deserialize, Serialize};

use crate::models::{character::Character, cooldown::Cooldown};

#[derive(Debug, Serialize, Deserialize)]
pub struct Drop {
    pub code: String,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FightResult {
    Win,
    Loss,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fight {
    pub xp: u32,
    pub gold: u32,
    pub drops: Vec<Drop>,
    pub turns: u32,
    pub logs: Vec<String>,
    pub result: FightResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterFightData {
    pub cooldown: Cooldown,
    pub fight: Fight,
    pub character: Character,
}
