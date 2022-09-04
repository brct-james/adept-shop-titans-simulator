use crate::decimals::round_to_2;
use crate::inputs::{create_dungeon_input, DungeonInput};

use super::equipment::ElementType;

use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

use std::string::ToString;

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
    hp: f64,
    hp_max: f64,
    damage: f64,
    defense_cap: f64,
    aoe_damage_base: f64,
    aoe_chance: f64,
    is_boss: bool,
    is_extreme: bool,
    barrier_type: Option<ElementType>,
    barrier_hp: f64,
    barrier_hp_max: f64,
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

    pub fn get_defense_cap(&self) -> f64 {
        return self.defense_cap;
    }

    pub fn get_damage_info(&self) -> (f64, f64) {
        return (self.damage, self.aoe_damage);
    }

    pub fn get_aoe_info(&self) -> (f64, f64) {
        return (self.aoe_chance, self.aoe_damage);
    }

    pub fn get_crit_info(&self) -> (f64, f64) {
        return (self.crit_chance, self.crit_chance_modifier);
    }

    pub fn get_barrier_info(&self) -> (f64, f64, f64, Option<ElementType>) {
        return (
            self.barrier_hp,
            self.barrier_hp_max,
            self.barrier_modifier,
            self.barrier_type,
        );
    }

    pub fn set_barrier_hp_and_modifier(&mut self, barrier_hp: f64, barrier_modifier: f64) {
        self.barrier_hp = barrier_hp;
        self.barrier_modifier = barrier_modifier;
    }

    pub fn get_hp_info(&self) -> (f64, f64) {
        return (self.hp, self.hp_max);
    }

    pub fn set_hp(&mut self, hp: f64) {
        self.hp = hp;
    }

    pub fn get_evasion(&self) -> f64 {
        return self.evasion;
    }

    pub fn init_barrier_modifier(&mut self) {
        if self.barrier_hp == 0.0 {
            self.barrier_modifier = 1.0;
        } else {
            self.barrier_modifier = 0.2;
        }
    }

    pub fn round_floats_for_display(&self) -> Encounter {
        let mut e2 = self.clone();
        e2.hp = round_to_2(e2.hp);
        e2.hp_max = round_to_2(e2.hp_max);
        e2.damage = round_to_2(e2.damage);
        e2.defense_cap = round_to_2(e2.defense_cap);
        e2.aoe_damage_base = round_to_2(e2.aoe_damage_base);
        e2.aoe_chance = round_to_2(e2.aoe_chance);
        e2.barrier_hp = round_to_2(e2.barrier_hp);
        e2.barrier_hp_max = round_to_2(e2.barrier_hp_max);
        e2.evasion = round_to_2(e2.evasion);
        e2.crit_chance_modifier = round_to_2(e2.crit_chance_modifier);
        e2.crit_chance = round_to_2(e2.crit_chance);
        e2.barrier_modifier = round_to_2(e2.barrier_modifier);
        e2.aoe_damage = round_to_2(e2.aoe_damage);
        return e2;
    }
}

/// Create an encounter performing type validation and calculating certain fields
pub fn create_encounter(
    zone: String,
    hp: f64,
    damage: f64,
    defense_cap: f64,
    aoe_damage_base: f64,
    aoe_chance: f64,
    is_boss: bool,
    is_extreme: bool,
    mini_boss: Option<MiniBossType>,
    barrier_type: Option<ElementType>,
    barrier_hp: f64,
    max_num_heroes: u8,
) -> Result<Encounter, &'static str> {
    if damage <= 0.0 {
        return Err("Damage <= 0");
    }

    let mut evasion = -1.0;
    let mut hp_modifier = 1.0;
    let mut damage_modifier = 1.0;
    let mut crit_chance_modifier = 1.0;
    let crit_chance = 0.1;
    let barrier_modifier = 0.2;
    let aoe_damage = aoe_damage_base / damage;

    match mini_boss {
        Some(mb) => match mb {
            MiniBossType::Agile => {
                evasion = 0.4;
            }
            MiniBossType::Dire => {
                hp_modifier = 1.5;
                crit_chance_modifier = 3.0;
            }
            MiniBossType::Huge => {
                hp_modifier = 2.0;
            }
            MiniBossType::Legendary => {
                hp_modifier = 1.5;
                damage_modifier = 1.25;
                crit_chance_modifier = 1.5;
                evasion = 0.1;
            }
        },
        _ => (),
    }

    let encounter = Encounter {
        zone,
        hp: hp * hp_modifier,
        hp_max: hp * hp_modifier,
        damage: damage * damage_modifier,
        defense_cap,
        aoe_damage_base,
        aoe_chance: aoe_chance / 100.0,
        is_boss,
        is_extreme,
        barrier_type,
        barrier_hp,
        barrier_hp_max: barrier_hp,
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Dungeon {
    zone: String,

    // Misc
    max_num_heroes: u8,

    // Normal Encounters
    hp: [f64; 4],
    damage: [f64; 4],
    defense_cap: [f64; 4],
    aoe_damage: [f64; 4],
    aoe_chance: [f64; 4],
    minimum_power: [u32; 4],

    // Extreme Only
    barrier_types: [ElementType; 3],
    barrier_health: f64,

    // Bosses
    boss_hp: [f64; 4],
    boss_damage: [f64; 4],
    boss_defense_cap: [f64; 4],
    boss_aoe_damage: [f64; 4],
    boss_aoe_chance: [f64; 4],
    boss_minimum_power: [u32; 4],
    boss_barrier_type: ElementType,
    boss_barrier_health: f64,
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
                if sel_diff == 4 {
                    self.barrier_health
                } else {
                    0.0
                },
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

impl From<Dungeon> for DungeonInput {
    fn from(item: Dungeon) -> Self {
        let mut barrier_types: [String; 3] = [
            String::from("Any"),
            String::from("Any"),
            String::from("Any"),
        ];
        for (i, bt) in item.barrier_types.iter().enumerate() {
            barrier_types[i] = bt.to_string();
        }
        let boss_barrier_type = item.boss_barrier_type.to_string();
        return create_dungeon_input(
            item.zone,
            item.max_num_heroes,
            item.hp,
            item.damage,
            item.defense_cap,
            item.aoe_damage,
            item.aoe_chance,
            item.minimum_power,
            barrier_types,
            item.barrier_health,
            item.boss_hp,
            item.boss_damage,
            item.boss_defense_cap,
            item.boss_aoe_damage,
            item.boss_aoe_chance,
            item.boss_minimum_power,
            boss_barrier_type,
            item.boss_barrier_health,
        );
    }
}

/// Create a dungeon performing type validation and calculating certain fields
pub fn create_dungeon(
    zone: String,
    max_num_heroes: u8,
    hp: [f64; 4],
    damage: [f64; 4],
    defense_cap: [f64; 4],
    aoe_damage: [f64; 4],
    aoe_chance: [f64; 4],
    minimum_power: [u32; 4],
    barrier_types: [ElementType; 3],
    barrier_health: f64,
    boss_hp: [f64; 4],
    boss_damage: [f64; 4],
    boss_defense_cap: [f64; 4],
    boss_aoe_damage: [f64; 4],
    boss_aoe_chance: [f64; 4],
    boss_minimum_power: [u32; 4],
    boss_barrier_type: ElementType,
    boss_barrier_health: f64,
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
