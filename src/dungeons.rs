use super::equipment::ElementType;

use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Defines the valid types of mini boss
pub enum MiniBossType {
    Agile,
    Dire,
    Huge,
    Legendary,
}

impl Distribution<MiniBossType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MiniBossType {
        match rng.gen_range(0..=3) {
            0 => MiniBossType::Agile,
            1 => MiniBossType::Dire,
            2 => MiniBossType::Huge,
            _ => MiniBossType::Legendary,
        }
    }
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
    zone: String,

    // Misc
    max_num_heroes: u8,

    // Normal Encounters
    hp: [u32; 4],
    damage: [u32; 4],
    defense_cap: [u32; 4],
    aoe_damage: [u16; 4],
    aoe_chance: [u16; 4],
    minimum_power: [u32; 4],

    // Extreme Only
    barrier_types: [ElementType; 3],
    barrier_health: u32,

    // Bosses
    boss_hp: [u32; 4],
    boss_damage: [u32; 4],
    boss_defense_cap: [u32; 4],
    boss_aoe_damage: [u16; 4],
    boss_aoe_chance: [u16; 4],
    boss_minimum_power: [u32; 4],
    boss_barrier_type: ElementType,
    boss_barrier_health: u32,
}

impl Dungeon {
    /// Difficulty settings (include all that should apply):
    /// 1 - Easy, 2 - Medium, 3 - Hard, 4 - Extreme,
    /// 5 - Boss Easy, 6 - Boss Medium, 7 - Boss Hard, 8 - Boss Extreme
    ///
    /// force_minibosses:
    /// false - No Minibosses, true - Only Minibosses, none - Random Chance of Minibosses
    pub fn generate_encounter_from_dungeon(
        &self,
        difficulty_settings: &Vec<usize>,
        force_minibosses: Option<bool>,
    ) -> Result<Encounter, &'static str> {
        // Check for out of bounds
        for &difficulty in difficulty_settings {
            if difficulty > 8 || difficulty < 1 {
                return Err("difficulty settings must be within range 1-8 inclusive");
            }
        }

        let mut rng = rand::thread_rng();
        let diff_rand = rng.gen_range(0..difficulty_settings.len());
        let mut sel_diff = difficulty_settings[diff_rand];
        let encounter: Encounter;

        if sel_diff <= 4 {
            // is not boss
            sel_diff -= 1;

            // if necessary select a miniboss type
            let miniboss: Option<MiniBossType>;
            match force_minibosses {
                Some(setting) => {
                    miniboss = if setting {
                        Some(rand::random::<MiniBossType>())
                    } else {
                        None
                    }
                }
                _ => {
                    if rng.gen_range(0..2) == 1 {
                        miniboss = Some(rand::random::<MiniBossType>());
                    } else {
                        miniboss = None;
                    }
                }
            }

            encounter = create_encounter(
                self.zone.to_string(),
                self.hp[sel_diff],
                self.damage[sel_diff],
                self.defense_cap[sel_diff],
                self.aoe_damage[sel_diff],
                self.aoe_chance[sel_diff],
                false,
                sel_diff == 4,
                miniboss,
                if sel_diff == 4 {
                    Some(self.barrier_types[rng.gen_range(0..3)])
                } else {
                    None
                },
                self.barrier_health,
                self.max_num_heroes,
            )
            .unwrap();
        } else {
            // is boss
            sel_diff = sel_diff - 4;
            sel_diff -= 1;

            encounter = create_encounter(
                self.zone.to_string(),
                self.boss_hp[sel_diff],
                self.boss_damage[sel_diff],
                self.boss_defense_cap[sel_diff],
                self.boss_aoe_damage[sel_diff],
                self.boss_aoe_chance[sel_diff],
                true,
                sel_diff == 4,
                None,
                if sel_diff == 4 {
                    Some(self.boss_barrier_type)
                } else {
                    None
                },
                self.boss_barrier_health,
                self.max_num_heroes,
            )
            .unwrap();
        }

        return Ok(encounter);
    }
}

/// Create a dungeon performing type validation and calculating certain fields
pub fn create_dungeon(
    zone: String,
    max_num_heroes: u8,
    hp: [u32; 4],
    damage: [u32; 4],
    defense_cap: [u32; 4],
    aoe_damage: [u16; 4],
    aoe_chance: [u16; 4],
    minimum_power: [u32; 4],
    barrier_types: [ElementType; 3],
    barrier_health: u32,
    boss_hp: [u32; 4],
    boss_damage: [u32; 4],
    boss_defense_cap: [u32; 4],
    boss_aoe_damage: [u16; 4],
    boss_aoe_chance: [u16; 4],
    boss_minimum_power: [u32; 4],
    boss_barrier_type: ElementType,
    boss_barrier_health: u32,
) -> Result<Dungeon, &'static str> {
    let dungeon = Dungeon {
        zone,
        max_num_heroes,
        hp,
        damage,
        defense_cap,
        aoe_damage,
        aoe_chance,
        minimum_power,
        barrier_types,
        barrier_health,
        boss_hp,
        boss_damage,
        boss_defense_cap,
        boss_aoe_damage,
        boss_aoe_chance,
        boss_minimum_power,
        boss_barrier_type,
        boss_barrier_health,
    };

    return Ok(dungeon);
}
