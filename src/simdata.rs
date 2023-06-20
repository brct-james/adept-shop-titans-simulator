use std::collections::HashMap;

use crate::{
    dungeons::Dungeon,
    equipment::Blueprint,
    hero_builder::{Hero, HeroClass},
    heroes::SimHero,
    skills::{HeroSkill, InnateSkill},
};

#[derive(Default, Clone)]
pub struct SimData {
    pub hero_classes: HashMap<String, HeroClass>,
    pub hero_skill_tier_1_name_map: HashMap<String, String>,
    pub hero_skill_any_tier_to_tier_1_name_map: HashMap<String, String>,
    pub hero_skill_map: HashMap<String, HeroSkill>,
    pub loaded_valid_skills: Vec<String>,
    pub innate_skill_tier_1_name_map: HashMap<String, String>,
    pub innate_skill_any_tier_to_tier_1_name_nap: HashMap<String, String>,
    pub class_innate_skill_names_map: HashMap<String, String>,
    pub innate_skill_map: HashMap<String, InnateSkill>,
    pub bp_map: HashMap<String, Blueprint>,
    pub loaded_heroes: HashMap<String, SimHero>,
    pub loaded_heroes_from_builder: HashMap<String, Hero>,
    pub loaded_dungeons: HashMap<String, Dungeon>,
    pub hero_skill_abbreviation_map: HashMap<String, String>,
    pub hero_abbreviation_skill_map: HashMap<String, String>,
}
