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

/// Information on blueprints/gear
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Blueprint {
    name: String,
    type_: String,
    unlock_prerequisite: String,
    research_scrolls: u16,
    antique_tokens: u16,
    tier: u8,
    value: u32,
    crafting_time: u32,
    crafting_time_formatted: String,
    value_per_crafting_time: f64,
    merchant_xp: u32,
    merchant_xp_per_crafting_time: f64,
    worker_xp: u32,
    fusion_xp: u32,
    favor: u32,
    airship_power: u32,

    required_worker_1: String,
    worker_level_1: u8,
    required_worker_2: String,
    worker_level_2: u8,
    required_worker_3: String,
    worker_level_3: u8,

    iron_cost: u16,
    wood_cost: u16,
    leather_cost: u16,
    herbs_cost: u16,
    steel_cost: u16,
    ironwood_cost: u16,
    fabric_cost: u16,
    oil_cost: u16,
    ether_cost: u16,
    jewel_cost: u16,

    component_name_1: String,
    component_quality_1: String,
    component_amount_1: u8,
    component_name_2: String,
    component_quality_2: String,
    component_amount_2: u8,

    atk: f64,
    def: f64,
    hp: f64,
    eva: f64,
    crit: f64,

    elemental_affinity: String,
    spirit_affinity: String,

    // crafting upgrades

    // ascension
    discount_energy: u16,
    surcharge_energy: u16,
    suggest_energy: u16,
    speed_up_energy: u16,
}

pub fn create_blueprint(
    name: String,
    type_: String,
    unlock_prerequisite: String,
    research_scrolls: u16,
    antique_tokens: u16,
    tier: u8,
    value: u32,
    crafting_time: u32,
    crafting_time_formatted: String,
    value_per_crafting_time: f64,
    merchant_xp: u32,
    merchant_xp_per_crafting_time: f64,
    worker_xp: u32,
    fusion_xp: u32,
    favor: u32,
    airship_power: u32,

    required_worker_1: String,
    worker_level_1: u8,
    required_worker_2: String,
    worker_level_2: u8,
    required_worker_3: String,
    worker_level_3: u8,

    iron_cost: u16,
    wood_cost: u16,
    leather_cost: u16,
    herbs_cost: u16,
    steel_cost: u16,
    ironwood_cost: u16,
    fabric_cost: u16,
    oil_cost: u16,
    ether_cost: u16,
    jewel_cost: u16,

    component_name_1: String,
    component_quality_1: String,
    component_amount_1: u8,
    component_name_2: String,
    component_quality_2: String,
    component_amount_2: u8,

    atk: f64,
    def: f64,
    hp: f64,
    eva: f64,
    crit: f64,

    elemental_affinity: String,
    spirit_affinity: String,

    // crafting upgrades

    // ascension
    discount_energy: u16,
    surcharge_energy: u16,
    suggest_energy: u16,
    speed_up_energy: u16,
) -> Blueprint {
    return Blueprint {
        name,
        type_,
        unlock_prerequisite,
        research_scrolls,
        antique_tokens,
        tier,
        value,
        crafting_time,
        crafting_time_formatted,
        value_per_crafting_time,
        merchant_xp,
        merchant_xp_per_crafting_time,
        worker_xp,
        fusion_xp,
        favor,
        airship_power,

        required_worker_1,
        worker_level_1,
        required_worker_2,
        worker_level_2,
        required_worker_3,
        worker_level_3,

        iron_cost,
        wood_cost,
        leather_cost,
        herbs_cost,
        steel_cost,
        ironwood_cost,
        fabric_cost,
        oil_cost,
        ether_cost,
        jewel_cost,

        component_name_1,
        component_quality_1,
        component_amount_1,
        component_name_2,
        component_quality_2,
        component_amount_2,

        atk,
        def,
        hp,
        eva,
        crit,

        elemental_affinity,
        spirit_affinity,

        // crafting upgrades

        // ascension
        discount_energy,
        surcharge_energy,
        suggest_energy,
        speed_up_energy,
    };
}
