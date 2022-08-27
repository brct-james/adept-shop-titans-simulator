use super::equipment::ElementType;

use serde::{Deserialize, Serialize};

/// Defines the valid types of mini boss
pub enum MiniBossType {
    Agile,
    Dire,
    Huge,
    Legendary,
}

/// A specific combat encounter for a simulation
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Encounter {
    zone: String,
    hp: u32,
    damage: u32,
    defense_cap: u32,
    aoe_damage_base: u16,
    aoe_chance: f64,
    is_boss: bool,
    is_extreme: bool,
    barrier_type: Option<ElementType>,
    barrier_hp: u32,
    max_num_heroes: u8,
    evasion: f64,
    crit_chance_modifier: f64,
    crit_chance: f64,
    barrier_modifier: f64,
    aoe_damage: f64,
}

impl Encounter {
    pub fn is_extreme_or_boss(&self) -> (bool, bool) {
        return (self.is_extreme, self.is_boss);
    }
}

/// Create an encounter performing type validation and calculating certain fields
pub fn create_encounter(
    zone: String,
    hp: u32,
    damage: u32,
    defense_cap: u32,
    aoe_damage_base: u16,
    aoe_chance: u16,
    is_boss: bool,
    is_extreme: bool,
    mini_boss: Option<MiniBossType>,
    barrier_type: Option<ElementType>,
    barrier_hp: u32,
    max_num_heroes: u8,
) -> Result<Encounter, &'static str> {
    if damage <= 0 {
        return Err("Damage <= 0");
    }

    let mut evasion = -1.0f64;
    let mut hp_modifier = 1.0f64;
    let mut damage_modifier = 1.0f64;
    let mut crit_chance_modifier = 1.0f64;
    let crit_chance = 0.1f64;
    let barrier_modifier = 0.2f64;
    let aoe_damage = f64::from(aoe_damage_base) / f64::from(damage);

    match mini_boss {
        Some(mb) => match mb {
            MiniBossType::Agile => {
                evasion = 0.4f64;
            }
            MiniBossType::Dire => {
                hp_modifier = 1.5f64;
                crit_chance_modifier = 3f64;
            }
            MiniBossType::Huge => {
                hp_modifier = 2.0f64;
            }
            MiniBossType::Legendary => {
                hp_modifier = 1.5f64;
                damage_modifier = 1.25f64;
                crit_chance_modifier = 1.5f64;
                evasion = 0.1f64;
            }
        },
        _ => (),
    }

    let encounter = Encounter {
        zone,
        hp: (f64::from(hp) * hp_modifier).round() as u32,
        damage: (f64::from(damage) * damage_modifier).round() as u32,
        defense_cap,
        aoe_damage_base,
        aoe_chance: f64::from(aoe_chance) / 100.0f64,
        is_boss,
        is_extreme,
        barrier_type,
        barrier_hp,
        max_num_heroes,
        evasion,
        crit_chance_modifier,
        crit_chance,
        barrier_modifier,
        aoe_damage,
    };

    return Ok(encounter);
}

/// Contains information for generating combat Encounters
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Dungeon {
    barriers: Vec<String>,
    hp_range: [i16; 2],
    // etc
}
