use serde::{Deserialize, Serialize};

use crate::decimals::round_to_2;

use super::heroes::{create_hero, Hero};

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
