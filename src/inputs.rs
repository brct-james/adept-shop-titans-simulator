use serde::{Deserialize, Serialize};

use crate::decimals::{_round_array_of_len_4_to_2, round_to_2};
use crate::equipment::ElementType;

use std::str::FromStr;

use super::heroes::{create_hero, Hero};

use super::dungeons::{create_dungeon, Dungeon};

/// Defines HeroeInput format for deserialization from CSV
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HeroInput {
    identifier: String,
    class: String,
    level: u8,
    rank: u8,
    innate_tier: u8,
    hp: f64,
    attack: f64,
    defense: f64,
    threat: u16,
    critical_chance: f64,
    critical_multiplier: f64,
    evasion: f64,
    element_qty: u16,
    element_type: String,
    armadillo_qty: u8,
    lizard_qty: u8,
    shark_qty: u8,
    dinosaur_qty: u8,
    mundra_qty: u8,
    attack_modifier: f64,
    defense_modifier: f64,
}

impl HeroInput {
    pub fn _round_floats_for_display(&self) -> HeroInput {
        let mut hi2 = self.clone();
        hi2.hp = round_to_2(hi2.hp);
        hi2.attack = round_to_2(hi2.attack);
        hi2.defense = round_to_2(hi2.defense);
        hi2.critical_chance = round_to_2(hi2.critical_chance);
        hi2.critical_multiplier = round_to_2(hi2.critical_multiplier);
        hi2.evasion = round_to_2(hi2.evasion);
        hi2.attack_modifier = round_to_2(hi2.attack_modifier);
        hi2.defense_modifier = round_to_2(hi2.defense_modifier);
        return hi2;
    }
}

impl From<HeroInput> for Hero {
    /// Create a hero from the input object performing type validation and calculating certain fields
    fn from(item: HeroInput) -> Self {
        return create_hero(
            item.identifier,
            item.class,
            item.level,
            item.rank,
            item.innate_tier,
            item.hp,
            item.attack,
            item.defense,
            item.threat,
            item.critical_chance,
            item.critical_multiplier,
            item.evasion,
            item.element_qty,
            item.element_type,
            item.armadillo_qty,
            item.lizard_qty,
            item.shark_qty,
            item.dinosaur_qty,
            item.mundra_qty,
            item.attack_modifier,
            item.defense_modifier,
        )
        .unwrap();
    }
}

pub fn create_hero_input(
    identifier: String,
    class: String,
    level: u8,
    rank: u8,
    innate_tier: u8,
    hp: f64,
    attack: f64,
    defense: f64,
    threat: u16,
    critical_chance: f64,
    critical_multiplier: f64,
    evasion: f64,
    element_qty: u16,
    element_type: String,
    armadillo_qty: u8,
    lizard_qty: u8,
    shark_qty: u8,
    dinosaur_qty: u8,
    mundra_qty: u8,
    attack_modifier: f64,
    defense_modifier: f64,
) -> HeroInput {
    return HeroInput {
        identifier,
        class,
        level,
        rank,
        innate_tier,
        hp,
        attack,
        defense,
        threat,
        critical_chance,
        critical_multiplier,
        evasion,
        element_qty,
        element_type,
        armadillo_qty,
        lizard_qty,
        shark_qty,
        dinosaur_qty,
        mundra_qty,
        attack_modifier,
        defense_modifier,
    };
}

pub fn load_heroes_from_csv(path: String) -> Vec<Hero> {
    let mut heroes: Vec<Hero> = vec![];
    let mut reader = csv::Reader::from_path(path).unwrap();
    for result in reader.deserialize() {
        let hero_in: HeroInput = result.unwrap();
        heroes.push(Hero::from(hero_in));
    }
    return heroes;
}

pub fn _save_heroes_to_csv(path: String, heroes: Vec<Hero>) -> Result<(), std::io::Error> {
    let mut wtr = csv::Writer::from_path(path)?;

    for hero in heroes {
        wtr.serialize(HeroInput::from(hero)._round_floats_for_display())?;
    }

    wtr.flush()?;
    return Ok(());
}

