use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    decimals::round_to_2,
    equipment::Blueprint,
    heroes::{create_sim_hero, SimHero},
    inputs::{create_hero_input, HeroInput},
    skills::{HeroSkill, InnateSkill},
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
    hp_regen: f64,
    atk: f64,
    def: f64,
    eva: f64,
    crit_chance: f64,
    crit_mult: f64,
    threat_rating: u16,
    element_type: String,
    element_qty: u16,
    survive_fatal_blow_chance: f64,

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
    hp_regen: f64,
    atk: f64,
    def: f64,
    eva: f64,
    crit_chance: f64,
    crit_mult: f64,
    threat_rating: u16,
    element_type: String,
    element_qty: u16,
    survive_fatal_blow_chance: f64,

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
        hp_regen,
        atk,
        def,
        eva,
        crit_chance,
        crit_mult,
        threat_rating,
        element_type,
        element_qty,
        survive_fatal_blow_chance,

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
    pub fn set_hero_skills(&mut self, new_skills: Vec<String>) {
        self.skills[0] = new_skills.get(0).unwrap_or(&String::from("")).to_string();
        self.skills[1] = new_skills.get(1).unwrap_or(&String::from("")).to_string();
        self.skills[2] = new_skills.get(2).unwrap_or(&String::from("")).to_string();
        self.skills[3] = new_skills.get(3).unwrap_or(&String::from("")).to_string();
    }

    pub fn validate_equipment(
        &mut self,
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

        let mut element_qty = 0u16;

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

            let split_vec = self.elements_socketed[i].split(" ").collect::<Vec<&str>>();
            if split_vec.len() < 2 {
                panic!(
                    "Element {} must conform to format [type] [grade: 1-4]",
                    self.elements_socketed[i]
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
                if element == blueprint.get_elemental_affinity() {
                    element_qty += 5;
                }
            } else {
                panic!("Unknown element type {}", element);
            }
        }

        self.element_qty = element_qty;
    }

    pub fn calculate_innate_skill_name(
        &self,
        class_innate_skill_names_map: &HashMap<String, String>,
    ) -> String {
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
        let innate_skill = self.calculate_innate_skill_name(class_innate_skill_names_map);

        let mut innate_skill_variants: Vec<&InnateSkill> = innate_skill_map
            .values()
            .filter(|is| {
                is.get_tier_1_name() == innate_skill && is.get_element_qty_req() < self.element_qty
            })
            .collect::<Vec<&InnateSkill>>();

        innate_skill_variants.sort_unstable_by_key(|is| is.get_skill_tier());

        let innate_skill_info = innate_skill_variants[innate_skill_variants.len() - 1];

        self.innate_tier = innate_skill_info.get_skill_tier();
    }

    /// Calculate skill tier and get the correct skill
    pub fn calculate_hero_skill_tier(
        &self,
        hero_skill_tier_1_name_map: &HashMap<String, String>,
        hero_skill_map: &HashMap<String, HeroSkill>,
        base_skill_name: String,
    ) -> (u8, HeroSkill) {
        if !hero_skill_map.contains_key(&base_skill_name) {
            panic!("Unknown skill name: {}", base_skill_name);
        }

        let mut skill = &hero_skill_map[&base_skill_name];
        let mut tier = skill.get_skill_tier();
        let mut checked_upgrade = false;

        loop {
            let tier_formatted_skill_name = f!("{} T{}", skill.get_tier_1_name(), tier);
            let tier_adjusted_skill_name =
                hero_skill_tier_1_name_map[&tier_formatted_skill_name].to_string();
            skill = &hero_skill_map[&tier_adjusted_skill_name];
            let skill_tier_ele_req = skill.get_element_qty_req();

            if self.element_qty < skill_tier_ele_req {
                // Tier too high, scale down
                tier -= 1;
                if checked_upgrade {
                    // Checked upgrade and failed, so revert and exit loop
                    break;
                }
            } else if self.element_qty > skill_tier_ele_req && tier < 4 {
                // May be able to upgrade tier
                tier += 1;
                checked_upgrade = true;
            } else {
                // Element qty == requirement, just exit
                break;
            }
        }

        let tier_formatted_skill_name = f!("{} T{}", skill.get_tier_1_name(), tier);
        let tier_adjusted_skill_name =
            hero_skill_tier_1_name_map[&tier_formatted_skill_name].to_string();

        return (tier, hero_skill_map[&tier_adjusted_skill_name].clone());
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

    // pub fn calculate_attack_modifier(
    //     &mut self,
    //     hero_skill_map: &HashMap<String, HeroSkill>,
    //     class_innate_skill_names_map: &HashMap<String, String>,
    //     innate_skill_map: &HashMap<String, InnateSkill>,
    // ) {
    //     let mut attack_modifier = 0.0f64;

    //     let innate_skill_name = self.calculate_innate_skill_name(class_innate_skill_names_map);
    //     let innate_skill = innate_skill_map[&innate_skill_name].clone();

    //     attack_modifier += innate_skill.get_attack_percent();

    //     for skill_name in &self.skills {
    //         if !hero_skill_map.contains_key(skill_name) {
    //             panic!(
    //                 "Skill {} could not be found in keys for hero_skill_map",
    //                 skill_name
    //             );
    //         }
    //         let skill = hero_skill_map[skill_name].clone();
    //         attack_modifier += skill.get_attack_percent();
    //     }

    //     self.atk_modifier = attack_modifier;
    // }

    // pub fn calculate_defense_modifier(
    //     &mut self,
    //     hero_skill_map: &HashMap<String, HeroSkill>,
    //     class_innate_skill_names_map: &HashMap<String, String>,
    //     innate_skill_map: &HashMap<String, InnateSkill>,
    // ) {
    //     let mut defense_modifier = 0.0f64;

    //     let innate_skill_name = self.calculate_innate_skill_name(class_innate_skill_names_map);
    //     let innate_skill = innate_skill_map[&innate_skill_name].clone();

    //     defense_modifier += innate_skill.get_defense_percent();

    //     for skill_name in &self.skills {
    //         if !hero_skill_map.contains_key(skill_name) {
    //             panic!(
    //                 "Skill {} could not be found in keys for hero_skill_map",
    //                 skill_name
    //             );
    //         }
    //         let skill = hero_skill_map[skill_name].clone();
    //         defense_modifier += skill.get_defense_percent();
    //     }

    //     self.def_modifier = defense_modifier;
    // }

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
        self.threat_rating = class.base_threat_rating;

        self.element_type = class.element_type.to_string();
    }

    pub fn calculate_stat_improvements_from_gear_and_skills(
        &mut self,
        bp_map: &HashMap<String, Blueprint>,
        hero_skill_tier_1_name_map: &HashMap<String, String>,
        hero_skill_map: &HashMap<String, HeroSkill>,
        class_innate_skill_names_map: &HashMap<String, String>,
        innate_skill_map: &HashMap<String, InnateSkill>,
    ) {
        let mut blueprints: Vec<Blueprint> = Default::default();
        for equip_name in &self.equipment_equipped {
            blueprints.push(bp_map[equip_name].clone());
        }

        let innate_skill_name = self.calculate_innate_skill_name(class_innate_skill_names_map);
        let innate_skill = innate_skill_map
            .values()
            .filter(|v| {
                v.get_tier_1_name() == innate_skill_name && v.get_skill_tier() == self.innate_tier
            })
            .collect::<Vec<&InnateSkill>>()[0];

        let mut equip_atk_value = 0.0f64;
        let mut equip_hp_value = 0.0f64;
        let mut equip_def_value = 0.0f64;
        let mut equip_eva_percent = 0.0f64;
        let mut equip_crit_chance_percent = 0.0f64;

        let mut spirit_bonus_atk_value: f64 = 0.0;
        let mut spirit_bonus_atk_percent: f64 = 0.0;
        let mut spirit_bonus_def_value: f64 = 0.0;
        let mut spirit_bonus_def_percent: f64 = 0.0;
        let mut spirit_bonus_hp_value: f64 = 0.0;
        let mut spirit_bonus_hp_percent: f64 = 0.0;
        let mut spirit_bonus_hp_regen_value: f64 = 0.0;
        let mut spirit_bonus_eva_percent: f64 = 0.0;
        let mut spirit_bonus_crit_dmg_percent: f64 = 0.0;
        let mut spirit_bonus_crit_chance_percent: f64 = 0.0;
        let mut spirit_bonus_threat_rating_value: u16 = 0;
        let _spirit_bonus_rest_time_percent: f64 = 0.0;
        let _spirit_bonus_xp_percent: f64 = 0.0;
        let mut spirit_bonus_survive_fatal_blow_chance_percent = 0.0f64;

        // Calculate gear bonuses
        for (gear_index, blueprint) in blueprints.iter().enumerate() {
            let mut bonus_item_all_stats_percent = 0.0f64;
            let mut bonus_item_atk_percent = 0.0f64;
            let mut bonus_item_def_percent = 0.0f64;

            // Check for bonus stats from innate skill
            bonus_item_all_stats_percent +=
                innate_skill.get_bonus_stats_from_all_equipment_percent();

            if innate_skill.get_item_types().len() > 0 {
                // Has bonuses associated with atleast one item type
                for itype in innate_skill.get_item_types() {
                    if blueprint.get_type() == itype {
                        // Have that type equipped, apply bonus(es)
                        bonus_item_atk_percent += innate_skill.get_attack_with_item_percent();
                        bonus_item_def_percent += innate_skill.get_defense_with_item_percent();
                        bonus_item_all_stats_percent +=
                            innate_skill.get_all_stats_with_item_percent();
                    }
                }
            }

            // Check for skills that give bonus stats to gear
            for skill_name in &self.skills {
                if skill_name == "" {
                    continue;
                }
                if !hero_skill_map.contains_key(skill_name) {
                    panic!(
                        "Skill {} could not be found in keys for hero_skill_map",
                        skill_name
                    );
                }

                // Calculate skill tier and get the correct skill
                let (_, skill) = self.calculate_hero_skill_tier(
                    hero_skill_tier_1_name_map,
                    hero_skill_map,
                    skill_name.to_string(),
                );

                // Get all stats bonus if applicable
                bonus_item_all_stats_percent += skill.get_bonus_stats_from_all_equipment_percent();

                if skill.get_item_types().len() > 0 {
                    // Has bonuses associated with atleast one item type
                    for itype in skill.get_item_types() {
                        if blueprint.get_type() == itype {
                            // Have that type equipped, apply bonus(es)
                            bonus_item_atk_percent += skill.get_attack_with_item_percent();
                            bonus_item_def_percent += skill.get_defense_with_item_percent();
                        }
                    }
                }
            }

            let gear_quality = self.equipment_quality[gear_index].as_str();
            let gear_quality_bonus: f64;
            match gear_quality {
                "Normal" => gear_quality_bonus = 1.0,
                "Superior" => gear_quality_bonus = 1.25,
                "Flawless" => gear_quality_bonus = 1.5,
                "Epic" => gear_quality_bonus = 2.0,
                "Legendary" => gear_quality_bonus = 3.0,
                _ => panic!("Unknown gear_quality {}", gear_quality),
            }

            let gear_element = &self.elements_socketed[gear_index];
            let gear_element_split = gear_element.split_whitespace().collect::<Vec<&str>>();
            let gear_element_tier = gear_element_split[1].parse::<u8>().unwrap();

            let mut gear_element_atk_bonus: f64;
            let mut gear_element_def_bonus: f64;
            let mut gear_element_hp_bonus: f64;

            match gear_element_tier {
                1u8 => {
                    // Check 5 / Tier 5 (Luxurious)
                    if *gear_element == String::from("Luxurious 1") {
                        gear_element_atk_bonus = 26.0;
                        gear_element_def_bonus = 18.0;
                        gear_element_hp_bonus = 5.0;
                    } else {
                        gear_element_atk_bonus = 14.0;
                        gear_element_def_bonus = 10.0;
                        gear_element_hp_bonus = 3.0;
                    }
                }
                2u8 => {
                    gear_element_atk_bonus = 38.0;
                    gear_element_def_bonus = 25.0;
                    gear_element_hp_bonus = 8.0;
                }
                3u8 => {
                    // Check 15 / Tier 10 (Opulent)
                    if *gear_element == String::from("Opulent 3") {
                        gear_element_atk_bonus = 63.0;
                        gear_element_def_bonus = 42.0;
                        gear_element_hp_bonus = 13.0;
                    } else {
                        gear_element_atk_bonus = 48.0;
                        gear_element_def_bonus = 32.0;
                        gear_element_hp_bonus = 10.0;
                    }
                }
                4u8 => {
                    gear_element_atk_bonus = 89.0;
                    gear_element_def_bonus = 59.0;
                    gear_element_hp_bonus = 18.0;
                }
                _ => panic!("Unknown gear_element_tier {}", gear_element_tier),
            }
            let element_affinity = blueprint.get_elemental_affinity();
            if element_affinity.as_str() == gear_element_split[0] {
                gear_element_atk_bonus *= 1.5;
                gear_element_def_bonus *= 1.5;
                gear_element_hp_bonus *= 1.5;
            }

            let gear_spirit = &self.spirits_socketed[gear_index];
            let gear_spirit_split = gear_spirit.split_whitespace().collect::<Vec<&str>>();
            let gear_spirit_name = gear_spirit_split[0];
            let gear_spirit_tier = gear_spirit_split[1];

            let spirit_affinity = blueprint.get_spirit_affinity();

            let mut gear_spirit_atk_bonus: f64;
            let mut gear_spirit_def_bonus: f64;
            let mut gear_spirit_hp_bonus: f64;

            match gear_spirit_tier {
                "T4" => {
                    // Low-Tier Spirits
                    gear_spirit_atk_bonus = 16.0;
                    gear_spirit_def_bonus = 11.0;
                    gear_spirit_hp_bonus = 3.0;
                }
                "T5" => {
                    // Xolotl Spirit
                    gear_spirit_atk_bonus = 26.0;
                    gear_spirit_def_bonus = 18.0;
                    gear_spirit_hp_bonus = 5.0;
                }
                "T7" => {
                    // Mid-Tier Spirits
                    gear_spirit_atk_bonus = 41.0;
                    gear_spirit_def_bonus = 27.0;
                    gear_spirit_hp_bonus = 8.0;
                }
                "T9" => {
                    // High-Tier Spirits
                    gear_spirit_atk_bonus = 48.0;
                    gear_spirit_def_bonus = 32.0;
                    gear_spirit_hp_bonus = 10.0;
                }
                "TM" => {
                    // Mundra Spirit
                    gear_spirit_atk_bonus = 50.0;
                    gear_spirit_def_bonus = 33.0;
                    gear_spirit_hp_bonus = 10.0;
                }
                "T11" => {
                    // Quetzalcoatl Spirit
                    gear_spirit_atk_bonus = 63.0;
                    gear_spirit_def_bonus = 42.0;
                    gear_spirit_hp_bonus = 13.0; // only gives 10 on banana gun T6? only 6 on T5 imperial scutum? 10 on T5 silver thistle?? must be the min stuff from ress' sheet
                }
                "T12" => {
                    // Max-Tier Spirits
                    gear_spirit_atk_bonus = 89.0;
                    gear_spirit_def_bonus = 59.0;
                    gear_spirit_hp_bonus = 18.0;
                }
                _ => panic!("Unknown gear_spirit_tier {}", gear_spirit_tier),
            }

            let spirit_affinity_split: &str;
            if spirit_affinity.as_str() != "---" {
                spirit_affinity_split = spirit_affinity
                    .as_str()
                    .split_whitespace()
                    .collect::<Vec<&str>>()[0];
            } else {
                spirit_affinity_split = "NO_SPIRIT_AFFINITY";
            }

            match gear_spirit_name {
                "Armadillo" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_survive_fatal_blow_chance_percent += 0.25;
                    } else {
                        spirit_bonus_survive_fatal_blow_chance_percent += 0.15;
                    }
                }
                "Rhino" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_threat_rating_value += 10;
                    } else {
                        spirit_bonus_threat_rating_value += 5;
                    }
                }
                "Lizard" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_hp_regen_value += 5.0;
                    } else {
                        spirit_bonus_hp_regen_value += 3.0;
                    }
                }
                "Wolf" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_atk_percent += 0.1;
                    } else {
                        spirit_bonus_atk_percent += 0.05;
                    }
                }
                "Ram" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_def_percent += 0.1;
                    } else {
                        spirit_bonus_def_percent += 0.05;
                    }
                }
                "Eagle" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_crit_chance_percent += 0.03;
                    } else {
                        spirit_bonus_crit_chance_percent += 0.02;
                    }
                }
                "Ox" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_hp_percent += 0.05;
                    } else {
                        spirit_bonus_hp_percent += 0.03;
                    }
                }
                "Viper" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_crit_dmg_percent += 0.2;
                    } else {
                        spirit_bonus_crit_dmg_percent += 0.15;
                    }
                }
                "Cat" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_eva_percent += 0.03;
                    } else {
                        spirit_bonus_eva_percent += 0.02;
                    }
                }
                "Bear" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_atk_percent += 0.07;
                        spirit_bonus_hp_value += 20.0;
                    } else {
                        spirit_bonus_atk_percent += 0.05;
                        spirit_bonus_hp_value += 15.0;
                    }
                }
                "Walrus" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_hp_percent += 0.08;
                    } else {
                        spirit_bonus_hp_percent += 0.05;
                    }
                }
                "Mammoth" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_def_percent += 0.13;
                        spirit_bonus_threat_rating_value += 15;
                    } else {
                        spirit_bonus_def_percent += 0.1;
                        spirit_bonus_threat_rating_value += 10;
                    }
                }
                "Lion" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_atk_percent += 0.07;
                        spirit_bonus_eva_percent += 0.02;
                    } else {
                        spirit_bonus_atk_percent += 0.05;
                        spirit_bonus_eva_percent += 0.01;
                    }
                }
                "Tiger" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_def_percent += 0.07;
                        spirit_bonus_eva_percent += 0.02;
                    } else {
                        spirit_bonus_def_percent += 0.05;
                        spirit_bonus_eva_percent += 0.01;
                    }
                }
                "Phoenix" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_hp_percent += 0.05;
                        spirit_bonus_hp_regen_value += 5.0;
                    } else {
                        spirit_bonus_hp_percent += 0.04;
                        spirit_bonus_hp_regen_value += 3.0;
                    }
                }
                "Hydra" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_def_value += 125.0;
                        spirit_bonus_hp_value += 35.0;
                    } else {
                        spirit_bonus_def_value += 100.0;
                        spirit_bonus_hp_value += 25.0;
                    }
                }
                "Tarrasque" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_def_percent += 0.25;
                    } else {
                        spirit_bonus_def_percent += 0.2;
                    }
                }
                "Carbuncle" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_crit_chance_percent += 0.03;
                        spirit_bonus_eva_percent += 0.03;
                    } else {
                        spirit_bonus_crit_chance_percent += 0.02;
                        spirit_bonus_eva_percent += 0.02;
                    }
                }
                "Chimera" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_atk_percent += 0.15;
                        spirit_bonus_crit_dmg_percent += 0.15;
                    } else {
                        spirit_bonus_atk_percent += 0.1;
                        spirit_bonus_crit_dmg_percent += 0.1;
                    }
                }
                "Kraken" => {
                    if spirit_affinity_split == gear_spirit_name {
                        spirit_bonus_atk_value += 125.0;
                        spirit_bonus_atk_percent += 0.15;
                    } else {
                        spirit_bonus_atk_value += 100.0;
                        spirit_bonus_atk_percent += 0.1;
                    }
                }
                _ => (),
            }

            if spirit_affinity_split == gear_spirit_name {
                gear_spirit_atk_bonus *= 1.5;
                gear_spirit_def_bonus *= 1.5;
                gear_spirit_hp_bonus *= 1.5;
            }

            let spellknight_bonus: f64;
            // Items from chests have innate elements, and official data sheet doesn't have innate element as a field, so this is the best we've got
            if blueprint.get_unlock_prerequisite().contains("Chest") {
                spellknight_bonus =
                    1.0 + innate_skill.get_all_stats_for_equipment_with_innate_element_percent();
            } else {
                spellknight_bonus = 1.0;
            }

            // Calculate and apply gear bonus to running totals
            let item_attack_final = ((blueprint.get_atk() * gear_quality_bonus)
                + f64::min(gear_element_atk_bonus, blueprint.get_atk())
                + f64::min(gear_spirit_atk_bonus, blueprint.get_atk()))
                * (1.0 + bonus_item_atk_percent + bonus_item_all_stats_percent)
                * spellknight_bonus;
            let item_defense_final = ((blueprint.get_def() * gear_quality_bonus)
                + f64::min(gear_element_def_bonus, blueprint.get_def())
                + f64::min(gear_spirit_def_bonus, blueprint.get_def()))
                * (1.0 + bonus_item_def_percent + bonus_item_all_stats_percent)
                * spellknight_bonus;
            let item_hp_final = ((blueprint.get_hp() * gear_quality_bonus)
                + f64::min(gear_element_hp_bonus, blueprint.get_hp())
                + f64::min(gear_spirit_hp_bonus, blueprint.get_hp()))
                * (1.0 + bonus_item_all_stats_percent)
                * spellknight_bonus;
            // bonus_atk_value += blueprint.get_atk() * gear_quality_bonus * (1.0 + bonus_item_atk_percent + bonus_item_all_stats_percent);
            // bonus_def_value += blueprint.get_def() * gear_quality_bonus * (1.0 + bonus_item_def_percent + bonus_item_all_stats_percent);
            // bonus_hp_value += blueprint.get_hp() * gear_quality_bonus * (1.0 + bonus_item_all_stats_percent);
            equip_atk_value += item_attack_final;
            equip_def_value += item_defense_final;
            equip_hp_value += item_hp_final;
            equip_eva_percent += blueprint.get_eva() * (1.0 + bonus_item_all_stats_percent);
            equip_crit_chance_percent +=
                blueprint.get_crit() * (1.0 + bonus_item_all_stats_percent);
        }

        // Calculate hero-wide skill bonuses
        let mut skill_bonus_atk_percent: f64 = 0.0;
        let mut skill_bonus_atk_value: f64 = 0.0;
        let mut skill_bonus_hp_percent: f64 = 0.0;
        let mut skill_bonus_hp_value: f64 = 0.0;
        let mut skill_bonus_hp_regen_value: f64 = 0.0;
        let mut skill_bonus_def_percent: f64 = 0.0;
        let mut skill_bonus_eva_percent: f64 = 0.0;
        let mut skill_bonus_crit_chance_percent: f64 = 0.0;
        let mut skill_bonus_crit_damage_percent: f64 = 0.0;
        let mut skill_bonus_threat_rating_value: u16 = 0;
        let mut _skill_bonus_rest_time_percent: f64 = 0.0;
        let mut _skill_bonus_xp_percent_percent: f64 = 0.0;
        let mut skill_bonus_survive_fatal_blow_chance_percent: f64 = 0.0;

        // Get bonuses from innate skill
        skill_bonus_atk_percent += innate_skill.get_attack_percent();
        skill_bonus_hp_percent += innate_skill.get_hp_percent();
        skill_bonus_hp_value += innate_skill.get_hp_value();
        skill_bonus_hp_regen_value += innate_skill.get_hp_regen_value();
        skill_bonus_def_percent += innate_skill.get_defense_percent();
        skill_bonus_eva_percent += innate_skill.get_evasion_percent();
        skill_bonus_crit_chance_percent += innate_skill.get_crit_chance_percent();
        skill_bonus_crit_damage_percent += innate_skill.get_crit_damage_percent();
        skill_bonus_threat_rating_value += innate_skill.get_threat_rating_value();
        _skill_bonus_rest_time_percent += innate_skill.get_rest_time_percent();

        // Get bonuses from hero skills
        for skill_name in &self.skills {
            if skill_name == "" {
                continue;
            }
            if !hero_skill_map.contains_key(skill_name) {
                panic!(
                    "Skill {} could not be found in keys for hero_skill_map",
                    skill_name
                );
            }

            // Calculate skill tier and get the correct skill
            let (_, skill) = self.calculate_hero_skill_tier(
                hero_skill_tier_1_name_map,
                hero_skill_map,
                skill_name.to_string(),
            );

            skill_bonus_atk_percent += skill.get_attack_percent();
            skill_bonus_atk_value += skill.get_attack_value();
            skill_bonus_hp_percent += skill.get_hp_percent();
            skill_bonus_hp_value += skill.get_hp_value();
            skill_bonus_def_percent += skill.get_defense_percent();
            skill_bonus_eva_percent += skill.get_evasion_percent();
            skill_bonus_crit_chance_percent += skill.get_crit_chance_percent();
            skill_bonus_crit_damage_percent += skill.get_crit_damage_percent();
            _skill_bonus_rest_time_percent += skill.get_rest_time_percent();
            _skill_bonus_xp_percent_percent += skill.get_xp_percent();
            skill_bonus_survive_fatal_blow_chance_percent +=
                skill.get_survive_fatal_blow_chance_percent();
        }

        // Adjust threat_rating
        let final_threat_rating =
            self.threat_rating + skill_bonus_threat_rating_value + spirit_bonus_threat_rating_value;
        self.threat_rating = final_threat_rating;

        let mut geo_astramancer_element_qty_or_chieftain_threat_bonus: f64 = 0.0;
        match self.class.as_str() {
            "Geomancer" => {
                geo_astramancer_element_qty_or_chieftain_threat_bonus = f64::from(self.element_qty)
            }
            "Astramancer" => {
                geo_astramancer_element_qty_or_chieftain_threat_bonus = f64::from(self.element_qty)
            }
            "Chieftain" => {
                geo_astramancer_element_qty_or_chieftain_threat_bonus =
                    0.4 * f64::from(self.threat_rating)
            }
            _ => (),
        }

        // println!("--{}--", self.identifier);
        // ATK calc
        let base_atk = self.atk;
        let seeded_atk = base_atk + f64::from(self.atk_seeds * 4);
        let summarized_base_atk_value = seeded_atk + spirit_bonus_atk_value + skill_bonus_atk_value;
        let summarized_atk_percent_modifier = 1.0
            + skill_bonus_atk_percent
            + geo_astramancer_element_qty_or_chieftain_threat_bonus
            + spirit_bonus_atk_percent;
        let modified_atk_value = summarized_base_atk_value * summarized_atk_percent_modifier;
        let modified_atk_gear_value = equip_atk_value * summarized_atk_percent_modifier;
        let final_atk = modified_atk_value + modified_atk_gear_value;
        self.atk = final_atk;
        // println!("final_atk: {}", final_atk);
        // ((seeded_atk + gear_spirit_bonus_atk_value + sum(skill_bonus_atk_value)) * (1 + ((skill_atk_percent + geo_astramancer_element_qty_or_chieftain_threat_bonus) + bonus_spirit_atk_percent)/100)) + (bonus_atk_value * (1 + ((skill_atk_percent + geo_astramancer_element_qty_or_chieftain_threat_bonus) + bonus_spirit_atk_percent)/100)))

        // ATK mod calc
        let final_atk_mod = skill_bonus_atk_percent
            + geo_astramancer_element_qty_or_chieftain_threat_bonus
            + spirit_bonus_atk_percent;
        self.atk_modifier = final_atk_mod;

        // DEF
        let base_def = self.def;
        let seeded_def = base_def + f64::from(self.def_seeds * 4);
        let final_def = (seeded_def + equip_def_value + spirit_bonus_def_value)
            * (1.0 + skill_bonus_def_percent + spirit_bonus_def_percent);
        self.def = final_def;
        // println!("final_def: {}", final_def);

        // DEF mod
        let final_def_mod = skill_bonus_atk_percent + spirit_bonus_def_percent;
        self.def_modifier = final_def_mod;

        // HP
        let base_hp = self.hp;
        let seeded_hp = base_hp + f64::from(self.hp_seeds);
        let final_hp = (seeded_hp + equip_hp_value + skill_bonus_hp_value + spirit_bonus_hp_value)
            * (1.0 + skill_bonus_hp_percent + spirit_bonus_hp_percent);
        self.hp = final_hp;
        // println!("final_hp: {}", final_hp);

        // HP Regen
        let final_hp_regen = skill_bonus_hp_regen_value + spirit_bonus_hp_regen_value;
        self.hp_regen = final_hp_regen;

        // Other Stats
        // EVA
        let final_eva =
            self.eva + equip_eva_percent + skill_bonus_eva_percent + spirit_bonus_eva_percent;
        self.eva = final_eva;

        // Crit Chance
        let final_crit_chance = self.crit_chance
            + equip_crit_chance_percent
            + skill_bonus_crit_chance_percent
            + spirit_bonus_crit_chance_percent;
        self.crit_chance = final_crit_chance;

        // Crit Damage
        let final_crit_damage =
            self.crit_mult + skill_bonus_crit_damage_percent + spirit_bonus_crit_dmg_percent;
        self.crit_mult = final_crit_damage;

        // Rest Time
        // let final_rest_time = self.rest_time + skill_bonus_rest_time_percent + spirit_bonus_rest_time_percent;
        // self.rest_time = final_rest_time;

        // XP Percent
        // let final_xp = self.xp + skill_bonus_xp_percent + spirit_bonus_xp_percent;
        // self.xp = final_xp;

        // Survive Fatal Blow Chance
        let final_survive_fatal_blow_chance = self.survive_fatal_blow_chance
            + skill_bonus_survive_fatal_blow_chance_percent
            + spirit_bonus_survive_fatal_blow_chance_percent;
        self.survive_fatal_blow_chance = final_survive_fatal_blow_chance;

        // println!("\n");
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
            item.hp_regen,
            item.atk,
            item.def,
            item.threat_rating,
            item.crit_chance,
            item.crit_mult,
            item.eva,
            item.survive_fatal_blow_chance,
            item.element_qty,
            item.element_type,
            i2.calculate_spirit_qty(String::from("Armadillo T7")),
            i2.calculate_spirit_qty(String::from("Lizard T7")),
            i2.calculate_spirit_qty(String::from("Shark T9")),
            i2.calculate_spirit_qty(String::from("Dinosaur T9")),
            i2.calculate_spirit_qty(String::from("Mundra T10")),
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
