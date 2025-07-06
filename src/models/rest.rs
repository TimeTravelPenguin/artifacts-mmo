use serde::{Deserialize, Serialize};

use crate::models::{character::Character, cooldown::Cooldown};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRestData {
    pub cooldown: Cooldown,
    pub hp_restored: u32,
    pub character: Character,
}