/// Defines DungeonInput format for deserialization from YAML
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DungeonInput {
    zone: String,
    max_num_heroes: u8,
    hp: [f64; 4],
    damage: [f64; 4],
    defense_cap: [f64; 4],
    aoe_damage: [f64; 4],
    aoe_chance: [f64; 4],
    minimum_power: [u32; 4],
    barrier_types: [String; 3],
    barrier_health: f64,
    boss_hp: [f64; 4],
    boss_damage: [f64; 4],
    boss_defense_cap: [f64; 4],
    boss_aoe_damage: [f64; 4],
    boss_aoe_chance: [f64; 4],
    boss_minimum_power: [u32; 4],
    boss_barrier_type: String,
    boss_barrier_health: f64,
}

impl DungeonInput {
    pub fn _round_floats_for_display(&self) -> DungeonInput {
        let mut di2 = self.clone();
        di2.hp = _round_array_of_len_4_to_2(di2.hp);
        di2.damage = _round_array_of_len_4_to_2(di2.damage);
        di2.defense_cap = _round_array_of_len_4_to_2(di2.defense_cap);
        di2.aoe_damage = _round_array_of_len_4_to_2(di2.aoe_damage);
        di2.aoe_chance = _round_array_of_len_4_to_2(di2.aoe_chance);
        di2.barrier_health = round_to_2(di2.barrier_health);

        di2.boss_hp = _round_array_of_len_4_to_2(di2.boss_hp);
        di2.boss_damage = _round_array_of_len_4_to_2(di2.boss_damage);
        di2.boss_defense_cap = _round_array_of_len_4_to_2(di2.boss_defense_cap);
        di2.boss_aoe_damage = _round_array_of_len_4_to_2(di2.boss_aoe_damage);
        di2.boss_aoe_chance = _round_array_of_len_4_to_2(di2.boss_aoe_chance);
        di2.boss_barrier_health = round_to_2(di2.boss_barrier_health);

        return di2;
    }
}

impl From<DungeonInput> for Dungeon {
    /// Create a hero from the input object performing type validation and calculating certain fields
    fn from(item: DungeonInput) -> Self {
        let mut barrier_types: [ElementType; 3] =
            [ElementType::Any, ElementType::Any, ElementType::Any];
        for (i, bt) in item.barrier_types.iter().enumerate() {
            barrier_types[i] = ElementType::from_str(bt.as_str()).unwrap();
        }
        let boss_barrier_type = ElementType::from_str(item.boss_barrier_type.as_str()).unwrap();
        return create_dungeon(
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
        )
        .unwrap();
    }
}

pub fn create_dungeon_input(
    zone: String,
    max_num_heroes: u8,
    hp: [f64; 4],
    damage: [f64; 4],
    defense_cap: [f64; 4],
    aoe_damage: [f64; 4],
    aoe_chance: [f64; 4],
    minimum_power: [u32; 4],
    barrier_types: [String; 3],
    barrier_health: f64,
    boss_hp: [f64; 4],
    boss_damage: [f64; 4],
    boss_defense_cap: [f64; 4],
    boss_aoe_damage: [f64; 4],
    boss_aoe_chance: [f64; 4],
    boss_minimum_power: [u32; 4],
    boss_barrier_type: String,
    boss_barrier_health: f64,
) -> DungeonInput {
    return DungeonInput {
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
}

pub fn load_dungeons_from_yaml(path: String) -> Vec<Dungeon> {
    let mut dungeons: Vec<Dungeon> = vec![];
    let reader = std::fs::File::open(path).unwrap();
    for dungeon_in in serde_yaml::from_reader::<std::fs::File, Vec<DungeonInput>>(reader).unwrap() {
        dungeons.push(Dungeon::from(dungeon_in));
    }
    return dungeons;
}

pub fn _save_dungeons_to_yaml(path: String, dungeons: Vec<Dungeon>) -> Result<(), std::io::Error> {
    let writer = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .expect("Couldn't open file");

    serde_yaml::to_writer(writer, &dungeons).unwrap();

    return Ok(());
}
