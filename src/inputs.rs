use serde::{Deserialize, Serialize};

use crate::decimals::{_round_array_of_len_4_to_2, round_to_2};
use crate::equipment::{Blueprint, ElementType};
use crate::hero_builder::{create_hero, Hero, HeroClass};

use std::collections::HashMap;
use std::str::FromStr;

use super::heroes::{create_sim_hero, SimHero};

use super::dungeons::{create_dungeon, Dungeon};

/// Defines HeroeInput format for deserialization from CSV
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SimHeroInput {
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

impl SimHeroInput {
    pub fn _round_floats_for_display(&self) -> SimHeroInput {
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

impl From<SimHeroInput> for SimHero {
    /// Create a hero from the input object performing type validation and calculating certain fields
    fn from(item: SimHeroInput) -> Self {
        return create_sim_hero(
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

pub fn create_sim_hero_input(
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
) -> SimHeroInput {
    return SimHeroInput {
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

pub fn load_sim_heroes_from_csv(path: String) -> Vec<SimHero> {
    let mut heroes: Vec<SimHero> = vec![];
    let mut reader = csv::Reader::from_path(path).unwrap();
    for result in reader.deserialize() {
        let hero_in: SimHeroInput = result.unwrap();
        heroes.push(SimHero::from(hero_in));
    }
    return heroes;
}

pub fn _save_sim_heroes_to_csv(path: String, heroes: Vec<SimHero>) -> Result<(), std::io::Error> {
    let already_exists = std::path::Path::new(&path).exists();
    let writer = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(!already_exists)
        .from_writer(writer);

    for hero in heroes {
        wtr.serialize(SimHeroInput::from(hero)._round_floats_for_display())?;
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
    barrier_healths: [f64; 4],
    boss_hp: [f64; 4],
    boss_damage: [f64; 4],
    boss_defense_cap: [f64; 4],
    boss_aoe_damage: [f64; 4],
    boss_aoe_chance: [f64; 4],
    boss_minimum_power: [u32; 4],
    boss_barrier_type: String,
    boss_barrier_healths: [f64; 4],
}

impl DungeonInput {
    pub fn _round_floats_for_display(&self) -> DungeonInput {
        let mut di2 = self.clone();
        di2.hp = _round_array_of_len_4_to_2(di2.hp);
        di2.damage = _round_array_of_len_4_to_2(di2.damage);
        di2.defense_cap = _round_array_of_len_4_to_2(di2.defense_cap);
        di2.aoe_damage = _round_array_of_len_4_to_2(di2.aoe_damage);
        di2.aoe_chance = _round_array_of_len_4_to_2(di2.aoe_chance);
        di2.barrier_healths = _round_array_of_len_4_to_2(di2.barrier_healths);

        di2.boss_hp = _round_array_of_len_4_to_2(di2.boss_hp);
        di2.boss_damage = _round_array_of_len_4_to_2(di2.boss_damage);
        di2.boss_defense_cap = _round_array_of_len_4_to_2(di2.boss_defense_cap);
        di2.boss_aoe_damage = _round_array_of_len_4_to_2(di2.boss_aoe_damage);
        di2.boss_aoe_chance = _round_array_of_len_4_to_2(di2.boss_aoe_chance);
        di2.boss_barrier_healths = _round_array_of_len_4_to_2(di2.boss_barrier_healths);

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
            item.barrier_healths,
            item.boss_hp,
            item.boss_damage,
            item.boss_defense_cap,
            item.boss_aoe_damage,
            item.boss_aoe_chance,
            item.boss_minimum_power,
            boss_barrier_type,
            item.boss_barrier_healths,
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
    barrier_healths: [f64; 4],
    boss_hp: [f64; 4],
    boss_damage: [f64; 4],
    boss_defense_cap: [f64; 4],
    boss_aoe_damage: [f64; 4],
    boss_aoe_chance: [f64; 4],
    boss_minimum_power: [u32; 4],
    boss_barrier_type: String,
    boss_barrier_healths: [f64; 4],
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
        barrier_healths,
        boss_hp,
        boss_damage,
        boss_defense_cap,
        boss_aoe_damage,
        boss_aoe_chance,
        boss_minimum_power,
        boss_barrier_type,
        boss_barrier_healths,
    };
}

pub fn load_dungeons_from_yaml(path: String) -> HashMap<String, Dungeon> {
    let mut dungeons: HashMap<String, Dungeon> = Default::default();
    let reader = std::fs::File::open(path).unwrap();
    for (dungeon_key, dungeon_in) in
        serde_yaml::from_reader::<std::fs::File, HashMap<String, DungeonInput>>(reader).unwrap()
    {
        dungeons.insert(dungeon_key, Dungeon::from(dungeon_in));
    }
    return dungeons;
}

pub fn _save_dungeons_to_yaml(
    path: String,
    dungeons: HashMap<String, Dungeon>,
) -> Result<(), std::io::Error> {
    let writer = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    serde_yaml::to_writer(writer, &dungeons).unwrap();

    return Ok(());
}

/// Defines HeroInput format for deserialization from CSV
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HeroInput {
    identifier: String,
    class: String,
    level: u8,
    rank: u8,

    hp: f64,
    atk: f64,
    def: f64,
    eva: f64,
    crit_chance: f64,
    crit_mult: f64,
    threat_rating: u16,
    element_type: String,

    hp_seeds: u8,
    atk_seeds: u8,
    def_seeds: u8,

    skill_1: String,
    skill_2: String,
    skill_3: String,
    skill_4: String,
    skill_5: String,

    equipment_equipped_1: String,
    equipment_quality_1: String,
    elements_socketed_1: String,
    spirits_socketed_1: String,

    equipment_equipped_2: String,
    equipment_quality_2: String,
    elements_socketed_2: String,
    spirits_socketed_2: String,

    equipment_equipped_3: String,
    equipment_quality_3: String,
    elements_socketed_3: String,
    spirits_socketed_3: String,

    equipment_equipped_4: String,
    equipment_quality_4: String,
    elements_socketed_4: String,
    spirits_socketed_4: String,

    equipment_equipped_5: String,
    equipment_quality_5: String,
    elements_socketed_5: String,
    spirits_socketed_5: String,

    equipment_equipped_6: String,
    equipment_quality_6: String,
    elements_socketed_6: String,
    spirits_socketed_6: String,
}

impl HeroInput {
    pub fn _round_floats_for_display(&self) -> HeroInput {
        let mut hi2 = self.clone();
        hi2.hp = round_to_2(hi2.hp);
        hi2.atk = round_to_2(hi2.atk);
        hi2.def = round_to_2(hi2.def);
        hi2.eva = round_to_2(hi2.eva);
        hi2.crit_chance = round_to_2(hi2.crit_chance);
        hi2.crit_mult = round_to_2(hi2.crit_mult);
        return hi2;
    }
}

impl From<HeroInput> for Hero {
    /// Create a hero from the input object performing type validation and calculating certain fields
    fn from(item: HeroInput) -> Self {
        let skills: [String; 5] = [
            item.skill_1,
            item.skill_2,
            item.skill_3,
            item.skill_4,
            item.skill_5,
        ];
        let equipment_equipped: [String; 6] = [
            item.equipment_equipped_1,
            item.equipment_equipped_2,
            item.equipment_equipped_3,
            item.equipment_equipped_4,
            item.equipment_equipped_5,
            item.equipment_equipped_6,
        ];
        let equipment_quality: [String; 6] = [
            item.equipment_quality_1,
            item.equipment_quality_2,
            item.equipment_quality_3,
            item.equipment_quality_4,
            item.equipment_quality_5,
            item.equipment_quality_6,
        ];
        let elements_socketed: [String; 6] = [
            item.elements_socketed_1,
            item.elements_socketed_2,
            item.elements_socketed_3,
            item.elements_socketed_4,
            item.elements_socketed_5,
            item.elements_socketed_6,
        ];
        let spirits_socketed: [String; 6] = [
            item.spirits_socketed_1,
            item.spirits_socketed_2,
            item.spirits_socketed_3,
            item.spirits_socketed_4,
            item.spirits_socketed_5,
            item.spirits_socketed_6,
        ];

        return create_hero(
            item.identifier,
            item.class,
            item.level,
            item.rank,
            item.hp,
            item.atk,
            item.def,
            item.eva,
            item.crit_chance,
            item.crit_mult,
            item.threat_rating,
            item.element_type,
            item.hp_seeds,
            item.atk_seeds,
            item.def_seeds,
            skills,
            equipment_equipped,
            equipment_quality,
            elements_socketed,
            spirits_socketed,
        );
    }
}

pub fn create_hero_input(
    identifier: String,
    class: String,
    level: u8,
    rank: u8,

    hp: f64,
    atk: f64,
    def: f64,
    eva: f64,
    crit_chance: f64,
    crit_mult: f64,
    threat_rating: u16,
    element_type: String,

    hp_seeds: u8,
    atk_seeds: u8,
    def_seeds: u8,

    skills: [String; 5],

    equipment_equipped: [String; 6],
    equipment_quality: [String; 6],
    elements_socketed: [String; 6],
    spirits_socketed: [String; 6],
) -> HeroInput {
    return HeroInput {
        identifier,
        class,
        level,
        rank,

        hp,
        atk,
        def,
        eva,
        crit_chance,
        crit_mult,
        threat_rating,
        element_type,

        hp_seeds,
        atk_seeds,
        def_seeds,

        skill_1: skills[0].clone(),
        skill_2: skills[1].clone(),
        skill_3: skills[2].clone(),
        skill_4: skills[3].clone(),
        skill_5: skills[4].clone(),

        equipment_equipped_1: equipment_equipped[0].clone(),
        equipment_equipped_2: equipment_equipped[1].clone(),
        equipment_equipped_3: equipment_equipped[2].clone(),
        equipment_equipped_4: equipment_equipped[3].clone(),
        equipment_equipped_5: equipment_equipped[4].clone(),
        equipment_equipped_6: equipment_equipped[5].clone(),

        equipment_quality_1: equipment_quality[0].clone(),
        equipment_quality_2: equipment_quality[1].clone(),
        equipment_quality_3: equipment_quality[2].clone(),
        equipment_quality_4: equipment_quality[3].clone(),
        equipment_quality_5: equipment_quality[4].clone(),
        equipment_quality_6: equipment_quality[5].clone(),

        elements_socketed_1: elements_socketed[0].clone(),
        elements_socketed_2: elements_socketed[1].clone(),
        elements_socketed_3: elements_socketed[2].clone(),
        elements_socketed_4: elements_socketed[3].clone(),
        elements_socketed_5: elements_socketed[4].clone(),
        elements_socketed_6: elements_socketed[5].clone(),

        spirits_socketed_1: spirits_socketed[0].clone(),
        spirits_socketed_2: spirits_socketed[1].clone(),
        spirits_socketed_3: spirits_socketed[2].clone(),
        spirits_socketed_4: spirits_socketed[3].clone(),
        spirits_socketed_5: spirits_socketed[4].clone(),
        spirits_socketed_6: spirits_socketed[5].clone(),
    };
}

pub fn _load_heroes_from_csv(
    path: String,
    bp_map: HashMap<String, Blueprint>,
    hero_classes: HashMap<String, HeroClass>,
) -> HashMap<String, Hero> {
    let mut heroes: HashMap<String, Hero> = Default::default();
    let mut reader = csv::Reader::from_path(path).unwrap();
    for result in reader.deserialize() {
        let hero_in: HeroInput = result.unwrap();
        let identifier = hero_in.identifier.to_string();
        let mut hero = Hero::from(hero_in);
        hero.validate_equipment(&bp_map, &hero_classes);
        hero.scale_by_class(&hero_classes);
        heroes.insert(identifier, hero);
    }
    return heroes;
}

pub fn load_heroes_as_sim_heroes_from_csv(
    path: String,
    bp_map: HashMap<String, Blueprint>,
    hero_classes: HashMap<String, HeroClass>,
) -> HashMap<String, SimHero> {
    let mut heroes: HashMap<String, SimHero> = Default::default();
    let mut reader = csv::Reader::from_path(path).unwrap();
    for result in reader.deserialize() {
        let hero_in: HeroInput = result.unwrap();
        let identifier = hero_in.identifier.to_string();
        let mut hero = Hero::from(hero_in);
        hero.validate_equipment(&bp_map, &hero_classes);
        hero.scale_by_class(&hero_classes);
        heroes.insert(identifier, SimHero::from(hero));
    }
    return heroes;
}

pub fn _save_heroes_to_csv(
    path: String,
    heroes: HashMap<String, Hero>,
) -> Result<(), std::io::Error> {
    let heroes_vec: Vec<Hero> = heroes.values().cloned().collect();
    let already_exists = std::path::Path::new(&path).exists();
    let writer = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(!already_exists)
        .from_writer(writer);

    for hero in heroes_vec {
        wtr.serialize(HeroInput::from(hero)._round_floats_for_display())?;
    }

    wtr.flush()?;
    return Ok(());
}

pub fn load_hero_classes_from_yaml(path: String) -> HashMap<String, HeroClass> {
    let mut hero_classes: HashMap<String, HeroClass> = Default::default();
    let reader = std::fs::File::open(path).unwrap();
    for (class_name, hero_class) in
        serde_yaml::from_reader::<std::fs::File, HashMap<String, HeroClass>>(reader).unwrap()
    {
        hero_classes.insert(class_name, hero_class);
    }
    return hero_classes;
}

pub fn _save_hero_classes_to_yaml(
    path: String,
    hero_classes: HashMap<String, HeroClass>,
) -> Result<(), std::io::Error> {
    let already_exists = std::path::Path::new(&path).exists();
    let mut hashmap: HashMap<String, HeroClass>;
    if already_exists {
        hashmap = load_hero_classes_from_yaml(path.to_string());
        hashmap.extend(hero_classes);
    } else {
        hashmap = hero_classes;
    }

    let writer = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    serde_yaml::to_writer(writer, &hashmap).unwrap();

    return Ok(());
}
