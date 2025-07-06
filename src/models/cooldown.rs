use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CooldownReason {
    Movement,
    Fight,
    Crafting,
    Gathering,
    BuyGe,
    SellGe,
    BuyNpc,
    SellNpc,
    CancelGe,
    DeleteItem,
    DepositItem,
    WithdrawItem,
    DepositGold,
    WithdrawGold,
    Equip,
    Unequip,
    Task,
    ChristmasExchange,
    Recycling,
    Rest,
    Use,
    BuyBankExpansion,
    GiveItem,
    GiveGold,
    ChangeSkin,
    Rename,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cooldown {
    pub total_seconds: u32,
    pub remaining_seconds: u32,
    pub started_at: DateTime<Utc>,
    pub expiration: DateTime<Utc>,
    reason: CooldownReason,
}
