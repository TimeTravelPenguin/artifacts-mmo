use thiserror::Error;

/// General error codes
#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum GeneralError {
    #[error("[Error 422] Invalid payload: {0}")]
    InvalidPayload(String),
    #[error("[Error 429] Too many requests: {0}")]
    TooManyRequests(String),
    #[error("[Error 404] Not found: {0}")]
    NotFound(String),
    #[error("[Error 500] Fatal error: {0}")]
    FatalError(String),
}

impl GeneralError {
    /// Returns the error code associated with the error.
    pub fn code(&self) -> u16 {
        match self {
            GeneralError::InvalidPayload(_) => 422,
            GeneralError::TooManyRequests(_) => 429,
            GeneralError::NotFound(_) => 404,
            GeneralError::FatalError(_) => 500,
        }
    }

    /// Returns the error message associated with the error.
    pub fn message(&self) -> String {
        match self {
            GeneralError::InvalidPayload(msg) => msg.clone(),
            GeneralError::TooManyRequests(msg) => msg.clone(),
            GeneralError::NotFound(msg) => msg.clone(),
            GeneralError::FatalError(msg) => msg.clone(),
        }
    }
}

/// Account related errors
#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum AccountError {
    #[error("[Error 452] Token invalid: {0}")]
    TokenInvalid(String),
    #[error("[Error 453] Token expired: {0}")]
    TokenExpired(String),
    #[error("[Error 454] Token missing: {0}")]
    TokenMissing(String),
    #[error("[Error 455] Token generation failed: {0}")]
    TokenGenerationFail(String),
    #[error("[Error 456] Username already used: {0}")]
    UsernameAlreadyUsed(String),
    #[error("[Error 457] Email already used: {0}")]
    EmailAlreadyUsed(String),
    #[error("[Error 458] Same password: {0}")]
    SamePassword(String),
    #[error("[Error 459] Current password invalid: {0}")]
    CurrentPasswordInvalid(String),
}

impl AccountError {
    /// Returns the error code associated with the error.
    pub fn code(&self) -> u16 {
        match self {
            AccountError::TokenInvalid(_) => 452,
            AccountError::TokenExpired(_) => 453,
            AccountError::TokenMissing(_) => 454,
            AccountError::TokenGenerationFail(_) => 455,
            AccountError::UsernameAlreadyUsed(_) => 456,
            AccountError::EmailAlreadyUsed(_) => 457,
            AccountError::SamePassword(_) => 458,
            AccountError::CurrentPasswordInvalid(_) => 459,
        }
    }

    /// Returns the error message associated with the error.
    pub fn message(&self) -> String {
        match self {
            AccountError::TokenInvalid(msg) => msg.clone(),
            AccountError::TokenExpired(msg) => msg.clone(),
            AccountError::TokenMissing(msg) => msg.clone(),
            AccountError::TokenGenerationFail(msg) => msg.clone(),
            AccountError::UsernameAlreadyUsed(msg) => msg.clone(),
            AccountError::EmailAlreadyUsed(msg) => msg.clone(),
            AccountError::SamePassword(msg) => msg.clone(),
            AccountError::CurrentPasswordInvalid(msg) => msg.clone(),
        }
    }
}

/// Character related errors
#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum CharacterError {
    #[error("[Error 483] Not enough HP: {0}")]
    NotEnoughHp(String),
    #[error("[Error 484] Maximum utilities equipped: {0}")]
    MaximumUtilitiesEquipped(String),
    #[error("[Error 485] Item already equipped: {0}")]
    ItemAlreadyEquipped(String),
    #[error("[Error 486] Character locked: {0}")]
    Locked(String),
    #[error("[Error 474] Not this task: {0}")]
    NotThisTask(String),
    #[error("[Error 475] Too many items for task: {0}")]
    TooManyItemsTask(String),
    #[error("[Error 487] No task: {0}")]
    NoTask(String),
    #[error("[Error 488] Task not completed: {0}")]
    TaskNotCompleted(String),
    #[error("[Error 489] Already has a task: {0}")]
    AlreadyTask(String),
    #[error("[Error 490] Already on map: {0}")]
    AlreadyMap(String),
    #[error("[Error 491] Slot equipment error: {0}")]
    SlotEquipmentError(String),
    #[error("[Error 492] Gold insufficient: {0}")]
    GoldInsufficient(String),
    #[error("[Error 493] Skill level not sufficient: {0}")]
    NotSkillLevelRequired(String),
    #[error("[Error 494] Name already used: {0}")]
    NameAlreadyUsed(String),
    #[error("[Error 495] Maximum characters reached: {0}")]
    MaxCharactersReached(String),
    #[error("[Error 496] Level not sufficient: {0}")]
    NotLevelRequired(String),
    #[error("[Error 497] Inventory full: {0}")]
    InventoryFull(String),
    #[error("[Error 498] Character not found: {0}")]
    NotFound(String),
    #[error("[Error 499] In cooldown: {0}")]
    InCooldown(String),
}

