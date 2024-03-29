use crate::{
    decimals::round_to_2,
    inputs::{create_sim_hero_input, SimHeroInput},
};

use std::str::FromStr;
use std::string::ToString;

use crate::equipment::{BoosterType, ElementType};

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

/// One or more Heroes fighting together in a dungeon and what booster they have
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Team {
    heroes: Vec<SimHero>,
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
    pub fn get_team_hero_names(&self) -> Vec<String> {
        let mut res: Vec<String> = Default::default();
        for hero in self.heroes.iter() {
            res.push(hero.get_identifier());
        }
        return res;
    }

    pub fn get_index_of_hero_with_identifier(&self, identifier: &String) -> Option<usize> {
        let mut index = 0;
        for hero in self.heroes.iter() {
            if hero.get_identifier().eq(identifier) {
                return Some(index);
            }
            index += 1;
        }
        return None;
    }

    pub fn get_heroes(&self) -> Vec<SimHero> {
        return self.heroes.clone();
    }

    pub fn _get_hero_at_index(&self, index: usize) -> SimHero {
        return self.heroes[index].clone();
    }

    pub fn set_hero_at_index(&mut self, index: usize, new_hero: SimHero) {
        self.heroes[index] = new_hero;
    }

    pub fn round_floats_for_display(&self) -> Team {
        let mut t2 = self.clone();
        let mut heroes: Vec<SimHero> = vec![];
        for hero in &t2.heroes {
            heroes.push(hero.round_floats_for_display());
        }
        t2.heroes = heroes;
        return t2;
    }

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
                hero.damage_taken_when_hit = 1.5 * damage
                    + ((hero.defense - 0.0) / (defense_cap / 6.0 - 0.0))
                        * (0.5 * damage - 1.5 * damage);
            } else if hero.defense <= defense_cap / 3.0 {
                hero.damage_taken_when_hit = 0.5 * damage
                    + ((hero.defense - defense_cap / 6.0)
                        / (defense_cap / 3.0 - defense_cap / 6.0))
                        * (0.3 * damage - 0.5 * damage);
            } else {
                hero.damage_taken_when_hit = 0.3 * damage
                    + ((hero.defense - defense_cap / 3.0) / (defense_cap - defense_cap / 3.0))
                        * (0.25 * damage - 0.3 * damage);
            }
            hero.crit_damage_taken_when_hit = f64::max(hero.damage_taken_when_hit, damage) * 1.5;
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

    pub fn calculate_targeting_chances(&mut self) -> Vec<f64> {
        // // Targeting Chances (lines 567-604)
        // let mut target_chance_total = 0.0;
        // let mut target_chance_heroes = [0f64; 4];
        // // Compute hero chance to get targeted
        // for i in 0..std::cmp::min(self.heroes.len(), 5) {
        //     if self.heroes[i].hp > 0.0 {
        //         if i < 4 {
        //             for ii in i..4 {
        //                 target_chance_heroes[ii] += f64::from(self.heroes[i].threat);
        //             }
        //         }
        //         target_chance_total += f64::from(self.heroes[i].threat);
        //     }
        // }

        // for i in 0..target_chance_heroes.len() {
        //     target_chance_heroes[i] /= target_chance_total
        // }

        // return target_chance_heroes;

        let mut target_chance_total: f64 = 0.0;
        let mut target_chance_heroes: Vec<f64> = vec![0.0; self.heroes.len()];
        // Compute hero chance to get targeted
        for i in 0..self.heroes.len() {
            if self.heroes[i].hp <= 0.0 {
                target_chance_heroes[i] = 0.0;
            } else {
                target_chance_heroes[i] += f64::from(self.heroes[i].threat);
                target_chance_total += f64::from(self.heroes[i].threat);
            }
        }
        for i in 0..target_chance_heroes.len() {
            target_chance_heroes[i] /= target_chance_total;
        }
        return target_chance_heroes;
    }

    pub fn update_ninja_bonus_and_extreme_crit_bonus(
        &mut self,
        round: i16,
        is_extreme: bool,
    ) -> Vec<String> {
        let mut log_queue: Vec<String> = vec![];
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
                log_queue.push(f!(
                    "Setting ninja_bonus to {} and ninja_evasion to {} for hero {}",
                    hero.ninja_bonus,
                    hero.ninja_evasion,
                    hero.identifier
                ));
            }

            hero.extreme_crit_bonus = 0.0;

            if is_extreme
                && hero.evasion + f64::from(hero.berserker_stage) * 0.1 + hero.ninja_evasion < 0.0
            {
                hero.extreme_crit_bonus = -25.0
                    * (hero.evasion + f64::from(hero.berserker_stage) * 0.1 + hero.ninja_evasion);
                log_queue.push(f!(
                    "Setting extreme_crit_bonus to {} for hero {}",
                    hero.extreme_crit_bonus,
                    hero.identifier
                ));
            }
        }
        return log_queue;
    }

    pub fn calculate_mob_attack(
        &mut self,
        aoe_chance: f64,
        aoe_damage: f64,
        mut heroes_alive: usize,
        mut lord_save: bool,
        round: i16,
        mut update_target: bool,
        target_chance_heroes: Vec<f64>,
        crit_chance: f64,
        crit_chance_modifier: f64,
    ) -> (usize, bool, bool, Vec<String>, String) {
        let mut panic_msg: String = String::from("");
        let mut log_queue: Vec<String> = vec![];
        let mut rng = thread_rng();

        let lord_present: bool;
        let lord_alive: bool;
        let lord_index: usize;

        log_queue.push("Calculating Mob Attack".to_string());
        match self.get_class_index("Lord".to_string()) {
            Some(index) => {
                if self.heroes[index].hp <= 0.0 {
                    lord_present = true;
                    lord_alive = false;
                    lord_index = index;
                    log_queue.push(f!("Found Lord but not Alive in Team at {}", index));
                } else {
                    lord_present = true;
                    lord_alive = true;
                    lord_index = index;
                    log_queue.push(f!("Found Lord Alive in Team at {}", index));
                }
            }
            _ => {
                lord_present = false;
                lord_alive = false;
                lord_index = 0;
                log_queue.push("No Lord Found Alive in Team".to_string());
            }
        }

        if rng.gen::<f64>() < aoe_chance && heroes_alive > 1 {
            log_queue.push("Mob Attempting AOE Attack".to_string());
            // Have to do two iterations through self.heroes, the first is to apply damage, the second is to check for death / lord save. This is because I can't borrow a mutable reference to the lord while also borrowing heroes itself
            // Apply Damage
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
                        log_queue.push(f!("Hero {} evades AOE attack", hero.identifier));
                        hero.dodges += 1;
                        if hero.class == "Dancer" || hero.class == "Acrobat" {
                            log_queue.push(f!("Hero {} gains guaranteed crit", hero.identifier));
                            hero.guaranteed_crit = true;
                        }
                    } else {
                        let damage = (hero.damage_taken_when_hit * aoe_damage).ceil();
                        log_queue.push(f!(
                            "Hero {} is hit by AOE, takes damage {:.2}, new health: {:.2}. checking if Hero is sensei and didn't already lost innate last round",
                            hero.identifier,
                            damage,
                            hero.hp - damage
                        ));
                        hero.hp -= damage;

                        // Check if innate lost
                        if hero.class == "Sensei" && hero.lost_innate != round - 1 {
                            log_queue.push(f!(
                                "Hero {} is sensei and lost innate due to taking damage",
                                hero.identifier
                            ));
                            hero.lost_innate = round;
                        }
                    }
                } else {
                    log_queue.push(f!(
                        "Hero {} is already at 0 hp so not damaged by AOE",
                        hero.identifier
                    ));
                }
            }

            // Check that lord survived, if so store their HP for use in lord save
            let mut lord_hp: f64 = 0.0;
            let lord_dmg_taken_when_hit: f64 =
                self.heroes[lord_index].damage_taken_when_hit.clone();
            let lord_identifier: String = self.heroes[lord_index].identifier.clone();
            if self.heroes[lord_index].hp > 0.0 {
                lord_hp = self.heroes[lord_index].hp;
            }

            // Check for Death / Lord Save
            for hero in &mut self.heroes {
                if hero.hp <= 0.0 {
                    log_queue.push(f!("Hero {} hp reduced to 0", hero.identifier));
                    if rng.gen::<f64>() >= hero.survive_chance {
                        // Surviving Fatal Blow did not activate
                        log_queue.push(f!(
                            "Hero {} did not survive fatal blow, checking for Lord save",
                            hero.identifier
                        ));
                        if lord_present
                            && lord_alive
                            && lord_save
                            && hero.class != "Lord"
                            && lord_hp > 0.0
                        {
                            // Lord Saves
                            log_queue.push(f!("Hero {} saved by Lord", hero.identifier));
                            lord_save = false;
                            hero.hp += (hero.damage_taken_when_hit * aoe_damage).ceil();
                            lord_hp -= (lord_dmg_taken_when_hit * aoe_damage).ceil();
                            log_queue.push(f!(
                                "Hero {} now has HP of {:.2}, Lord {} now has HP of {:.2}",
                                hero.identifier,
                                hero.hp,
                                lord_identifier,
                                lord_hp
                            ));
                        } else {
                            // lord doesnt save
                            log_queue.push(f!("Hero {} dies", hero.identifier));
                            hero.hp = 0.0;
                            heroes_alive -= 1;
                            update_target = true;
                        }
                    } else {
                        // Surviving Fatal Blow Activated
                        log_queue
                            .push(f!("Hero {} survived fatal blow with 1 HP", hero.identifier));
                        hero.hp = 1.0;
                        hero.survive_chance = 0.0;
                    }
                }
            }

            // Check if Lord Died while Saving
            if lord_hp <= 0.0 && lord_hp != self.heroes[lord_index].hp {
                // Lord Dies in Saving
                log_queue.push(f!("Lord {} dies while saving a hero", lord_identifier));
                if rng.gen::<f64>() >= self.heroes[lord_index].survive_chance {
                    // Surviving Fatal Blow did not activate
                    log_queue.push(f!("Lord {} did not survive fatal blow", lord_identifier));
                    self.heroes[lord_index].hp = 0.0;
                    heroes_alive -= 1;
                    update_target = true;
                } else {
                    // Surviving Fatal Blow Activated
                    log_queue.push(f!("Lord {} survived fatal blow with 1 HP", lord_identifier));
                    self.heroes[lord_index].hp = 1.0;
                    self.heroes[lord_index].survive_chance = 0.0;
                }
            } else {
                // Lord survived, save HP out
                self.heroes[lord_index].hp = lord_hp;
            }
        } else {
            // Mob attacks only one hero
            log_queue.push("Mob Attempting Single Target Attack".to_string());
            let mut target_opt: Option<usize> = None;
            let mut target_rng: f64 = rng.gen::<f64>();
            for i in 0..target_chance_heroes.len() {
                target_rng -= target_chance_heroes[i];
                if target_rng <= 0.0 {
                    // Hero i targeted
                    target_opt = Some(i);
                    break;
                }
            }
            match target_opt {
                Some(target) => {
                    // check hit/evade
                    // let hero = &mut self.heroes[target];
                    log_queue.push(f!(
                        "Mob is targeting hero at index {}: Hero {} ",
                        target,
                        self.heroes[target].identifier
                    ));
                    if self.heroes[target].guaranteed_evade
                        || rng.gen::<f64>()
                            < f64::min(
                                self.heroes[target].evasion
                                    + f64::from(self.heroes[target].berserker_stage) * 0.1
                                    + self.heroes[target].ninja_evasion,
                                self.heroes[target].evasion_cap,
                            )
                    {
                        log_queue.push(f!(
                            "Hero {} evades single target attack",
                            self.heroes[target].identifier
                        ));
                        self.heroes[target].dodges += 1;
                        if self.heroes[target].class == "Danger"
                            || self.heroes[target].class == "Acrobat"
                        {
                            log_queue.push(f!(
                                "Hero {} gains guaranteed crit",
                                self.heroes[target].identifier
                            ));
                            self.heroes[target].guaranteed_crit = true;
                        }
                    } else {
                        log_queue.push(f!(
                            "Hero {} is hit by single target attack, checking for crit damage",
                            self.heroes[target].identifier
                        ));
                        // Hit, check crit
                        let was_crit: bool;
                        if rng.gen::<f64>()
                            > crit_chance * crit_chance_modifier
                                + self.heroes[target].extreme_crit_bonus
                        {
                            // not crit
                            was_crit = false;
                            self.heroes[target].hp -= self.heroes[target].damage_taken_when_hit;
                            log_queue.push(f!(
                                "Hero {} is hit with a NORMAL attack, takes {:.2} damage bringing hp to {:.2}",
                                self.heroes[target].identifier,
                                self.heroes[target].damage_taken_when_hit,
                                self.heroes[target].hp
                            ));
                        } else {
                            was_crit = true;
                            self.heroes[target].hp -=
                                self.heroes[target].crit_damage_taken_when_hit;
                            self.heroes[target].crits_taken += 1;
                            log_queue.push(f!(
                                "Hero {} is hit with a CRITICAL attack, takes {:.2} damage bringing hp to {:.2}",
                                self.heroes[target].identifier,
                                self.heroes[target].crit_damage_taken_when_hit,
                                self.heroes[target].hp
                            ));
                        }

                        if self.heroes[target].hp <= 0.0 {
                            log_queue.push(f!(
                                "Hero {} hp reduced to 0",
                                self.heroes[target].identifier
                            ));
                            if rng.gen::<f64>() >= self.heroes[target].survive_chance {
                                // surviving fatal blow did not activate
                                log_queue.push(f!(
                                    "Hero {} did not survive fatal blow, checking for lord save",
                                    self.heroes[target].identifier
                                ));
                                if lord_present
                                    && lord_alive
                                    && lord_save
                                    && self.heroes[target].class != "Lord"
                                    && self.heroes[lord_index].hp > 0.0
                                {
                                    // Lord Saves
                                    log_queue.push(f!(
                                        "Hero {} is saved by lord",
                                        self.heroes[target].identifier
                                    ));
                                    lord_save = false;
                                    let damage_transfered = if was_crit {
                                        self.heroes[target].crit_damage_taken_when_hit
                                    } else {
                                        self.heroes[target].damage_taken_when_hit
                                    };
                                    self.heroes[target].hp += damage_transfered;
                                    self.heroes[lord_index].hp -= damage_transfered;
                                    log_queue.push(f!(
                                        "Hero {} now has HP of {:.2}, Lord {} now has HP of {:.2}",
                                        self.heroes[target].identifier,
                                        self.heroes[target].hp,
                                        self.heroes[lord_index].identifier,
                                        self.heroes[lord_index].hp
                                    ));
                                    if self.heroes[lord_index].hp <= 0.0 {
                                        // lord dies in saving
                                        log_queue.push(f!(
                                            "Lord {} dies while saving hero {}",
                                            self.heroes[lord_index].identifier,
                                            self.heroes[target].identifier
                                        ));
                                        if rng.gen::<f64>()
                                            >= self.heroes[lord_index].survive_chance
                                        {
                                            // surviving fatal blow did not activate
                                            log_queue.push(f!(
                                                "Lord {} did not survive fatal blow",
                                                self.heroes[lord_index].identifier
                                            ));
                                            self.heroes[lord_index].hp = 0.0;
                                            heroes_alive -= 1;
                                            update_target = true;
                                        } else {
                                            // survive fatal blow
                                            log_queue.push(f!(
                                                "Lord {} survived fatal blow with 1 HP",
                                                self.heroes[lord_index].identifier
                                            ));
                                            self.heroes[lord_index].hp = 1.0;
                                            self.heroes[lord_index].survive_chance = 0.0;
                                        }
                                    }
                                } else {
                                    // lord doesnt save
                                    log_queue
                                        .push(f!("Hero {} dies", self.heroes[target].identifier));
                                    self.heroes[target].hp = 0.0;
                                    heroes_alive -= 1;
                                    update_target = true;
                                }
                            } else {
                                // surviving fatal blow activated
                                log_queue.push(f!(
                                    "Hero {} survived fatal blow with 1 HP",
                                    self.heroes[target].identifier
                                ));
                                self.heroes[target].hp = 1.0;
                                self.heroes[target].survive_chance = 0.0;
                            }
                        }

                        // check sensei lost innate
                        log_queue.push(f!(
                            "Checking if Hero {} is sensei and didn't already lose innate last round",
                            self.heroes[target].identifier
                        ));
                        if self.heroes[target].class == "Sensei"
                            && self.heroes[target].lost_innate != round - 1
                        {
                            log_queue.push(f!(
                                "Hero {} is sensei and lost innate due to taking damage",
                                self.heroes[target].identifier
                            ));
                            self.heroes[target].lost_innate = round;
                        }
                    }
                }
                None => {
                    panic_msg = f!("Should not be possible for targetOpt to remain None, as RNG range is [0,1) and target_chance_heroes values should sum to 1... Something is broken. target_opt: {:?} target_rng: {}, target_chance_heroes: {:?}, heroes_alive: {}, heroes: {:#?}", target_opt, target_rng, target_chance_heroes, heroes_alive, self.heroes);
                }
            };
        }

        return (heroes_alive, lord_save, update_target, log_queue, panic_msg);
    }

    pub fn calculate_hemma_drain(
        &mut self,
        champion_innate_tier: u8,
        hemma_mult: f64,
        round: i16,
    ) -> Vec<String> {
        let mut log_queue: Vec<String> = vec![];
        log_queue.push("Calculating Hemma Drain".to_string());
        let mut hemma_index = 0usize;
        match self.get_class_index("Hemma".to_string()) {
            Some(index) => hemma_index = index,
            _ => (),
        }
        log_queue.push(f!(
            "Hemma found at index {} and has hp {:.2}",
            hemma_index,
            self.heroes[hemma_index].hp
        ));

        if self.heroes[hemma_index].hp > 0.0 {
            log_queue.push("Hemma is alive".to_string());
            let mut hemma_hero = self.heroes[hemma_index].clone();
            for (i, hero) in self.heroes.iter_mut().enumerate() {
                if i != hemma_index
                    && hero.hp > (0.11 - 0.01 * f64::from(champion_innate_tier)) * hero.hp_max
                {
                    log_queue.push(f!(
                        "Hero {} is not hemma and has enough HP ({:.2}) to steal",
                        hero.identifier,
                        hero.hp
                    ));
                    hemma_hero.hemma_bonus += hemma_hero.attack * hemma_mult;
                    hero.hp =
                        hero.hp - (0.11 - 0.01 * f64::from(champion_innate_tier)) * hero.hp_max;
                    if hero.class == "Sensei" && hero.lost_innate != round - 1 {
                        log_queue.push(f!("Hero {} is sensei and loses innate", hero.identifier));
                        hero.lost_innate = round;
                    }
                    log_queue.push(f!(
                        "New hemma bonus is {}, new Hero {} HP is {:.2}",
                        hemma_hero.hemma_bonus,
                        hero.identifier,
                        hero.hp
                    ));
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
            log_queue.push(f!(
                "(Meta-Info) Hemma Hero saved back to team heroes vector, and now has hp of {:.2}",
                self.heroes[hemma_index].hp
            ));
        }

        return log_queue;
    }

    pub fn calculate_berserker_ninja_samurai_round_effects(&mut self, round: i16) -> Vec<String> {
        let mut log_queue: Vec<String> = vec![];
        log_queue.push("Calculating berserker, ninja, samurai round effects".to_string());
        for hero in &mut self.heroes {
            // Check Berserker Activation
            if hero.class == "Berserker" || hero.class == "Jarl" {
                log_queue.push(f!("Hero {} is class {}", hero.identifier, hero.class));
                if hero.hp >= hero.jarl_hp_stage_1 * hero.hp_max {
                    hero.berserker_stage = 0;
                } else if hero.hp >= hero.jarl_hp_stage_2 * hero.hp_max {
                    hero.berserker_stage = 1;
                } else if hero.hp >= hero.jarl_hp_stage_3 * hero.hp_max {
                    hero.berserker_stage = 2;
                } else if hero.hp > 0.0 {
                    hero.berserker_stage = 3;
                }
                log_queue.push(f!(
                    "Hero {} now has berserker_stage of {}",
                    hero.identifier,
                    hero.berserker_stage
                ));
            }

            // Ninja Check
            if hero.class == "Ninja" && hero.hp < hero.hp_max {
                hero.ninja_bonus = 0.0;
                hero.ninja_evasion = 0.0;
                log_queue.push(f!(
                    "Hero {} is ninja and loses bonus because hp < hp_max",
                    hero.identifier
                ));
            }

            if hero.class == "Sensei" && hero.lost_innate == round {
                hero.ninja_bonus = 0.0;
                hero.ninja_evasion = 0.0;
                log_queue.push(f!(
                    "Hero {} is sensei and loses bonus because they lost_innate this round",
                    hero.identifier
                ));
            }

            // Samurai Check
            if round == 1 && (hero.class == "Samurai" || hero.class == "Daimyo") {
                hero.guaranteed_crit = true;
                hero.guaranteed_evade = false;
                log_queue.push(f!("Hero {} is class {} and gains guaranteed_crit while losing guaranteed_evade as round = 1", hero.identifier, hero.class));
            }
        }

        return log_queue;
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
    ) -> (u8, f64, f64, f64, i32, Vec<String>) {
        let mut log_queue: Vec<String> = vec![];
        log_queue.push("Calculate Heroes Attack".to_string());

        let mut polonia_loot: u8 = 0;
        let mut rng = thread_rng();

        log_queue.push(f!("Attack order is {:?}", attack_order));
        for i in 0..self.get_heroes_len() {
            let jj = attack_order[i];
            let hero = &mut self.heroes[jj];

            log_queue.push(f!("Hero {} has hp of {:.2}", hero.identifier, hero.hp));
            if hero.hp > 0.0 {
                log_queue.push(f!("Hero {} attempts to attack", hero.identifier));
                if rng.gen::<f64>() > encounter_evasion {
                    // hit mob, check crit
                    log_queue.push(f!("Hero {} hits mob, checking crit", hero.identifier));
                    if hero.guaranteed_crit
                        || rng.gen::<f64>() < hero.critical_chance + hero.ninja_bonus + rudo_bonus
                    {
                        // crit, if samurai variant ignore barrier else reduce damage by barrier mod
                        hero.crits_dealt += 1;
                        log_queue.push(f!(
                            "Hero {} hits with a CRITICAL hit, calculating damage",
                            hero.identifier
                        ));
                        let mut damage = (hero.attack
                            * (hero.attack_modifier
                                + 0.2 * f64::from(hero.mundra_qty)
                                + f64::from(shark_active)
                                    * 0.01
                                    * f64::from(hero.shark_qty)
                                    * 20.0
                                + f64::from(dinosaur_active)
                                    * f64::from(hero.dinosaur_qty)
                                    * 0.01
                                    * 25.0
                                + 0.1
                                    * f64::from(1 + hero.berserker_level)
                                    * f64::from(hero.berserker_stage))
                            + hero.hemma_bonus)
                            * (hero.critical_multiplier + hero.consecutive_crit_bonus);
                        if round != 1 || (hero.class != "Samurai" && hero.class != "Daimyo") {
                            log_queue.push(f!(
                                "Hero {} is class {} and round is not 1 so do not pierce elemental barrier. damage *= barrier_modifier: {} * {} = {}",
                                hero.identifier,
                                hero.class,
                                damage,
                                barrier_modifier,
                                damage * barrier_modifier
                            ));
                            damage *= barrier_modifier;
                        } else {
                            log_queue.push(f!(
                                "Hero {} is class {} and round is 1 so pierce elemental barrier. damage = damage: {}",
                                hero.identifier,
                                hero.class,
                                damage
                            ));
                        }
                        encounter_hp -= damage;
                        hero.damage_dealt += damage;
                        log_queue.push(f!(
                            "Hero {} deals damage {:.2} to mob, bringing hp to {:.2}",
                            hero.identifier,
                            damage,
                            encounter_hp
                        ));
                        if hero.class == "Conquistador" {
                            hero.consecutive_crit_bonus =
                                f64::min(hero.consecutive_crit_bonus + 0.25, 1.0);
                            log_queue.push(f!(
                                "Hero {} is conquistador, new consecutive crit bonus is {}",
                                hero.identifier,
                                hero.consecutive_crit_bonus
                            ));
                        }
                    } else {
                        // not crit, deal damage
                        log_queue.push(f!(
                            "Hero {} misses crit, does NORMAL attack",
                            hero.identifier
                        ));
                        let damage = (hero.attack
                            * (hero.attack_modifier
                                + 0.2 * f64::from(hero.mundra_qty)
                                + f64::from(shark_active)
                                    * 0.01
                                    * f64::from(hero.shark_qty)
                                    * 20.0
                                + f64::from(dinosaur_active)
                                    * f64::from(hero.dinosaur_qty)
                                    * 0.01
                                    * 25.0
                                + 0.1
                                    * f64::from(1 + hero.berserker_level)
                                    * f64::from(hero.berserker_stage))
                            + hero.hemma_bonus)
                            * barrier_modifier;
                        encounter_hp -= damage;
                        hero.damage_dealt += damage;
                        log_queue.push(f!(
                            "Hero {} deals damage {:.2} to mob, bringing hp to {:.2}",
                            hero.identifier,
                            damage,
                            encounter_hp
                        ));
                        if hero.class == "Conquistador" {
                            hero.consecutive_crit_bonus = 0.0;
                            log_queue.push(f!(
                                "Hero {} is conquistador, consecutive crit bonus reset to 0",
                                hero.identifier
                            ));
                        }
                        if count_loot {
                            if rng.gen::<f64>() < loot_chance {
                                polonia_loot += 1;
                            }
                            log_queue.push(f!("Polonia loot is now {}", polonia_loot));
                        }

                        // Damage Barrier
                        log_queue.push(f!(
                            "Checking for barrier (type: {:#?}) damage",
                            barrier_type
                        ));
                        match barrier_type {
                            Some(barrier_element) => {
                                if barrier_hp > 0.0 && barrier_element == hero.element_type {
                                    barrier_hp -= f64::from(hero.element_qty);
                                    log_queue.push(f!(
                                        "Hero Matches Element Type, New Barrier HP is {:.2}",
                                        barrier_hp
                                    ));
                                } else if barrier_hp > 0.0 && hero.element_type == ElementType::Any
                                {
                                    barrier_hp -= f64::from(hero.element_qty) * 0.3;
                                    log_queue.push(f!(
                                        "Hero Has Any Element Type, New Barrier HP is {:.2}",
                                        barrier_hp
                                    ));
                                }
                            }
                            _ => (),
                        }
                    }
                } else {
                    // Missed
                    hero.attacks_missed += 1;
                    log_queue.push(f!("Hero {} missed attack", hero.identifier));
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
            log_queue.push(f!("Barrier Modifier is now {}", barrier_modifier));

            if encounter_hp < encounter_hp_max / 2.0 {
                shark_active = 1;
                log_queue.push("Mob now below 50% HP, Shark Activating".to_string());
            }

            hero.guaranteed_crit = false;
        }

        return (
            polonia_loot,
            barrier_modifier,
            barrier_hp,
            encounter_hp,
            shark_active,
            log_queue,
        );
    }

    pub fn calculate_healing(&mut self, champion: String, champion_innate_tier: u8) -> Vec<String> {
        let mut log_queue: Vec<String> = vec![];
        log_queue.push("Calculating Healing".to_string());
        for hero in &mut self.heroes {
            if hero.hp > 0.0 {
                log_queue.push(f!(
                    "Hero {} is alive, hp is {:.2}",
                    hero.identifier,
                    hero.hp
                ));
                let mut before_hp = hero.hp;

                hero.hp = f64::min(before_hp + hero.hp_regen, hero.hp_max);
                log_queue.push(f!(
                    "Hero {} regenerates for {:.2}",
                    hero.identifier,
                    hero.hp - before_hp
                ));

                before_hp = hero.hp;

                if champion == "Lilu" {
                    match champion_innate_tier {
                        1 => hero.hp = f64::min(hero.hp + 3.0, hero.hp_max),
                        2 => hero.hp = f64::min(hero.hp + 5.0, hero.hp_max),
                        3 => hero.hp = f64::min(hero.hp + 10.0, hero.hp_max),
                        4 => hero.hp = f64::min(hero.hp + 20.0, hero.hp_max),
                        _ => (),
                    }
                    log_queue.push(f!(
                        "Hero {} is healed by Lilu for {:.2}",
                        hero.identifier,
                        hero.hp - before_hp
                    ));
                    // running_hp = hero.hp;
                }
                log_queue.push(f!(
                    "Hero {} has new hp of {:.2} total",
                    hero.identifier,
                    hero.hp
                ));
            }
        }
        return log_queue;
    }

    pub fn check_berserker_activation(&mut self) -> Vec<String> {
        let mut log_queue: Vec<String> = vec![];
        log_queue.push("Checking Berserker Activation".to_string());
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
                log_queue.push(f!(
                    "Hero {} is class {} and new berserker stage is {}",
                    hero.identifier,
                    hero.class,
                    hero.berserker_stage
                ));
            }
        }
        return log_queue;
    }

    pub fn get_heroes_len(&self) -> usize {
        return self.heroes.len();
    }

    pub fn get_heroes_hp_as_strings(&self) -> StringVec {
        let mut res = StringVec(vec![]);
        for hero in &self.heroes {
            res.push(f!(
                "{}: {:.2} ({:.2}%)",
                hero.identifier,
                hero.hp,
                hero.hp / hero.hp_max * 100.0
            ));
        }
        return res;
    }

    pub fn get_heroes_hp(&self) -> Vec<f64> {
        let mut res: Vec<f64> = vec![];
        for hero in &self.heroes {
            res.push(hero.hp);
        }
        return res;
    }

    pub fn get_heroes_accuracy_stats(&self) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
        let mut res: (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) = (vec![], vec![], vec![], vec![]);

        for hero in &self.heroes {
            res.0.push(hero.crits_taken);
            res.1.push(hero.crits_dealt);
            res.2.push(hero.dodges);
            res.3.push(hero.attacks_missed);
        }

        return res;
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

    pub fn get_team_damage_dealt_total(&self) -> Vec<f64> {
        let mut res: Vec<f64> = vec![];
        for hero in &self.heroes {
            res.push(hero.damage_dealt);
        }
        return res;
    }
}

/// Create a team performing type validation and calculating certain fields
pub fn create_team(
    heroes: Vec<SimHero>,
    booster: Option<BoosterType>,
) -> Result<Team, &'static str> {
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
pub struct SimHero {
    identifier: String,
    class: String,
    archetype: HeroArchetype,
    level: u8,
    rank: u8,
    innate_tier: u8,
    hp: f64,
    hp_max: f64,
    hp_regen: f64,
    attack: f64,
    defense: f64,
    threat: u16,
    critical_chance: f64,
    critical_multiplier: f64,
    evasion: f64,
    survive_fatal_blow_chance: f64,
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
    damage_taken_when_hit: f64,
    crit_damage_taken_when_hit: f64,
    damage_dealt: f64,
    // skills: Vec<Skill>,
    // accuracy_tracking
    crits_taken: u8,
    crits_dealt: u8,
    dodges: u8,
    attacks_missed: u8,
}

impl SimHero {
    pub fn get_class(&self) -> String {
        return self.class.to_string();
    }

    pub fn get_identifier(&self) -> String {
        return self.identifier.to_string();
    }

    fn modify_for_extreme_encounter(&mut self) {
        self.evasion -= 0.2;
    }
    fn modify_for_boss_encounter(&mut self) {
        self.defense = (self.defense / self.defense_modifier)
            * (self.defense_modifier + 0.2 * f64::from(self.mundra_qty));
    }
    pub fn round_floats_for_display(&self) -> SimHero {
        let mut h2 = self.clone();
        h2.hp = round_to_2(h2.hp);
        h2.hp_max = round_to_2(h2.hp_max);
        h2.hp_regen = round_to_2(h2.hp_regen);
        h2.attack = round_to_2(h2.attack);
        h2.defense = round_to_2(h2.defense);
        h2.critical_chance = round_to_2(h2.critical_chance);
        h2.critical_multiplier = round_to_2(h2.critical_multiplier);
        h2.evasion = round_to_2(h2.evasion);
        h2.attack_modifier = round_to_2(h2.attack_modifier);
        h2.defense_modifier = round_to_2(h2.defense_modifier);
        h2.extreme_crit_bonus = round_to_2(h2.extreme_crit_bonus);
        h2.survive_chance = round_to_2(h2.survive_chance);
        h2.consecutive_crit_bonus = round_to_2(h2.consecutive_crit_bonus);
        h2.jarl_hp_stage_1 = round_to_2(h2.jarl_hp_stage_1);
        h2.jarl_hp_stage_2 = round_to_2(h2.jarl_hp_stage_2);
        h2.jarl_hp_stage_3 = round_to_2(h2.jarl_hp_stage_3);
        h2.ninja_bonus = round_to_2(h2.ninja_bonus);
        h2.ninja_evasion = round_to_2(h2.ninja_evasion);
        h2.evasion_cap = round_to_2(h2.evasion_cap);
        h2.hemma_bonus = round_to_2(h2.hemma_bonus);
        h2.damage_taken_when_hit = round_to_2(h2.damage_taken_when_hit);
        h2.crit_damage_taken_when_hit = round_to_2(h2.crit_damage_taken_when_hit);
        h2.damage_dealt = round_to_2(h2.damage_dealt);
        return h2;
    }
}

impl From<SimHero> for SimHeroInput {
    fn from(item: SimHero) -> Self {
        return create_sim_hero_input(
            item.identifier,
            item.class,
            item.level,
            item.rank,
            item.innate_tier,
            item.hp,
            item.hp_regen,
            item.attack * item.attack_modifier,
            item.defense,
            item.threat,
            item.critical_chance,
            item.critical_multiplier,
            item.evasion,
            item.survive_fatal_blow_chance,
            item.element_qty,
            item.element_type.to_string(),
            item.armadillo_qty,
            item.lizard_qty,
            item.shark_qty,
            item.dinosaur_qty,
            item.mundra_qty,
            item.attack_modifier - 1.0,
            item.defense_modifier - 1.0,
        );
    }
}

/// Create a hero performing type validation and calculating certain fields
pub fn create_sim_hero(
    identifier: String,
    class: String,
    level: u8,
    rank: u8,
    innate_tier: u8,
    hp: f64,
    hp_regen: f64,
    attack: f64,
    defense: f64,
    threat: u16,
    critical_chance: f64,
    critical_multiplier: f64,
    evasion: f64,
    survive_fatal_blow_chance: f64,
    element_qty: u16,
    element_type_string: String,
    armadillo_qty: u8,
    lizard_qty: u8,
    shark_qty: u8,
    dinosaur_qty: u8,
    mundra_qty: u8,
    attack_modifier: f64,
    defense_modifier: f64,
) -> Result<SimHero, &'static str> {
    let atk_mod = 1.0 + attack_modifier;
    let def_mod = 1.0 + defense_modifier;

    let archetype: HeroArchetype;
    let red_list: [String; 12] = [
        String::from("Soldier"),
        String::from("Mercenary"),
        String::from("Barbarian"),
        String::from("Chieftain"),
        String::from("Knight"),
        String::from("Lord"),
        String::from("Ranger"),
        String::from("Warden"),
        String::from("Samurai"),
        String::from("Daimyo"),
        String::from("Berserker"),
        String::from("Jarl"),
    ];
    let green_list: [String; 12] = [
        String::from("Thief"),
        String::from("Trickster"),
        String::from("Monk"),
        String::from("Grandmaster"),
        String::from("Musketeer"),
        String::from("Conquistador"),
        String::from("Wanderer"),
        String::from("Pathfinder"),
        String::from("Ninja"),
        String::from("Sensei"),
        String::from("Dancer"),
        String::from("Acrobat"),
    ];
    let blue_list: [String; 12] = [
        String::from("Mage"),
        String::from("Archmage"),
        String::from("Cleric"),
        String::from("Bishop"),
        String::from("Druid"),
        String::from("Arch Druid"),
        String::from("Sorcerer"),
        String::from("Warlock"),
        String::from("Spellblade"),
        String::from("Spellknight"),
        String::from("Geomancer"),
        String::from("Astramancer"),
    ];
    let champion_list: [String; 12] = [
        String::from("Argon"),
        String::from("Lilu"),
        String::from("Polonia"),
        String::from("Yami"),
        String::from("Rudo"),
        String::from("Sia"),
        String::from("Donovan"),
        String::from("Ashley"),
        String::from("Hemma"),
        String::from("Aang"),
        String::from("Sokka"),
        String::from("King Reinholdt"),
    ];

    if red_list.contains(&class) {
        archetype = HeroArchetype::RedFighter;
    } else if green_list.contains(&class) {
        archetype = HeroArchetype::GreenRogue;
    } else if blue_list.contains(&class) {
        archetype = HeroArchetype::BlueSpellcaster;
    } else if champion_list.contains(&class) {
        archetype = HeroArchetype::Champion;
    } else {
        return Err("Unknown Class, Could Not Create Hero");
    }

    let element_type: ElementType = ElementType::from_str(element_type_string.as_str()).unwrap();
    // match element_type_string.as_str() {
    //     "Air" => element_type = ElementType::Air,
    //     "Water" => element_type = ElementType::Water,
    //     "Fire" => element_type = ElementType::Fire,
    //     "Earth" => element_type = ElementType::Earth,
    //     "Light" => element_type = ElementType::Light,
    //     "Dark" => element_type = ElementType::Dark,
    //     "Any" => element_type = ElementType::Any,
    //     _ => return Err("Unknown Element Type, Could Not Create Hero"),
    // }

    let mut hero = SimHero {
        identifier,
        class,
        archetype,
        level,
        rank,
        innate_tier,
        hp,
        hp_max: hp,
        hp_regen: hp_regen,
        attack: attack / atk_mod,
        defense: defense,
        threat,
        critical_chance,
        critical_multiplier,
        evasion,
        survive_fatal_blow_chance,
        element_qty,
        element_type,
        armadillo_qty,
        lizard_qty,
        shark_qty,
        dinosaur_qty,
        mundra_qty,
        attack_modifier: atk_mod,
        defense_modifier: def_mod,
        extreme_crit_bonus: 1.0,
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
        damage_taken_when_hit: 0.0,
        crit_damage_taken_when_hit: 0.0,
        damage_dealt: 0.0,
        crits_taken: 0,
        crits_dealt: 0,
        dodges: 0,
        attacks_missed: 0,
    };

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

pub struct StringVec(Vec<String>);

impl StringVec {
    fn push(&mut self, new_entry: String) {
        self.0.push(new_entry);
    }
}

impl std::fmt::Display for StringVec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut comma_separated = String::new();

        for num in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&num.to_string());
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f, "[{}]", comma_separated)
    }
}
