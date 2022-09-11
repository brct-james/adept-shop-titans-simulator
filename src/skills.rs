use serde::{Deserialize, Serialize};

/// Information on hero skills
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HeroSkill {
    name: String,
    type_: String,
    skill_tier: u8,
    rarity: String,
    element_qty_req: u16,
    tier_1_name: String,
    requires_class_promotion: bool,
    incompatible_with_t1_name: String,
    attack_percent: f64,
    attack_value: f64,
    hp_percent: f64,
    hp_value: f64,
    defense_percent: f64,
    evasion_percent: f64,
    crit_chance_percent: f64,
    crit_damage_percent: f64,
    rest_time_percent: f64,
    xp_percent: f64,
    survive_fatal_blow_chance_percent: f64,
    bonus_stats_from_all_equipment_percent: f64,
    break_chance_with_all_equipment_percent: f64,
    attack_with_item_percent: f64,
    defense_with_item_percent: f64,
    item_types: Vec<String>,
    classes_allowed: Vec<String>,
}

impl HeroSkill {
    pub fn _get_type(&self) -> String {
        return self.type_.to_string();
    }
}

pub fn create_hero_skill(
    name: String,
    type_: String,
    skill_tier: u8,
    rarity: String,
    element_qty_req: u16,
    tier_1_name: String,
    requires_class_promotion: bool,
    incompatible_with_t1_name: String,
    attack_percent: f64,
    attack_value: f64,
    hp_percent: f64,
    hp_value: f64,
    defense_percent: f64,
    evasion_percent: f64,
    crit_chance_percent: f64,
    crit_damage_percent: f64,
    rest_time_percent: f64,
    xp_percent: f64,
    survive_fatal_blow_chance_percent: f64,
    bonus_stats_from_all_equipment_percent: f64,
    break_chance_with_all_equipment_percent: f64,
    attack_with_item_percent: f64,
    defense_with_item_percent: f64,
    item_types: Vec<String>,
    classes_allowed: Vec<String>,
) -> HeroSkill {
    return HeroSkill {
        name,
        type_,
        skill_tier,
        rarity,
        element_qty_req,
        tier_1_name,
        requires_class_promotion,
        incompatible_with_t1_name,
        attack_percent,
        attack_value,
        hp_percent,
        hp_value,
        defense_percent,
        evasion_percent,
        crit_chance_percent,
        crit_damage_percent,
        rest_time_percent,
        xp_percent,
        survive_fatal_blow_chance_percent,
        bonus_stats_from_all_equipment_percent,
        break_chance_with_all_equipment_percent,
        attack_with_item_percent,
        defense_with_item_percent,
        item_types,
        classes_allowed,
    };
}

/// Information on innate skills
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InnateSkill {
    name: String,
    type_: String,
    skill_tier: u8,
    // description
    element_qty_req: u16,
    tier_1_name: String,
    requires_class_promotion: bool,
    attack_percent: f64,
    hp_percent: f64,
    hp_value: f64,
    hp_regen_value: f64,
    defense_percent: f64,
    evasion_percent: f64,
    crit_chance_percent: f64,
    crit_damage_percent: f64,
    threat_rating_value: f64,
    rest_time_percent: f64,
    bonus_stats_from_all_equipment_percent: f64,
    all_stats_for_equipment_with_innate_element_percent: f64,
    all_stats_with_item_percent: f64,
    attack_with_item_percent: f64,
    defense_with_item_percent: f64,
    item_types: Vec<String>,
    classes_allowed: Vec<String>,
}

impl InnateSkill {
    pub fn _get_type(&self) -> String {
        return self.type_.to_string();
    }
}

pub fn create_innate_skill(
    name: String,
    type_: String,
    skill_tier: u8,
    element_qty_req: u16,
    tier_1_name: String,
    requires_class_promotion: bool,
    attack_percent: f64,
    hp_percent: f64,
    hp_value: f64,
    hp_regen_value: f64,
    defense_percent: f64,
    evasion_percent: f64,
    crit_chance_percent: f64,
    crit_damage_percent: f64,
    threat_rating_value: f64,
    rest_time_percent: f64,
    bonus_stats_from_all_equipment_percent: f64,
    all_stats_for_equipment_with_innate_element_percent: f64,
    all_stats_with_item_percent: f64,
    attack_with_item_percent: f64,
    defense_with_item_percent: f64,
    item_types: Vec<String>,
    classes_allowed: Vec<String>,
) -> InnateSkill {
    return InnateSkill {
        name,
        type_,
        skill_tier,
        element_qty_req,
        tier_1_name,
        requires_class_promotion,
        attack_percent,
        hp_percent,
        hp_value,
        hp_regen_value,
        defense_percent,
        evasion_percent,
        crit_chance_percent,
        crit_damage_percent,
        threat_rating_value,
        rest_time_percent,
        bonus_stats_from_all_equipment_percent,
        all_stats_for_equipment_with_innate_element_percent,
        all_stats_with_item_percent,
        attack_with_item_percent,
        defense_with_item_percent,
        item_types,
        classes_allowed,
    };
}