impl CharacterError {
    /// Returns the error code associated with the error.
    pub fn code(&self) -> u16 {
        match self {
            CharacterError::NotEnoughHp(_) => 483,
            CharacterError::MaximumUtilitiesEquipped(_) => 484,
            CharacterError::ItemAlreadyEquipped(_) => 485,
            CharacterError::Locked(_) => 486,
            CharacterError::NotThisTask(_) => 474,
            CharacterError::TooManyItemsTask(_) => 475,
            CharacterError::NoTask(_) => 487,
            CharacterError::TaskNotCompleted(_) => 488,
            CharacterError::AlreadyTask(_) => 489,
            CharacterError::AlreadyMap(_) => 490,
            CharacterError::SlotEquipmentError(_) => 491,
            CharacterError::GoldInsufficient(_) => 492,
            CharacterError::NotSkillLevelRequired(_) => 493,
            CharacterError::NameAlreadyUsed(_) => 494,
            CharacterError::MaxCharactersReached(_) => 495,
            CharacterError::NotLevelRequired(_) => 496,
            CharacterError::InventoryFull(_) => 497,
            CharacterError::NotFound(_) => 498,
            CharacterError::InCooldown(_) => 499,
        }
    }

    /// Returns the error message associated with the error.
    pub fn message(&self) -> String {
        match self {
            CharacterError::NotEnoughHp(msg) => msg.clone(),
            CharacterError::MaximumUtilitiesEquipped(msg) => msg.clone(),
            CharacterError::ItemAlreadyEquipped(msg) => msg.clone(),
            CharacterError::Locked(msg) => msg.clone(),
            CharacterError::NotThisTask(msg) => msg.clone(),
            CharacterError::TooManyItemsTask(msg) => msg.clone(),
            CharacterError::NoTask(msg) => msg.clone(),
            CharacterError::TaskNotCompleted(msg) => msg.clone(),
            CharacterError::AlreadyTask(msg) => msg.clone(),
            CharacterError::AlreadyMap(msg) => msg.clone(),
            CharacterError::SlotEquipmentError(msg) => msg.clone(),
            CharacterError::GoldInsufficient(msg) => msg.clone(),
            CharacterError::NotSkillLevelRequired(msg) => msg.clone(),
            CharacterError::NameAlreadyUsed(msg) => msg.clone(),
            CharacterError::MaxCharactersReached(msg) => msg.clone(),
            CharacterError::NotLevelRequired(msg) => msg.clone(),
            CharacterError::InventoryFull(msg) => msg.clone(),
            CharacterError::NotFound(msg) => msg.clone(),
            CharacterError::InCooldown(msg) => msg.clone(),
        }
    }
}

/// Item related errors
#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum ItemError {
    #[error("[Error 471] Insufficient quantity: {0}")]
    InsufficientQuantity(String),
    #[error("[Error 472] Invalid equipment: {0}")]
    InvalidEquipment(String),
    #[error("[Error 473] Recycling invalid item: {0}")]
    RecyclingInvalidItem(String),
    #[error("[Error 476] Invalid consumable: {0}")]
    InvalidConsumable(String),
    #[error("[Error 478] Missing item: {0}")]
    MissingItem(String),
}

impl ItemError {
    /// Returns the error code associated with the error.
    pub fn code(&self) -> u16 {
        match self {
            ItemError::InsufficientQuantity(_) => 471,
            ItemError::InvalidEquipment(_) => 472,
            ItemError::RecyclingInvalidItem(_) => 473,
            ItemError::InvalidConsumable(_) => 476,
            ItemError::MissingItem(_) => 478,
        }
    }

    /// Returns the error message associated with the error.
    pub fn message(&self) -> String {
        match self {
            ItemError::InsufficientQuantity(msg) => msg.clone(),
            ItemError::InvalidEquipment(msg) => msg.clone(),
            ItemError::RecyclingInvalidItem(msg) => msg.clone(),
            ItemError::InvalidConsumable(msg) => msg.clone(),
            ItemError::MissingItem(msg) => msg.clone(),
        }
    }
}

/// Grand Exchange related errors
#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum GrandExchangeError {
    #[error("[Error 479] Max quantity exceeded: {0}")]
    MaxQuantity(String),
    #[error("[Error 480] Item not in stock: {0}")]
    NotInStock(String),
    #[error("[Error 482] Price mismatch: {0}")]
    NotThePrice(String),
    #[error("[Error 436] Transaction in progress: {0}")]
    TransactionInProgress(String),
    #[error("[Error 431] No orders: {0}")]
    NoOrders(String),
    #[error("[Error 433] Max orders reached: {0}")]
    MaxOrders(String),
    #[error("[Error 434] Too many items: {0}")]
    TooManyItems(String),
    #[error("[Error 435] Same account: {0}")]
    SameAccount(String),
    #[error("[Error 437] Invalid item: {0}")]
    InvalidItem(String),
    #[error("[Error 438] Not your order: {0}")]
    NotYourOrder(String),
}

