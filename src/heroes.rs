use super::equipment::{BoosterType, ElementType};

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

/// One or more Heroes fighting together in a dungeon and what booster they have
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Team {
    heroes: Vec<Hero>,
    booster: Option<BoosterType>,
    num_fighters: u8,
    num_rogues: u8,
    num_spellcasters: u8,
    num_tricksters: u8,
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

    pub fn calculate_damage_from_encounter(&mut self, defense_cap: f64, damage: f64) {
        // Calc the amount of damage taken by each hero in encounter
        for hero in &mut self.heroes {
            if hero.defense <= defense_cap / 6.0 {
                hero.damage_taken = 1.5 * damage
                    + ((hero.defense - 0.0) / (defense_cap / 6.0 - 0.0))
                        * (0.5 * damage - 1.5 * damage);
            } else if hero.defense <= defense_cap / 3.0 {
                hero.damage_taken = 0.5 * damage
                    + ((hero.defense - defense_cap / 6.0)
                        / (defense_cap / 3.0 - defense_cap / 6.0))
                        * (0.3 * damage - 0.5 * damage);
            } else {
                hero.damage_taken = 0.3 * damage
                    + ((hero.defense - defense_cap / 3.0) / (defense_cap - defense_cap / 3.0))
                        * (0.25 * damage - 0.3 * damage);
            }
            hero.crit_damage_taken = f64::max(hero.damage_taken, damage) * 1.5;
        }
    }

    pub fn apply_champion_and_booster_bonuses(&mut self, is_boss: bool) -> (f64, bool, f64, u8) {
        let (champion, champion_innate_tier) = self.get_champion_info();
        let (num_spellcasters, num_rogues, num_fighters, num_tricksters) =
            self.get_num_archetypes();

        let mut champion_attack_bonus = 0f64;
        let mut champion_defense_bonus = 0f64;

        let mut hemma_mult = 0f64;
        let mut count_loot = false;
        let mut loot_chance = 0f64;
        let mut polonia_loot_cap = 20;

        let mut booster_attack_bonus = 0f64;
        let mut booster_defense_bonus = 0f64;

        match champion.as_str() {
            "Argon" => {
                champion_attack_bonus = 0.1f64 * f64::from(champion_innate_tier);
                champion_defense_bonus = champion_attack_bonus;
            }
            "Ashley" => {
                champion_attack_bonus = 0.05 + 0.05 * f64::from(champion_innate_tier);
                if is_boss {
                    champion_attack_bonus = champion_attack_bonus * 2.0;
                }
                champion_defense_bonus = champion_attack_bonus;
            }
            "Donovan" => {
                match champion_innate_tier {
                    1u8 => {
                        champion_attack_bonus = 0.05 * f64::from(num_spellcasters);
                    }
                    2u8 => {
                        champion_attack_bonus = 0.08 * f64::from(num_spellcasters);
                    }
                    3u8 => {
                        champion_attack_bonus = 0.10 * f64::from(num_spellcasters);
                    }
                    4u8 => {
                        champion_attack_bonus = 0.14 * f64::from(num_spellcasters);
                    }
                    _ => (),
                }

                for hero in &mut self.heroes {
                    hero.hp = hero.hp
                        * (1.0
                            + (0.04
                                + 0.01 * f64::from(champion_innate_tier)
                                + 0.02 * f64::from(std::cmp::max(champion_innate_tier - 3, 0)))
                                * f64::from(num_fighters));
                    hero.critical_chance = hero.critical_chance
                        + (0.02
                            + 0.01 * f64::from(champion_innate_tier)
                            + 0.01 * f64::from(std::cmp::max(champion_innate_tier - 3, 0)))
                            * f64::from(num_rogues);
                    hero.evasion = hero.evasion
                        + (0.02
                            + 0.01 * f64::from(champion_innate_tier)
                            + 0.01 * f64::from(std::cmp::max(champion_innate_tier - 3, 0)))
                            * f64::from(num_rogues);
                    if hero.class == "Mercenary" {
                        // it looks like mercenaries get an extra 1.25x cause of the +25% effect from champ skills
                        hero.hp *= 1.25;
                        hero.critical_chance *= 1.25;
                        hero.evasion *= 1.25;
                    }
                }
            }
            "Hemma" => {
                for hero in &mut self.heroes {
                    hero.hp = hero.hp
                        * (1.0
                            + 0.05 * f64::from(champion_innate_tier)
                            + 0.05 * f64::from(std::cmp::max(champion_innate_tier - 3, 0)));
                    if hero.class == "Mercenary" {
                        hero.hp *= 1.328;
                    }
                }
                hemma_mult = 0.04 + f64::from(champion_innate_tier) * 0.02;
            }
            "Lilu" => {
                for hero in &mut self.heroes {
                    hero.hp = hero.hp * (1.05 + 0.05 * f64::from(champion_innate_tier));
                    if hero.class == "Mercenary" {
                        hero.hp *= 1.25;
                    }
                }
            }
            "Polonia" => {
                champion_defense_bonus = 0.05 + 0.05 * f64::from(champion_innate_tier);
                for hero in &mut self.heroes {
                    if champion_innate_tier < 3 {
                        hero.evasion = hero.evasion + 0.05 * 1.25;
                    } else {
                        hero.evasion = hero.evasion + 0.1 * 1.25;
                    }

                    if hero.class == "Mercenary" {
                        hero.evasion *= 1.25;
                    }
                }
                count_loot = true;
                match champion_innate_tier {
                    1u8 => loot_chance = 0.3,
                    2u8 => loot_chance = 0.35,
                    3u8 => loot_chance = 0.4,
                    4u8 => loot_chance = 0.5,
                    _ => (),
                }
                loot_chance = loot_chance + f64::from(num_tricksters) * 0.02;
                polonia_loot_cap = polonia_loot_cap + num_tricksters * 2;
            }
            "Sia" => {
                champion_attack_bonus = 0.05 + 0.05 * f64::from(champion_innate_tier);
            }
            "Yami" => {
                for hero in &mut self.heroes {
                    hero.critical_chance =
                        hero.critical_chance + 0.05 * f64::from(champion_innate_tier);
                    hero.evasion = hero.evasion + 0.05 * f64::from(champion_innate_tier);
                    if hero.class == "Mercenary" {
                        hero.critical_chance *= 1.25;
                        hero.evasion *= 1.25;
                    }
                }
            }
            _ => (),
        }

        // Calculate Booster Bonuses
        match self.booster {
            Some(booster_type) => match booster_type {
                BoosterType::MegaPowerBooster => {
                    booster_attack_bonus = 0.8;
                    booster_defense_bonus = 0.8;
                    for hero in &mut self.heroes {
                        hero.critical_chance += 0.25;
                        hero.critical_multiplier += 0.5;
                    }
                }
                BoosterType::SuperPowerBooster => {
                    booster_attack_bonus = 0.4;
                    booster_defense_bonus = 0.4;
                    for hero in &mut self.heroes {
                        hero.critical_chance += 0.1;
                    }
                }
                BoosterType::PowerBooster => {
                    booster_attack_bonus = 0.2;
                    booster_defense_bonus = 0.2;
                }
            },
            _ => (),
        }

        // Apply Champion and Booster Bonuses
        for hero in &mut self.heroes {
            if hero.class == "Mercenary" {
                champion_attack_bonus *= 1.25;
                champion_defense_bonus *= 1.25;
            }
            hero.attack = hero.attack * (1.0 + champion_attack_bonus + booster_attack_bonus);
            hero.defense = hero.defense * (1.0 + champion_defense_bonus + booster_defense_bonus);
        }

        return (hemma_mult, count_loot, loot_chance, polonia_loot_cap);
    }

    pub fn initialize_survive_chance_hemma_guaranteed_crit_and_berserker_stage(&mut self) {
        for hero in &mut self.heroes {
            hero.survive_chance = f64::from(hero.armadillo_qty) * 15.0 / 100.0;
            if hero.class == "Cleric" || hero.class == "Bishop" {
                hero.survive_chance = 1.2;
            }

            if hero.class == "Hemma" {
                hero.hemma_bonus = 0.0;
            }

            hero.berserker_stage = 0;
            hero.guaranteed_crit = false;
        }
    }

    pub fn apply_class_special_effects(&mut self) {
        // Check team for certain classes with special effects
        for hero in &mut self.heroes {
            // Ninja Check
            if hero.class == "Ninja" || hero.class == "Sensei" {
                hero.ninja_bonus = 0.1 + f64::from(std::cmp::min(hero.innate_tier, 4)) * 0.1;
                hero.ninja_evasion = 0.15;
                if hero.innate_tier == 3 {
                    hero.ninja_evasion = 0.20;
                }
                if hero.innate_tier == 4 {
                    hero.ninja_evasion = 0.25;
                }
            }

            // Daimyo Check
            if hero.class == "Daimyo" {
                hero.guaranteed_evade = true;
            }

            hero.lost_innate = -5;

            hero.consecutive_crit_bonus = 0.0;
        }
    }

    pub fn calculate_targeting_chances(&mut self) -> [f64; 4] {
        // Targeting Chances (lines 567-604)
        let mut target_chance_total = 0.0;
        let mut target_chance_heroes = [0f64; 4];
        // Compute hero chance to get targeted
        for i in 0..std::cmp::min(self.heroes.len(), 5) {
            if self.heroes[i].hp > 0.0 {
                if i < 4 {
                    for ii in i..4 {
                        target_chance_heroes[ii] += f64::from(self.heroes[i].threat);
                    }
                }
                target_chance_total += f64::from(self.heroes[i].threat);
            }
        }

        for i in 0..target_chance_heroes.len() {
            target_chance_heroes[i] /= target_chance_total
        }

        return target_chance_heroes;
    }

    pub fn update_ninja_bonus_and_extreme_crit_bonus(&mut self, round: i16, is_extreme: bool) {
        for hero in &mut self.heroes {
            if hero.class == "Sensei" && hero.lost_innate == round - 2 {
                hero.ninja_bonus = 0.1 + f64::from(std::cmp::min(hero.innate_tier, 4)) * 0.1;
                hero.ninja_evasion = 0.15;
                if hero.innate_tier == 3 {
                    hero.ninja_evasion = 0.20;
                }
                if hero.innate_tier == 4 {
                    hero.ninja_evasion = 0.25;
                }
            }

            hero.extreme_crit_bonus = 0.0;

            if is_extreme
                && hero.evasion + f64::from(hero.berserker_stage) * 0.1 + hero.ninja_evasion < 0.0
            {
                hero.extreme_crit_bonus = -25.0
                    * (hero.evasion + f64::from(hero.berserker_stage) * 0.1 + hero.ninja_evasion);
            }
        }
    }

    pub fn calculate_mob_attack(
        &mut self,
        aoe_chance: f64,
        aoe_damage: f64,
        mut heroes_alive: usize,
        mut lord_save: bool,
        round: i16,
        mut update_target: bool,
        target_chance_heroes: [f64; 4],
        crit_chance: f64,
        crit_chance_modifier: f64,
    ) -> (usize, bool, bool) {
        let mut rng = thread_rng();

        let lord_present: bool;
        let lord_index: usize;

        match self.get_class_index("Lord".to_string()) {
            Some(index) => {
                lord_present = true;
                lord_index = index;
            }
            _ => {
                lord_present = false;
                lord_index = 0;
            }
        }

        let mut lord_hero = self.heroes[lord_index].clone();

        if rng.gen::<f64>() < aoe_chance && heroes_alive > 1 {
            for hero in &mut self.heroes {
                if hero.hp > 0.0 {
                    if hero.guaranteed_evade
                        || rng.gen::<f64>()
                            < f64::min(
                                hero.evasion
                                    + f64::from(hero.berserker_stage) * 0.1
                                    + hero.ninja_evasion,
                                hero.evasion_cap,
                            )
                    {
                        if hero.class == "Dancer" || hero.class == "Acrobat" {
                            hero.guaranteed_crit = true;
                        }
                    } else {
                        hero.hp -= (hero.damage_taken * aoe_damage).ceil();
                        if hero.hp <= 0.0 {
                            if rng.gen::<f64>() >= hero.survive_chance {
                                // Surviving Fatal Blow did not activate
                                if lord_present
                                    && lord_save
                                    && hero.class != "Lord"
                                    && lord_hero.hp > 0.0
                                {
                                    // Lord Saves
                                    lord_save = false;
                                    hero.hp += (hero.damage_taken * aoe_damage).ceil();
                                    lord_hero.hp -= (lord_hero.damage_taken * aoe_damage).ceil();
                                    if lord_hero.hp <= 0.0 {
                                        // Lord Dies in Saving
                                        if rng.gen::<f64>() >= lord_hero.survive_chance {
                                            // Surviving Fatal Blow did not activate
                                            lord_hero.hp = 0.0;
                                            heroes_alive -= 1;
                                            update_target = true;
                                        } else {
                                            // Surviving Fatal Blow Activated
                                            lord_hero.hp = 1.0;
                                            lord_hero.survive_chance = 0.0;
                                        }
                                    }
                                } else {
                                    // Surviving Fatal Blow Activated
                                    hero.hp = 1.0;
                                    hero.survive_chance = 0.0;
                                }
                            }
                        }

                        // Check if innate lost
                        if hero.class == "Sensei" && hero.lost_innate != round - 1 {
                            hero.lost_innate = round;
                        }
                    }
                }
            }
        } else {
            // Mob attacks only one hero
            let mut target = 0;
            let target_rng = rng.gen::<f64>();
            for i in (0..target_chance_heroes.len()).rev() {
                if target_rng > target_chance_heroes[i] && self.heroes[i].hp > 0.0 {
                    // Hero i targeted
                    target = i;
                    break;
                }
            }

            // check hit/evade
            let hero = &mut self.heroes[target];
            if hero.guaranteed_evade
                || rng.gen::<f64>()
                    < f64::min(
                        hero.evasion + f64::from(hero.berserker_stage) * 0.1 + hero.ninja_evasion,
                        hero.evasion_cap,
                    )
            {
                if hero.class == "Danger" || hero.class == "Acrobat" {
                    hero.guaranteed_crit = true;
                }
            } else {
                // Hit, check crit
                if rng.gen::<f64>() > crit_chance * crit_chance_modifier + hero.extreme_crit_bonus {
                    // not crit
                    hero.hp -= hero.damage_taken;
                } else {
                    hero.hp -= hero.crit_damage_taken;
                }

                if hero.hp <= 0.0 {
                    if rng.gen::<f64>() >= hero.survive_chance {
                        // surviving fatal blow did not activate
                        if lord_present && lord_save && hero.class != "Lord" && lord_hero.hp > 0.0 {
                            // Lord Saves
                            lord_save = false;
                            hero.hp += hero.damage_taken;
                            lord_hero.hp -= lord_hero.damage_taken;
                            if lord_hero.hp <= 0.0 {
                                // lord dies in saving
                                if rng.gen::<f64>() >= lord_hero.survive_chance {
                                    // surviving fatal blow did not activate
                                    lord_hero.hp = 0.0;
                                    heroes_alive -= 1;
                                    update_target = true;
                                } else {
                                    // survive fatal blow
                                    lord_hero.hp = 1.0;
                                    lord_hero.survive_chance = 0.0;
                                }
                            }
                        } else {
                            // lord doesnt save
                            hero.hp = 0.0;
                            heroes_alive -= 1;
                            update_target = true;
                        }
                    } else {
                        // surviving fatal blow activated
                        hero.hp = 1.0;
                        hero.survive_chance = 0.0;
                    }
                }

                // check sensei lost innate
                if hero.class == "Sensei" && hero.lost_innate != round - 1 {
                    hero.lost_innate = round;
                }
            }
        }

        // Save lord_hero back to heroes
        self.heroes[lord_index] = lord_hero;

        return (heroes_alive, lord_save, update_target);
    }

    pub fn calculate_hemma_drain(
        &mut self,
        champion: String,
        champion_innate_tier: u8,
        hemma_mult: f64,
        round: i16,
    ) {
        let mut hemma_index = 0usize;
        match self.get_class_index("Hemma".to_string()) {
            Some(index) => hemma_index = index,
            _ => (),
        }

        if champion == "Hemma" && self.heroes[hemma_index].hp > 0.0 {
            let mut hemma_hero = self.heroes[hemma_index].clone();
            for (i, hero) in self.heroes.iter_mut().enumerate() {
                if i != hemma_index
                    && hero.hp > (0.11 - 0.01 * f64::from(champion_innate_tier)) * hero.hp_max
                {
                    hemma_hero.hemma_bonus += hemma_hero.attack * hemma_mult;
                    hero.hp =
                        hero.hp - (0.11 - 0.01 * f64::from(champion_innate_tier)) * hero.hp_max;
                    if hero.class == "Sensei" && hero.lost_innate != round - 1 {
                        hero.lost_innate = round;
                    }
                }
            }
            self.heroes[hemma_index] = hemma_hero;
            self.heroes[hemma_index].hp = f64::min(
                self.heroes[hemma_index].hp
                    + 1.0
                    + f64::from(champion_innate_tier)
                    + f64::from(std::cmp::min(champion_innate_tier - 2, 0))
                    + f64::from(std::cmp::min(champion_innate_tier - 3, 0)),
                self.heroes[hemma_index].hp_max,
            );
        }
    }

    pub fn calculate_berserker_ninja_samurai_round_effects(&mut self, round: i16) {
        for hero in &mut self.heroes {
            // Check Berserker Activation
            if hero.class == "Berserker" || hero.class == "Jarl" {
                if hero.hp >= hero.jarl_hp_stage_1 * hero.hp_max {
                    hero.berserker_stage = 0;
                } else if hero.hp >= hero.jarl_hp_stage_2 * hero.hp_max {
                    hero.berserker_stage = 1;
                } else if hero.hp >= hero.jarl_hp_stage_3 * hero.hp_max {
                    hero.berserker_stage = 2;
                } else if hero.hp > 0.0 {
                    hero.berserker_stage = 3;
                }
            }

            // Ninja Check
            if hero.class == "Ninja" && hero.hp < hero.hp_max {
                hero.ninja_bonus = 0.0;
                hero.ninja_evasion = 0.0;
            }

            if hero.class == "Sensei" && hero.lost_innate == round {
                hero.ninja_bonus = 0.0;
                hero.ninja_evasion = 0.0;
            }

            // Samurai Check
            if round == 1 && (hero.class == "Samurai" || hero.class == "Daimyo") {
                hero.guaranteed_crit = true;
                hero.guaranteed_evade = false;
            }
        }
    }

    pub fn calculate_heroes_attack(
        &mut self,
        attack_order: Vec<usize>,
        round: i16,
        rudo_bonus: f64,
        mut shark_active: i32,
        dinosaur_active: i32,
        mut barrier_modifier: f64,
        count_loot: bool,
        loot_chance: f64,
        encounter_evasion: f64,
        mut encounter_hp: f64,
        mut barrier_hp: f64,
        barrier_hp_max: f64,
        encounter_hp_max: f64,
        barrier_type: Option<ElementType>,
    ) -> (u8, f64, f64, f64, i32) {
        let mut polonia_loot: u8 = 0;
        let mut rng = thread_rng();

        for i in 0..self.get_heroes_len() {
            let jj = attack_order[i];
            let hero = &mut self.heroes[jj];

            if hero.hp > 0.0 {
                if rng.gen::<f64>() > encounter_evasion {
                    // hit mob, check crit
                    if hero.guaranteed_crit
                        || rng.gen::<f64>() < hero.critical_chance + hero.ninja_bonus + rudo_bonus
                    {
                        // crit, if samurai variant ignore barrier else reduce damage by barrier mod
                        let mut damage = (hero.attack
                            * (hero.attack_modifier
                                + 0.2 * f64::from(hero.mundra_qty)
                                + f64::from(shark_active) * 0.01 * f64::from(hero.shark_qty)
                                + f64::from(dinosaur_active)
                                    * f64::from(hero.dinosaur_qty)
                                    * 0.01
                                + 0.1
                                    * f64::from(1 + hero.berserker_level)
                                    * f64::from(hero.berserker_stage))
                            + hero.hemma_bonus)
                            * (hero.critical_multiplier + hero.consecutive_crit_bonus);
                        if round != 1 || (hero.class != "Samurai" && hero.class != "Damiyo") {
                            damage *= barrier_modifier;
                        }
                        encounter_hp -= damage;
                        hero.damage_dealt += damage;
                        if hero.class == "Conquistador" {
                            hero.consecutive_crit_bonus =
                                f64::min(hero.consecutive_crit_bonus + 0.25, 1.0);
                        }
                    } else {
                        // not crit, deal damage
                        let damage = (hero.attack
                            * (hero.attack_modifier
                                + 0.2 * f64::from(hero.mundra_qty)
                                + f64::from(shark_active) * 0.01 * f64::from(hero.shark_qty)
                                + f64::from(dinosaur_active)
                                    * f64::from(hero.dinosaur_qty)
                                    * 0.01
                                + 0.1
                                    * f64::from(1 + hero.berserker_level)
                                    * f64::from(hero.berserker_stage))
                            + hero.hemma_bonus)
                            * barrier_modifier;
                        encounter_hp -= damage;
                        hero.damage_dealt += damage;
                        if hero.class == "Conquistador" {
                            hero.consecutive_crit_bonus = 0.0;
                        }
                        if count_loot {
                            if rng.gen::<f64>() < loot_chance {
                                polonia_loot += 1;
                            }
                        }

                        // Damage Barrier
                        match barrier_type {
                            Some(barrier_element) => {
                                if barrier_hp > 0.0 && barrier_element == hero.element_type {
                                    barrier_hp -= f64::from(hero.element_qty);
                                } else if barrier_hp > 0.0 && hero.element_type == ElementType::Any
                                {
                                    barrier_hp -= f64::from(hero.element_qty) * 0.3;
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }

            if barrier_hp <= 0.0 {
                barrier_modifier = 1.0;
            } else if barrier_hp <= 0.25 * barrier_hp_max {
                barrier_modifier = 0.8;
            } else if barrier_hp <= 0.5 * barrier_hp_max {
                barrier_modifier = 0.6;
            } else if barrier_hp <= 0.75 * barrier_hp_max {
                barrier_modifier = 0.4;
            }

            if encounter_hp < encounter_hp_max / 2.0 {
                shark_active = 1;
            }

            hero.guaranteed_crit = false;
        }

        return (
            polonia_loot,
            barrier_modifier,
            barrier_hp,
            encounter_hp,
            shark_active,
        );
    }

    pub fn calculate_healing(&mut self, champion: String, champion_innate_tier: u8) {
        for hero in &mut self.heroes {
            if hero.hp > 0.0 {
                hero.hp = f64::min(hero.hp + f64::from(hero.lizard_qty * 3), hero.hp_max);
                if hero.class == "Cleric" {
                    hero.hp = f64::min(
                        hero.hp + f64::from(std::cmp::min(hero.innate_tier, 3) * 5 - 5),
                        hero.hp_max,
                    );
                } else if hero.class == "Bishop" {
                    if hero.innate_tier == 2 {
                        hero.hp = f64::min(hero.hp + 5.0, hero.hp_max);
                    } else if hero.innate_tier >= 3 {
                        hero.hp = f64::min(hero.hp + 20.0, hero.hp_max);
                    }
                }

                if champion == "Lilu" {
                    match champion_innate_tier {
                        1 => hero.hp = f64::min(hero.hp + 3.0, hero.hp_max),
                        2 => hero.hp = f64::min(hero.hp + 5.0, hero.hp_max),
                        3 => hero.hp = f64::min(hero.hp + 10.0, hero.hp_max),
                        4 => hero.hp = f64::min(hero.hp + 20.0, hero.hp_max),
                        _ => (),
                    }
                }
            }
        }
    }

    pub fn check_berserker_activation(&mut self) {
        for hero in &mut self.heroes {
            if hero.class == "Berserker" || hero.class == "Jarl" {
                if hero.hp >= hero.jarl_hp_stage_1 * hero.hp_max {
                    hero.berserker_stage = 0;
                } else if hero.hp >= hero.jarl_hp_stage_2 * hero.hp_max {
                    hero.berserker_stage = 1;
                } else if hero.hp >= hero.jarl_hp_stage_3 * hero.hp_max {
                    hero.berserker_stage = 2;
                } else if hero.hp > 0.0 {
                    hero.berserker_stage = 3;
                }
            }
        }
    }

    pub fn get_heroes_len(&self) -> usize {
        return self.heroes.len();
    }

    pub fn get_class_index(&self, class_name: String) -> Option<usize> {
        for (i, hero) in self.heroes.iter().enumerate() {
            if hero.class == class_name {
                return Some(i);
            }
        }
        return None;
    }

    pub fn get_champion_info(&self) -> (String, u8) {
        return (self.champion.to_string(), self.champion_innate_tier);
    }

    pub fn get_num_archetypes(&self) -> (u8, u8, u8, u8) {
        return (
            self.num_spellcasters,
            self.num_rogues,
            self.num_fighters,
            self.num_tricksters,
        );
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
    let mut num_tricksters = 0u8;
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
        if hero.class == "Trickster" {
            num_tricksters += 1;
        }
    }

    let team = Team {
        heroes,
        booster,
        num_fighters,
        num_rogues,
        num_spellcasters,
        num_tricksters,
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
    innate_tier: u8,
    hp: f64,
    hp_max: f64,
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
    // line 176+
    survive_chance: f64,
    guaranteed_crit: bool,
    guaranteed_evade: bool,
    lost_innate: i16,
    consecutive_crit_bonus: f64,
    berserker_stage: u8,
    berserker_level: u8,
    jarl_hp_stage_1: f64,
    jarl_hp_stage_2: f64,
    jarl_hp_stage_3: f64,
    ninja_bonus: f64,
    ninja_evasion: f64,
    evasion_cap: f64,
    hemma_bonus: f64,
    // line 451
    damage_taken: f64,
    crit_damage_taken: f64,
    damage_dealt: f64,
    // skills: Vec<Skill>,
}

impl Hero {
    fn modify_for_extreme_encounter(&mut self) {
        self.evasion -= 0.2;
        self.extreme_crit_bonus = 1.0;
    }
    fn modify_for_boss_encounter(&mut self) {
        self.defense *= self.defense_modifier + 0.2 * f64::from(self.mundra_qty);
    }
}

/// Create a hero performing type validation and calculating certain fields
pub fn create_hero(
    identifier: String,
    class: String,
    archetype: HeroArchetype,
    level: u8,
    rank: u8,
    innate_tier: u8,
    hp: f64,
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
        innate_tier,
        hp,
        hp_max: hp,
        attack: f64::from(attack),
        defense: f64::from(defense),
        threat,
        critical_chance: f64::from(critical_chance) / 100.0,
        critical_multiplier,
        evasion: f64::from(evasion) / 100.0,
        element_qty,
        element_type,
        armadillo_qty,
        lizard_qty,
        shark_qty,
        dinosaur_qty,
        mundra_qty,
        attack_modifier: 1.0 + f64::from(attack_modifier) / 100.0,
        defense_modifier: 1.0 + f64::from(defense_modifier) / 100.0,
        extreme_crit_bonus: 0.0,
        survive_chance: 0.0,
        guaranteed_crit: false,
        guaranteed_evade: false,
        lost_innate: 0,
        consecutive_crit_bonus: 0.0,
        berserker_stage: 0,
        berserker_level: 0,
        jarl_hp_stage_1: 0.75,
        jarl_hp_stage_2: 0.5,
        jarl_hp_stage_3: 0.25,
        ninja_bonus: 0.0,
        ninja_evasion: 0.0,
        evasion_cap: 0.75,
        hemma_bonus: 0.0,
        damage_taken: 0.0,
        crit_damage_taken: 0.0,
        damage_dealt: 0.0,
    };

    hero.attack /= hero.attack_modifier;
    hero.defense /= hero.defense_modifier;

    if hero.rank == 4 {
        hero.jarl_hp_stage_1 = 0.8;
        hero.jarl_hp_stage_2 = 0.55;
        hero.jarl_hp_stage_3 = 0.3;
    }

    if hero.class == "Berserker" || hero.class == "Jarl" {
        hero.berserker_level = std::cmp::min(hero.rank, 4);
    } else if hero.class == "Pathfinder" {
        hero.evasion_cap = 0.78;
    }

    return Ok(hero);
}
