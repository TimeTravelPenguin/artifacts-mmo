use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "movement")]
    Movement,
    #[serde(rename = "fight")]
    Fight,
    #[serde(rename = "crafting")]
    Crafting,
    #[serde(rename = "gathering")]
    Gathering,
    #[serde(rename = "buy_ge")]
    BuyGe,
    #[serde(rename = "sell_ge")]
    SellGe,
    #[serde(rename = "buy_npc")]
    BuyNpc,
    #[serde(rename = "sell_npc")]
    SellNpc,
    #[serde(rename = "cancel_ge")]
    CancelGe,
    #[serde(rename = "delete_item")]
    DeleteItem,
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "withdraw")]
    Withdraw,
    #[serde(rename = "deposit_gold")]
    DepositGold,
    #[serde(rename = "withdraw_gold")]
    WithdrawGold,
    #[serde(rename = "equip")]
    Equip,
    #[serde(rename = "unequip")]
    Unequip,
    #[serde(rename = "task")]
    Task,
    #[serde(rename = "christmas_exchange")]
    ChristmasExchange,
    #[serde(rename = "recycling")]
    Recycling,
    #[serde(rename = "rest")]
    Rest,
    #[serde(rename = "use")]
    Use,
    #[serde(rename = "buy_bank_expansion")]
    BuyBankExpansion,
}
