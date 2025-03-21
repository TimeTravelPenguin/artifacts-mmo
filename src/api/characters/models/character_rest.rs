use serde::{Deserialize, Serialize};

use super::{Character, Cooldown};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterRest {
    /// Character details.
    #[serde(rename = "character")]
    pub character: Box<Character>,
    /// The amount of HP restored.
    #[serde(rename = "hp_restored")]
    pub hp_restored: i32,
    /// Cooldown details
    #[serde(rename = "cooldown")]
    pub cooldown: Box<Cooldown>,
}

impl CharacterRest {
    pub fn new(character: Box<Character>, hp_restored: i32, cooldown: Box<Cooldown>) -> Self {
        Self {
            character,
            hp_restored,
            cooldown,
        }
    }
}
