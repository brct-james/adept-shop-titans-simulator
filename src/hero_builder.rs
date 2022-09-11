use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    decimals::round_to_2,
    equipment::Blueprint,
    heroes::{create_sim_hero, SimHero},
    inputs::{create_hero_input, HeroInput},
    skills::{InnateSkill, HeroSkill},
};

/// Defines a HeroClass that contains info on base stats, allowed equipment, etc.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HeroClass {
    class: String,
    prerequisite: String,
    gold_hire_cost: u32,
    gem_hire_cost: u32,

    base_hp: Vec<f64>,
    base_atk: Vec<f64>,
    base_def: Vec<f64>,
    base_eva: f64,
    base_crit_chance: f64,
    base_crit_mult: f64,
    base_threat_rating: u16,

    element_type: String,
    equipment_allowed: [Vec<String>; 6],

    innate_skills: [String; 4],
}

pub fn _create_hero_class(
    class: String,
    prerequisite: String,
    gold_hire_cost: u32,
    gem_hire_cost: u32,

    base_hp: Vec<f64>,
    base_atk: Vec<f64>,
    base_def: Vec<f64>,
    base_eva: f64,
    base_crit_chance: f64,
    base_crit_mult: f64,
    base_threat_rating: u16,

    element_type: String,
    equipment_allowed: [Vec<String>; 6],

    innate_skills: [String; 4],
) -> HeroClass {
    return HeroClass {
        class,
        prerequisite,
        gold_hire_cost,
        gem_hire_cost,

        base_hp,
        base_atk,
        base_def,
        base_eva,
        base_crit_chance,
        base_crit_mult,
        base_threat_rating,

        element_type,
        equipment_allowed,

        innate_skills,
    };
}

/// Defines a Hero that contains info on base stats, equipment, and skills
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Hero {
    identifier: String,
    class: String,
    level: u8,
    rank: u8,
    innate_tier: u8,

    hp: f64,
    atk: f64,
    def: f64,
    eva: f64,
    crit_chance: f64,
    crit_mult: f64,
    threat_rating: u16,
    element_type: String,

    atk_modifier: f64,
    def_modifier: f64,

    hp_seeds: u8,
    atk_seeds: u8,
    def_seeds: u8,

    skills: [String; 4],

    equipment_equipped: [String; 6],
    equipment_quality: [String; 6],
    elements_socketed: [String; 6],
    spirits_socketed: [String; 6],
}

pub fn create_hero(
    identifier: String,
    class: String,
    level: u8,
    rank: u8,
    innate_tier: u8,

    hp: f64,
    atk: f64,
    def: f64,
    eva: f64,
    crit_chance: f64,
    crit_mult: f64,
    threat_rating: u16,
    element_type: String,

    atk_modifier: f64,
    def_modifier: f64,

    hp_seeds: u8,
    atk_seeds: u8,
    def_seeds: u8,

    skills: [String; 4],

    equipment_equipped: [String; 6],
    equipment_quality: [String; 6],
    elements_socketed: [String; 6],
    spirits_socketed: [String; 6],
) -> Hero {
    return Hero {
        identifier,
        class,
        level,
        rank,
        innate_tier,

        hp,
        atk,
        def,
        eva,
        crit_chance,
        crit_mult,
        threat_rating,
        element_type,

        atk_modifier,
        def_modifier,

        hp_seeds,
        atk_seeds,
        def_seeds,

        skills,

        equipment_equipped,
        equipment_quality,
        elements_socketed,
        spirits_socketed,
    };
}

impl Hero {
    pub fn validate_equipment(
        &self,
        bp_map: &HashMap<String, Blueprint>,
        hero_classes: &HashMap<String, HeroClass>,
    ) {
        if !hero_classes.contains_key(&self.class) {
            panic!(
                "Encountered unknown class {} for hero {}",
                self.class, self.identifier
            );
        }
        let class = hero_classes.get(&self.class).unwrap();

        for (i, equipment) in self.equipment_equipped.iter().enumerate() {
            if !bp_map.contains_key(equipment) {
                panic!(
                    "Equipment {} could not be validated as a known item",
                    equipment
                );
            }
            let blueprint = bp_map.get(equipment).unwrap();
            if !class.equipment_allowed[i].contains(&blueprint.get_type()) {
                panic!(
                    "Equipment {} is of type {} that is not allowed for this class in this slot (# {}). Valid options: {:#?}",
                    equipment,
                    blueprint.get_type(),
                    i,
                    class.equipment_allowed,
                )
            }
        }
    }

    pub fn calculate_innate_skill_name(&self, class_innate_skill_names_map: &HashMap<String, String>) -> String {
        if !class_innate_skill_names_map.contains_key(&self.class) {
            // Class not found in map
            panic!(
                "Class {} could not be found in keys for class_innate_skill_names_map",
                self.class
            );
        }

        let innate_skill = class_innate_skill_names_map[&self.class].clone();
        return innate_skill;
    }

    pub fn calculate_innate_tier(
        &mut self,
        class_innate_skill_names_map: &HashMap<String, String>,
        innate_skill_map: &HashMap<String, InnateSkill>,
    ) {
        let element_qty = self.calculate_element_qty();
        let innate_skill = self.calculate_innate_skill_name(class_innate_skill_names_map);

        let mut innate_skill_variants: Vec<&InnateSkill> = innate_skill_map
            .values()
            .filter(|is| {
                is.get_tier_1_name() == innate_skill && is.get_element_qty_req() < element_qty
            })
            .collect::<Vec<&InnateSkill>>();

        innate_skill_variants.sort_unstable_by_key(|is| is.get_skill_tier());

        println!("Innate_Skill_Variants: {:#?}", innate_skill_variants);

        let innate_skill_info = innate_skill_variants[innate_skill_variants.len() - 1];

        self.innate_tier = innate_skill_info.get_skill_tier();
    }

