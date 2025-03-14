use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mining {
    #[serde(rename = "mining_level")]
    pub level: i64,
    #[serde(rename = "mining_xp")]
    pub xp: i64,
    #[serde(rename = "mining_max_xp")]
    pub max_xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fishing {
    #[serde(rename = "fishing_level")]
    pub level: i64,
    #[serde(rename = "fishing_xp")]
    pub xp: i64,
    #[serde(rename = "fishing_max_xp")]
    pub max_xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Woodcutting {
    #[serde(rename = "woodcutting_level")]
    pub level: i64,
    #[serde(rename = "woodcutting_xp")]
    pub xp: i64,
    #[serde(rename = "woodcutting_max_xp")]
    pub max_xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weaponcrafting {
    #[serde(rename = "weaponcrafting_level")]
    pub level: i64,
    #[serde(rename = "weaponcrafting_xp")]
    pub xp: i64,
    #[serde(rename = "weaponcrafting_max_xp")]
    pub max_xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gearcrafting {
    #[serde(rename = "gearcrafting_level")]
    pub level: i64,
    #[serde(rename = "gearcrafting_xp")]
    pub xp: i64,
    #[serde(rename = "gearcrafting_max_xp")]
    pub max_xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Jewelrycrafting {
    #[serde(rename = "jewelrycrafting_level")]
    pub level: i64,
    #[serde(rename = "jewelrycrafting_xp")]
    pub xp: i64,
    #[serde(rename = "jewelrycrafting_max_xp")]
    pub max_xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cooking {
    #[serde(rename = "cooking_level")]
    pub level: i64,
    #[serde(rename = "cooking_xp")]
    pub xp: i64,
    #[serde(rename = "cooking_max_xp")]
    pub max_xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alchemy {
    #[serde(rename = "alchemy_level")]
    pub level: i64,
    #[serde(rename = "alchemy_xp")]
    pub xp: i64,
    #[serde(rename = "alchemy_max_xp")]
    pub max_xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub speed: i64,
    pub haste: i64,
    #[serde(rename = "critical_strike")]
    pub critical_strike: i64,
    pub wisdom: i64,
    pub prospecting: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub name: String,
    pub account: String,
    pub skin: String,
    pub level: i64,
    pub xp: i64,
    #[serde(rename = "max_xp")]
    pub max_xp: i64,
    pub gold: i64,
    pub hp: i64,
    #[serde(rename = "max_hp")]
    pub max_hp: i64,
    #[serde(rename = "attack_fire")]
    pub attack_fire: i64,
    #[serde(rename = "attack_earth")]
    pub attack_earth: i64,
    #[serde(rename = "attack_water")]
    pub attack_water: i64,
    #[serde(rename = "attack_air")]
    pub attack_air: i64,
    pub dmg: i64,
    #[serde(rename = "dmg_fire")]
    pub dmg_fire: i64,
    #[serde(rename = "dmg_earth")]
    pub dmg_earth: i64,
    #[serde(rename = "dmg_water")]
    pub dmg_water: i64,
    #[serde(rename = "dmg_air")]
    pub dmg_air: i64,
    #[serde(rename = "res_fire")]
    pub res_fire: i64,
    #[serde(rename = "res_earth")]
    pub res_earth: i64,
    #[serde(rename = "res_water")]
    pub res_water: i64,
    #[serde(rename = "res_air")]
    pub res_air: i64,
    pub x: i64,
    pub y: i64,
    pub cooldown: i64,
    #[serde(rename = "cooldown_expiration")]
    pub cooldown_expiration: String,
    #[serde(rename = "weapon_slot")]
    pub weapon_slot: String,
    #[serde(rename = "rune_slot")]
    pub rune_slot: String,
    #[serde(rename = "shield_slot")]
    pub shield_slot: String,
    #[serde(rename = "helmet_slot")]
    pub helmet_slot: String,
    #[serde(rename = "body_armor_slot")]
    pub body_armor_slot: String,
    #[serde(rename = "leg_armor_slot")]
    pub leg_armor_slot: String,
    #[serde(rename = "boots_slot")]
    pub boots_slot: String,
    #[serde(rename = "ring1_slot")]
    pub ring1_slot: String,
    #[serde(rename = "ring2_slot")]
    pub ring2_slot: String,
    #[serde(rename = "amulet_slot")]
    pub amulet_slot: String,
    #[serde(rename = "artifact1_slot")]
    pub artifact1_slot: String,
    #[serde(rename = "artifact2_slot")]
    pub artifact2_slot: String,
    #[serde(rename = "artifact3_slot")]
    pub artifact3_slot: String,
    #[serde(rename = "utility1_slot")]
    pub utility1_slot: String,
    #[serde(rename = "utility1_slot_quantity")]
    pub utility1_slot_quantity: i64,
    #[serde(rename = "utility2_slot")]
    pub utility2_slot: String,
    #[serde(rename = "utility2_slot_quantity")]
    pub utility2_slot_quantity: i64,
    #[serde(rename = "bag_slot")]
    pub bag_slot: String,
    pub task: String,
    #[serde(rename = "task_type")]
    pub task_type: String,
    #[serde(rename = "task_progress")]
    pub task_progress: i64,
    #[serde(rename = "task_total")]
    pub task_total: i64,
    #[serde(rename = "inventory_max_items")]
    pub inventory_max_items: i64,
    pub inventory: Vec<Inventory>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    pub slot: i64,
    pub code: String,
    pub quantity: i64,
}
