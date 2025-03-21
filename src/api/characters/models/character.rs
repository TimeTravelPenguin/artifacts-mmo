use serde::{Deserialize, Serialize};

use crate::api::compatibility::FromLegacy;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Vec<Character>,
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
    pub hp: i64,
    #[serde(rename = "max_hp")]
    pub max_hp: i64,
    pub speed: i64,
    pub haste: i64,
    #[serde(rename = "critical_strike")]
    pub critical_strike: i64,
    pub wisdom: i64,
    pub prospecting: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attack {
    #[serde(rename = "attack_fire")]
    pub fire: i64,
    #[serde(rename = "attack_earth")]
    pub earth: i64,
    #[serde(rename = "attack_water")]
    pub water: i64,
    #[serde(rename = "attack_air")]
    pub air: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    pub dmg: i64,
    #[serde(rename = "dmg_fire")]
    pub fire: i64,
    #[serde(rename = "dmg_earth")]
    pub earth: i64,
    #[serde(rename = "dmg_water")]
    pub water: i64,
    #[serde(rename = "dmg_air")]
    pub air: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resistance {
    #[serde(rename = "res_fire")]
    pub fire: i64,
    #[serde(rename = "res_earth")]
    pub earth: i64,
    #[serde(rename = "res_water")]
    pub water: i64,
    #[serde(rename = "res_air")]
    pub air: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Position {
        Position { x, y }
    }
}

impl From<(i64, i64)> for Position {
    fn from((x, y): (i64, i64)) -> Self {
        Position { x, y }
    }
}

impl From<Position> for (i64, i64) {
    fn from(position: Position) -> Self {
        (position.x, position.y)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slots {
    #[serde(rename = "weapon_slot")]
    pub weapon: String,
    #[serde(rename = "rune_slot")]
    pub rune: String,
    #[serde(rename = "shield_slot")]
    pub shield: String,
    #[serde(rename = "helmet_slot")]
    pub helmet: String,
    #[serde(rename = "body_armor_slot")]
    pub body_armor: String,
    #[serde(rename = "leg_armor_slot")]
    pub leg_armor: String,
    #[serde(rename = "boots_slot")]
    pub boots: String,
    #[serde(rename = "ring1_slot")]
    pub ring1: String,
    #[serde(rename = "ring2_slot")]
    pub ring2: String,
    #[serde(rename = "amulet_slot")]
    pub amulet: String,
    #[serde(rename = "artifact1_slot")]
    pub artifact1: String,
    #[serde(rename = "artifact2_slot")]
    pub artifact2: String,
    #[serde(rename = "artifact3_slot")]
    pub artifact3: String,
    #[serde(rename = "utility1_slot")]
    pub utility1: String,
    #[serde(rename = "utility1_slot_quantity")]
    pub utility1_quantity: i64,
    #[serde(rename = "utility2_slot")]
    pub utility2: String,
    #[serde(rename = "utility2_slot_quantity")]
    pub utility2_quantity: i64,
    #[serde(rename = "bag_slot")]
    pub bag: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skills {
    #[serde(flatten)]
    pub mining: Mining,
    #[serde(flatten)]
    pub fishing: Fishing,
    #[serde(flatten)]
    pub woodcutting: Woodcutting,
    #[serde(flatten)]
    pub weaponcrafting: Weaponcrafting,
    #[serde(flatten)]
    pub gearcrafting: Gearcrafting,
    #[serde(flatten)]
    pub jewelrycrafting: Jewelrycrafting,
    #[serde(flatten)]
    pub cooking: Cooking,
    #[serde(flatten)]
    pub alchemy: Alchemy,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub task: String,
    #[serde(rename = "task_type")]
    pub task_type: String,
    #[serde(rename = "task_progress")]
    pub task_progress: i64,
    #[serde(rename = "task_total")]
    pub task_total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    pub slot: i64,
    pub code: String,
    pub quantity: i64,
}

impl FromLegacy for Inventory {
    type LegacyType = openapi::models::InventorySlot;

    fn from_legacy(legacy: Self::LegacyType) -> Self {
        Inventory {
            slot: legacy.slot.into(),
            code: legacy.code.to_string(),
            quantity: legacy.quantity.into(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub name: String,
    pub account: String,
    pub skin: String,
    pub level: i64,
    pub xp: i64,
    #[serde(rename = "max_xp")]
    pub max_xp: i64,
    pub gold: i64,
    #[serde(flatten)]
    pub position: Position,
    #[serde(flatten)]
    pub stats: Stats,
    #[serde(flatten)]
    pub attack: Attack,
    #[serde(flatten)]
    pub defense: Damage,
    #[serde(flatten)]
    pub resistance: Resistance,
    #[serde(flatten)]
    pub skills: Skills,
    pub cooldown: i64,
    #[serde(rename = "cooldown_expiration")]
    pub cooldown_expiration: Option<String>,
    #[serde(flatten)]
    pub slots: Slots,
    #[serde(flatten)]
    pub task: Task,
    #[serde(rename = "inventory_max_items")]
    pub inventory_max_items: i64,
    pub inventory: Vec<Inventory>,
}

impl FromLegacy for Character {
    type LegacyType = openapi::models::CharacterSchema;

    fn from_legacy(legacy: Self::LegacyType) -> Self {
        Character {
            name: legacy.name,
            account: legacy.account,
            skin: legacy.skin.to_string(),
            level: legacy.level.into(),
            xp: legacy.xp.into(),
            max_xp: legacy.max_xp.into(),
            gold: legacy.gold.into(),
            position: Position::new(legacy.x.into(), legacy.y.into()),
            stats: Stats {
                hp: legacy.hp.into(),
                max_hp: legacy.max_hp.into(),
                speed: legacy.speed.into(),
                haste: legacy.haste.into(),
                critical_strike: legacy.critical_strike.into(),
                wisdom: legacy.wisdom.into(),
                prospecting: legacy.prospecting.into(),
            },
            attack: Attack {
                fire: legacy.attack_fire.into(),
                earth: legacy.attack_earth.into(),
                water: legacy.attack_water.into(),
                air: legacy.attack_air.into(),
            },
            defense: Damage {
                dmg: legacy.dmg.into(),
                fire: legacy.dmg_fire.into(),
                earth: legacy.dmg_earth.into(),
                water: legacy.dmg_water.into(),
                air: legacy.dmg_air.into(),
            },
            resistance: Resistance {
                fire: legacy.res_fire.into(),
                earth: legacy.res_earth.into(),
                water: legacy.res_water.into(),
                air: legacy.res_air.into(),
            },
            skills: Skills {
                mining: Mining {
                    level: legacy.mining_level.into(),
                    xp: legacy.mining_xp.into(),
                    max_xp: legacy.mining_max_xp.into(),
                },
                fishing: Fishing {
                    level: legacy.fishing_level.into(),
                    xp: legacy.fishing_xp.into(),
                    max_xp: legacy.fishing_max_xp.into(),
                },
                woodcutting: Woodcutting {
                    level: legacy.woodcutting_level.into(),
                    xp: legacy.woodcutting_xp.into(),
                    max_xp: legacy.woodcutting_max_xp.into(),
                },
                weaponcrafting: Weaponcrafting {
                    level: legacy.weaponcrafting_level.into(),
                    xp: legacy.weaponcrafting_xp.into(),
                    max_xp: legacy.weaponcrafting_max_xp.into(),
                },
                gearcrafting: Gearcrafting {
                    level: legacy.gearcrafting_level.into(),
                    xp: legacy.gearcrafting_xp.into(),
                    max_xp: legacy.gearcrafting_max_xp.into(),
                },
                jewelrycrafting: Jewelrycrafting {
                    level: legacy.jewelrycrafting_level.into(),
                    xp: legacy.jewelrycrafting_xp.into(),
                    max_xp: legacy.jewelrycrafting_max_xp.into(),
                },
                cooking: Cooking {
                    level: legacy.cooking_level.into(),
                    xp: legacy.cooking_xp.into(),
                    max_xp: legacy.cooking_max_xp.into(),
                },
                alchemy: Alchemy {
                    level: legacy.alchemy_level.into(),
                    xp: legacy.alchemy_xp.into(),
                    max_xp: legacy.alchemy_max_xp.into(),
                },
            },
            cooldown: legacy.cooldown.into(),
            cooldown_expiration: legacy.cooldown_expiration.clone(),
            slots: Slots {
                weapon: legacy.weapon_slot.to_string(),
                rune: legacy.rune_slot.to_string(),
                shield: legacy.shield_slot.to_string(),
                helmet: legacy.helmet_slot.to_string(),
                body_armor: legacy.body_armor_slot.to_string(),
                leg_armor: legacy.leg_armor_slot.to_string(),
                boots: legacy.boots_slot.to_string(),
                ring1: legacy.ring1_slot.to_string(),
                ring2: legacy.ring2_slot.to_string(),
                amulet: legacy.amulet_slot.to_string(),
                artifact1: legacy.artifact1_slot.to_string(),
                artifact2: legacy.artifact2_slot.to_string(),
                artifact3: legacy.artifact3_slot.to_string(),
                utility1: legacy.utility1_slot.to_string(),
                utility1_quantity: legacy.utility1_slot_quantity.into(),
                utility2: legacy.utility2_slot.to_string(),
                utility2_quantity: legacy.utility2_slot_quantity.into(),
                bag: legacy.bag_slot.to_string(),
            },
            task: Task {
                task: legacy.task.to_string(),
                task_type: legacy.task_type.to_string(),
                task_progress: legacy.task_progress.into(),
                task_total: legacy.task_total.into(),
            },
            inventory_max_items: legacy.inventory_max_items.into(),
            inventory: legacy
                .inventory
                .unwrap_or_default()
                .into_iter()
                .map(Inventory::from_legacy)
                .collect(),
        }
    }
}
