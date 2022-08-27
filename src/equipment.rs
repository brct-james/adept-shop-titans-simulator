use serde::{Deserialize, Serialize};

/// Defines valid element types
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum ElementType {
    Fire,
    Water,
    Air,
    Earth,
    Light,
    Dark,
}

/// Defines valid booster types
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum BoosterType {
    LootBugleI,
}
