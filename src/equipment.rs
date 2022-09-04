use serde::{Deserialize, Serialize};
use strum;

/// Defines valid element types
#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, strum::Display, strum::EnumString,
)]
pub enum ElementType {
    #[strum(serialize = "Fire")]
    Fire,

    #[strum(serialize = "Water")]
    Water,

    #[strum(serialize = "Air")]
    Air,

    #[strum(serialize = "Earth")]
    Earth,

    #[strum(serialize = "Light")]
    Light,

    #[strum(serialize = "Dark")]
    Dark,

    #[strum(serialize = "Any")]
    Any,
}

/// Defines valid booster types
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum BoosterType {
    PowerBooster,
    SuperPowerBooster,
    MegaPowerBooster,
}