    pub fn calculate_element_qty(&self) -> u16 {
        let mut element_qty = 0u16;
        for element_string in &self.elements_socketed {
            let split_vec: Vec<&str> = element_string.split(" ").collect();
            if split_vec.len() < 2 {
                panic!(
                    "Element {} must conform to format [type] [grade: 1-4]",
                    element_string
                );
            }
            let element = split_vec[0];
            let grade = split_vec[1];
            if element == self.element_type {
                match grade {
                    "1" => element_qty += 5,
                    "2" => element_qty += 10,
                    "3" => element_qty += 15,
                    "4" => element_qty += 25,
                    _ => panic!("Unknown element grade {}", grade),
                }
            }
        }
        return element_qty;
    }

    pub fn calculate_spirit_qty(&self, spirit_name: String) -> u8 {
        let spirit_qty = u8::try_from(
            self.spirits_socketed
                .iter()
                .filter(|x| **x == spirit_name)
                .count(),
        )
        .unwrap_or_default();

        return spirit_qty;
    }

    pub fn calculate_attack_modifier(
        &mut self,
        hero_skill_map: &HashMap<String, HeroSkill>,
        class_innate_skill_names_map: &HashMap<String, String>,
        innate_skill_map: &HashMap<String, InnateSkill>,
    ) {
        let mut attack_modifier = 0.0f64;
        
        let innate_skill_name = self.calculate_innate_skill_name(class_innate_skill_names_map);
        let innate_skill = innate_skill_map[&innate_skill_name].clone();

        attack_modifier += innate_skill.get_attack_percent();

        for skill_name in &self.skills {
            if !hero_skill_map.contains_key(skill_name) {
                panic!(
                    "Skill {} could not be found in keys for hero_skill_map",
                    skill_name
                );
            }
            let skill = hero_skill_map[skill_name].clone();
            attack_modifier += skill.get_attack_percent();
        }

        self.atk_modifier = attack_modifier;
    }

    pub fn calculate_defense_modifier(
        &mut self,
        hero_skill_map: &HashMap<String, HeroSkill>,
        class_innate_skill_names_map: &HashMap<String, String>,
        innate_skill_map: &HashMap<String, InnateSkill>,
    ) {
        let mut defense_modifier = 0.0f64;
        
        let innate_skill_name = self.calculate_innate_skill_name(class_innate_skill_names_map);
        let innate_skill = innate_skill_map[&innate_skill_name].clone();

        defense_modifier += innate_skill.get_defense_percent();

        for skill_name in &self.skills {
            if !hero_skill_map.contains_key(skill_name) {
                panic!(
                    "Skill {} could not be found in keys for hero_skill_map",
                    skill_name
                );
            }
            let skill = hero_skill_map[skill_name].clone();
            defense_modifier += skill.get_defense_percent();
        }

        self.def_modifier = defense_modifier;
    }

    pub fn scale_by_class(&mut self, hero_classes: &HashMap<String, HeroClass>) {
        if !hero_classes.contains_key(&self.class) {
            panic!(
                "Encountered unknown class {} for hero {}",
                self.class, self.identifier
            );
        }
        let class = hero_classes.get(&self.class).unwrap();

        let level_index = usize::from(self.level - 1);
        self.hp = class.base_hp[level_index];
        self.atk = class.base_atk[level_index];
        self.def = class.base_def[level_index];
        self.eva = class.base_eva;
        self.crit_chance = class.base_crit_chance;
        self.crit_mult = class.base_crit_mult;
        self.element_type = class.element_type.to_string();
    }

    pub fn _round_floats_for_display(&self) -> Hero {
        let mut h2 = self.clone();
        h2.hp = round_to_2(h2.hp);
        h2.atk = round_to_2(h2.atk);
        h2.def = round_to_2(h2.def);
        h2.eva = round_to_2(h2.eva);
        h2.crit_chance = round_to_2(h2.crit_chance);
        h2.crit_mult = round_to_2(h2.crit_mult);
        return h2;
    }
}

impl From<Hero> for SimHero {
    /// Create a hero from the input object performing type validation and calculating certain fields
    fn from(item: Hero) -> Self {
        let i2 = item.clone();
        return create_sim_hero(
            item.identifier,
            item.class,
            item.level,
            item.rank,
            item.innate_tier,
            item.hp,
            item.atk,
            item.def,
            item.threat_rating,
            item.crit_chance,
            item.crit_mult,
            item.eva,
            i2.calculate_element_qty(),
            item.element_type,
            i2.calculate_spirit_qty(String::from("Armadillo")),
            i2.calculate_spirit_qty(String::from("Lizard")),
            i2.calculate_spirit_qty(String::from("Shark")),
            i2.calculate_spirit_qty(String::from("Dinosaur")),
            i2.calculate_spirit_qty(String::from("Mundra")),
            item.atk_modifier,
            item.def_modifier,
        )
        .unwrap();
    }
}

impl From<Hero> for HeroInput {
    fn from(item: Hero) -> Self {
        return create_hero_input(
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
            item.skills,
            item.equipment_equipped,
            item.equipment_quality,
            item.elements_socketed,
            item.spirits_socketed,
        );
    }
}