impl GrandExchangeError {
    /// Returns the error code associated with the error.
    pub fn code(&self) -> u16 {
        match self {
            GrandExchangeError::MaxQuantity(_) => 479,
            GrandExchangeError::NotInStock(_) => 480,
            GrandExchangeError::NotThePrice(_) => 482,
            GrandExchangeError::TransactionInProgress(_) => 436,
            GrandExchangeError::NoOrders(_) => 431,
            GrandExchangeError::MaxOrders(_) => 433,
            GrandExchangeError::TooManyItems(_) => 434,
            GrandExchangeError::SameAccount(_) => 435,
            GrandExchangeError::InvalidItem(_) => 437,
            GrandExchangeError::NotYourOrder(_) => 438,
        }
    }

    /// Returns the error message associated with the error.
    pub fn message(&self) -> String {
        match self {
            GrandExchangeError::MaxQuantity(msg) => msg.clone(),
            GrandExchangeError::NotInStock(msg) => msg.clone(),
            GrandExchangeError::NotThePrice(msg) => msg.clone(),
            GrandExchangeError::TransactionInProgress(msg) => msg.clone(),
            GrandExchangeError::NoOrders(msg) => msg.clone(),
            GrandExchangeError::MaxOrders(msg) => msg.clone(),
            GrandExchangeError::TooManyItems(msg) => msg.clone(),
            GrandExchangeError::SameAccount(msg) => msg.clone(),
            GrandExchangeError::InvalidItem(msg) => msg.clone(),
            GrandExchangeError::NotYourOrder(msg) => msg.clone(),
        }
    }
}

/// Bank related errors
#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum BankError {
    #[error("[Error 460] Insufficient gold: {0}")]
    InsufficientGold(String),
    #[error("[Error 461] Transaction in progress: {0}")]
    TransactionInProgress(String),
    #[error("[Error 462] Bank full: {0}")]
    Full(String),
}

impl BankError {
    /// Returns the error code associated with the error.
    pub fn code(&self) -> u16 {
        match self {
            BankError::InsufficientGold(_) => 460,
            BankError::TransactionInProgress(_) => 461,
            BankError::Full(_) => 462,
        }
    }

    /// Returns the error message associated with the error.
    pub fn message(&self) -> String {
        match self {
            BankError::InsufficientGold(msg) => msg.clone(),
            BankError::TransactionInProgress(msg) => msg.clone(),
            BankError::Full(msg) => msg.clone(),
        }
    }
}

/// Map related errors
#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum MapError {
    #[error("[Error 597] Map not found: {0}")]
    NotFound(String),
    #[error("[Error 598] Map content not found: {0}")]
    ContentNotFound(String),
}

impl MapError {
    /// Returns the error code associated with the error.
    pub fn code(&self) -> u16 {
        match self {
            MapError::NotFound(_) => 597,
            MapError::ContentNotFound(_) => 598,
        }
    }

    /// Returns the error message associated with the error.
    pub fn message(&self) -> String {
        match self {
            MapError::NotFound(msg) => msg.clone(),
            MapError::ContentNotFound(msg) => msg.clone(),
        }
    }
}

/// An overall API error type which can wrap any of the domain-specific errors
#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum ApiError {
    #[error("General error: {0}")]
    General(#[from] GeneralError),
    #[error("Account error: {0}")]
    Account(#[from] AccountError),
    #[error("Character error: {0}")]
    Character(#[from] CharacterError),
    #[error("Item error: {0}")]
    Item(#[from] ItemError),
    #[error("Grand Exchange error: {0}")]
    GrandExchange(#[from] GrandExchangeError),
    #[error("Bank error: {0}")]
    Bank(#[from] BankError),
    #[error("Map error: {0}")]
    Map(#[from] MapError),
    #[error("Unknown error: {code} - {message}")]
    UnknownError { code: u16, message: String },
}

impl ApiError {
    /// Returns the error code associated with the error.
    pub fn code(&self) -> u16 {
        match self {
            ApiError::General(e) => e.code(),
            ApiError::Account(e) => e.code(),
            ApiError::Character(e) => e.code(),
            ApiError::Item(e) => e.code(),
            ApiError::GrandExchange(e) => e.code(),
            ApiError::Bank(e) => e.code(),
            ApiError::Map(e) => e.code(),
            ApiError::UnknownError { code, .. } => *code,
        }
    }

    /// Returns the error message associated with the error.
    pub fn message(&self) -> String {
        match self {
            ApiError::General(e) => e.to_string(),
            ApiError::Account(e) => e.to_string(),
            ApiError::Character(e) => e.to_string(),
            ApiError::Item(e) => e.to_string(),
            ApiError::GrandExchange(e) => e.to_string(),
            ApiError::Bank(e) => e.to_string(),
            ApiError::Map(e) => e.to_string(),
            ApiError::UnknownError { message, .. } => message.clone(),
        }
    }
}
