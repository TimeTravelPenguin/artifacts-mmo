use serde::{Deserialize, Serialize};

use crate::models::{character::Character, cooldown::Cooldown, map::Map};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMovementData {
    pub cooldown: Cooldown,
    pub character: Character,
    pub destination: Map,
}
