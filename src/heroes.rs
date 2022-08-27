use super::equipment::{BoosterType, ElementType};

use serde::{Deserialize, Serialize};

/// One or more Heroes fighting together in a dungeon and what booster they have
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Team {
    heroes: Vec<Hero>,
    booster: Option<BoosterType>,
    num_fighters: u8,
    num_rogues: u8,
    num_spellcasters: u8,
    champion: String,
    champion_innate_tier: u8,
}

/// Defines valid hero archetypes
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum HeroArchetype {
    RedFighter,
    GreenRogue,
    BlueSpellcaster,
    Champion,
}

impl Team {
    pub fn normalize_percents(&mut self, is_extreme: bool, is_boss: bool) {
        for hero in &mut self.heroes {
            if is_extreme {
                hero.modify_for_extreme_encounter();
            }

            if is_boss {
                hero.modify_for_boss_encounter();
            }
        }
    }
}

/// Create a team performing type validation and calculating certain fields
pub fn create_team(heroes: Vec<Hero>, booster: Option<BoosterType>) -> Result<Team, &'static str> {
    if heroes.len() < 1 {
        return Err("cannot form team with < 1 hero");
    }

    let mut num_fighters = 0u8;
    let mut num_rogues = 0u8;
    let mut num_spellcasters = 0u8;
    let mut champion = "None".to_string();
    let mut champion_innate_tier = 1u8;

    for hero in &heroes {
        match hero.archetype {
            HeroArchetype::RedFighter => num_fighters += 1,
            HeroArchetype::GreenRogue => num_rogues += 1,
            HeroArchetype::BlueSpellcaster => num_spellcasters += 1,
            HeroArchetype::Champion => {
                champion = hero.class.to_string();
                if hero.rank >= 11 {
                    champion_innate_tier = 4u8;
                } else if hero.rank >= 7 {
                    champion_innate_tier = 3u8;
                } else if hero.rank >= 4 {
                    champion_innate_tier = 2u8;
                }
            }
        }
    }

    let team = Team {
        heroes,
        booster,
        num_fighters,
        num_rogues,
        num_spellcasters,
        champion,
        champion_innate_tier,
    };

    return Ok(team);
}

/// Holds information on a hero / champion
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Hero {
    identifier: String,
    class: String,
    archetype: HeroArchetype,
    level: u8,
    rank: u8,
    hp: u32,
    attack: f64,
    defense: f64,
    threat: u16,
    critical_chance: f64,
    critical_multiplier: f64,
    evasion: f64,
    element_qty: u16,
    element_type: ElementType,
    armadillo_qty: u8,
    lizard_qty: u8,
    shark_qty: u8,
    dinosaur_qty: u8,
    mundra_qty: u8,
    attack_modifier: f64,  // %
    defense_modifier: f64, // %
    extreme_crit_bonus: f64,
    // skills: Vec<Skill>,
}

impl Hero {
    fn modify_for_extreme_encounter(&mut self) {
        self.evasion -= 0.2f64;
        self.extreme_crit_bonus = 1.0f64;
    }
    fn modify_for_boss_encounter(&mut self) {
        self.defense *= self.defense_modifier + 0.2f64 * f64::from(self.mundra_qty);
    }
}

/// Create a hero performing type validation and calculating certain fields
pub fn create_hero(
    identifier: String,
    class: String,
    archetype: HeroArchetype,
    level: u8,
    rank: u8,
    hp: u32,
    attack: u32,
    defense: u32,
    threat: u16,
    critical_chance: u16,
    critical_multiplier: f64,
    evasion: u16,
    element_qty: u16,
    element_type: ElementType,
    armadillo_qty: u8,
    lizard_qty: u8,
    shark_qty: u8,
    dinosaur_qty: u8,
    mundra_qty: u8,
    attack_modifier: u16,
    defense_modifier: u16,
) -> Result<Hero, &'static str> {
    let mut hero = Hero {
        identifier,
        class,
        archetype,
        level,
        rank,
        hp,
        attack: f64::from(attack),
        defense: f64::from(defense),
        threat,
        critical_chance: f64::from(critical_chance) / 100.0f64,
        critical_multiplier,
        evasion: f64::from(evasion) / 100.0f64,
        element_qty,
        element_type,
        armadillo_qty,
        lizard_qty,
        shark_qty,
        dinosaur_qty,
        mundra_qty,
        attack_modifier: 1.0f64 + f64::from(attack_modifier) / 100.0f64,
        defense_modifier: 1.0f64 + f64::from(defense_modifier) / 100.0f64,
        extreme_crit_bonus: 0.0f64,
    };

    hero.attack /= hero.attack_modifier;
    hero.defense /= hero.defense_modifier;

    return Ok(hero);
}
