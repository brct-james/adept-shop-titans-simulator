use super::dungeons::Encounter;
use super::heroes::Team;

use serde::{Deserialize, Serialize};

use rand::seq::SliceRandom;
use rand::thread_rng;

/// A simulated encounter between a Team and a Dungeon
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Simulation {
    team: Team,
    encounter: Encounter,
    metrics: Vec<String>,
}

impl Simulation {
    pub fn run(&mut self) -> Result<SimResult, &'static str> {
        // If encounter.is_boss then ignore Mundra
        // Error if more heroes in team than encounter allows

        // Normalize %s
        let (is_extreme, is_boss) = self.encounter.is_extreme_or_boss();
        self.team.normalize_percents(is_extreme, is_boss);

        // Calculate Champion Bonuses
        let mut champion_attack_bonus = 0f64;
        let mut champion_defense_bonus = 0f64;

        // Polonia Loot
        let mut count_loot = false;
        let mut loot_chance = 0f64;
        let mut polonia_loot_cap = 20;
        let mut polonia_loot_cap_hit = 0;
        let mut polonia_loot_total = 0;
        let mut polonia_loot = 0;

        // Booster Bonuses
        let booster_attack_bonus = 0f64;
        let booster_defense_bonus = 0f64;

        let mut hemma_mult = 0;
        let mut hemma_index = 0;

        let mut target = 0;

        let mut lord_present = false;
        let mut lord_index = 0;

        for i in 0..self.team.heroes.len() {
            if self.team.heroes[i].class == "Lord" && self.team.heroes[i].hp > 0 {
                lord_present = true;
                lord_index = i;
                break;
            }
        }

        match self.team.champion {
            "Argon" => {
                champion_attack_bonus = 0.1f64 * f64::from(self.team.champion_innate_tier);
                champion_defense_bonus = champion_attack_bonus;
            },
            "Ashley" => {
                champion_attack_bonus = 0.05 + 0.05 * f64::from(self.team.champion_innate_tier);
                if is_boss {
                    champion_attack_bonus = champion_attack_bonus * 2;
                }
                champion_defense_bonus = champion_attack_bonus;
            },
            "Donovan" => {
                match self.team.champion_innate_tier {
                    1u8 => {
                        champion_attack_bonus = 0.05 * f64::from(self.team.num_spellcasters);
                    },
                    2u8 => {
                        champion_attack_bonus = 0.08 * f64::from(self.team.num_spellcasters);
                    },
                    3u8 => {
                        champion_attack_bonus = 0.10 * f64::from(self.team.num_spellcasters);
                    },
                    4u8 => {
                        champion_attack_bonus = 0.14 * f64::from(self.team.num_spellcasters);
                    },
                }

                for hero in self.team.heroes {
                    hero.hp = hero.hp * (1.0 + (0.04 + 0.01 * f64::from(self.team.champion_innate_tier) + 0.02 * std::cmp::max(self.team.champion_innate_tier - 3, 0)) * f64::from(self.team.num_fighters));
                    hero.critical_chance = hero.critical_chance + (0.02 + 0.01 * f64::from(self.team.champion_innate_tier) + 0.01 * std::cmp::max(self.team.champion_innate_tier - 3, 0)) * f64::from(self.team.num_rogues);
                    hero.evasion = hero.evasion + (0.02 + 0.01 * f64::from(self.team.champion_innate_tier) + 0.01 * std::cmp::max(self.team.champion_innate_tier - 3, 0)) * f64::from(self.team.num_rogues);
                    if hero.class == "Mercenary" {
                        // it looks lie mercenaries get an extra 1.25x for some reason? Why is this?
                        hero.hp *= 1.25;
                        hero.critical_chance *= 1.25;
                        hero.evasion *= 1.25;
                    }
                }
            },
            "Hemma" => {
                for hero in self.team.heroes {
                    hero.hp = hero.hp * (1.0 + 0.05 * f64::from(self.team.champion_innate_tier) + 0.05 * std::cmp::max(self.team.champion_innate_tier - 3, 0));
                    if hero.class == "Mercenary" {
                        hero.hp *= 1.328;
                    }
                }
                hemma_mult = 0.04 + self.team.champion_innate_tier * 0.02;
            },
            "Lilu" => {
                for hero in self.team.heroes {
                    hero.hp = hero.hp * (1.05 + 0.05 * f64::from(self.team.champion_innate_tier));
                    if hero.class == "Mercenary" {
                        hero.hp *= 1.25;
                    }
                }
            },
            "Polonia" => {
                champion_defense_bonus = 0.05 + 0.05 * f64::from(self.team.champion_innate_tier);
                for hero in self.team.heroes {
                    if self.team.champion_innate_tier < 3 {
                        hero.evasion = hero.evasion + 0.05 * 1.25;
                    } else {
                        hero.evasion = hero.evasion + 0.1 * 1.25;
                    }

                    if hero.class == "Mercenary" {
                        hero.evasion *= 1.25;
                    }
                }
                count_loot = true;
                match self.team.champion_innate_tier {
                    1u8 => loot_chance = 0.3,
                    2u8 => loot_chance = 0.35,
                    3u8 => loot_chance = 4,
                    4u8 => loot_chance = 5,
                    _ => (),
                }
                loot_chance = loot_chance + self.team.num_tricksters * 0.02;
                polonia_loot_cap = polonia_loot_cap + self.team.num_tricksters * 2;
            },
            "Sia" => {
                champion_attack_bonus = 0.05 + 0.05 * f64::from(self.team.champion_innate_tier);
            },
            "Yami" => {
                for hero in self.team.heroes {
                    hero.critical_chance = hero.critical_chance + 0.05 * f64::from(self.team.champion_innate_tier);
                    hero.evasion = hero.evasion + 0.05 * f64::from(self.team.champion_innate_tier);
                    if hero.class == "Mercenary" {
                        hero.critical_chance *= 1.25;
                        hero.evasion *= 1.25;
                    }
                }
            },
            _ => (),
        }

        // Calculate Booster Bonuses
        match self.team.booster {
            Some(booster_type) => {
                match booster_type {
                    BoosterType::MegaPowerBooster => {
                        booster_attack_bonus = 0.8;
                        booster_defense_bonus = 0.8;
                        for hero in self.team.heroes {
                            hero.critical_chance += 0.25;
                            hero.critical_multiplier += 0.5;
                        }
                    },
                    BoosterType::SuperPowerBooster => {
                        booster_attack_bonus = 0.4;
                        booster_defense_bonus = 0.4;
                        for hero in self.team.heroes {
                            hero.critical_chance += 0.1;
                        }
                    },
                    BoosterType::PowerBooster => {
                        booster_attack_bonus = 0.2;
                        booster_defense_bonus = 0.2;
                    },
                    _ => (),
                }
            },
            _ = (),
        }

        // Apply Champion and Booster Bonuses
        for hero in self.team.heroes {
            if hero.class == "Mercenary" {
                champion_attack_bonus *= 1.25;
                champion_defense_bonus *= 1.25;
            }
            hero.attack = hero.attack * (1.0 + champion_attack_bonus + booster_attack_bonus);
            hero.defense = hero.defense * (1.0 + champion_attack_bonus + booster_attack_bonus);
        }

        // Calc the amount of damage taken by each hero in encounter
        for hero in self.team.heroes {
            if hero.defense <= self.encounter.defense_cap / 6 {
                hero.damage_taken = (1.5 * self.encounter.damage + ((hero.defense - 0)/(self.encounter.defense_cap / 6 - 0)) * (0.5 * self.encounter.damage - 1.5 * self.encounter.damage)).round() as u32;
            } else if hero.defense <= self.encounter.defense_cap / 3 {
                hero.damage_taken = (0.5 * self.encounter.damage + ((hero.defense - self.encounter.defense_cap / 6) / (self.encounter.defense_cap / 3 - self.encounter.defense_cap / 6)) * (0.3 * self.encounter.damage - 0.5 * self.encounter.damage)).round() as u32;
            } else {
                hero.damage_taken = (0.3 * self.encounter.damage + ((hero.defense - self.encounter.defense_cap / 3) / (self.encounter.defense_cap - self.encounter.defense_cap / 3)) * (0.25 * self.encounter.damage - 0.3 * self.encounter.damage)).round() as u32;
            }
            hero.crit_damage_taken = (std::cmp::max(hero.damage_taken, self.encounter.damage) * 1.5).round() as u32;
        }

        // PREVIOUS TO THIS IS SETUP, NOT RUN EACH SIMULATION, CONSIDER MOVING TO TRIALS CODE
        
        // Simulate Encounter
        let mut cont_fight = true;
        let mut won_fight = false;

        for (i, hero) in self.team.heroes.iter().enumerate() {
            hero.survive_chance = hero.armadillo_qty / 100;
            if hero.class == "Cleric" || hero.class == "Bishop" {
                hero.survive_chance = 1.2;
            }

            if hero.class == "Hemma" {
                hero.hemma_bonus = 0;
                hemma_index = i;
            }

            hero.berserker_stage = 0;
            hero.guaranteed_crit = false;
        }

        let mut update_target = true;
        let mut round = 0;
        let mut shark_active = 0;
        let mut dinosaur_active = 1;
        let mut lord_save = true;
        let mut rudo_bonus = 0f64;

        if self.team.champion == "Rudo" {
            match self.team.champion_innate_tier {
                1u8 => rudo_bonus = 0.3,
                2u8 => rudo_bonus = 0.4,
                3u8 => rudo_bonus = 0.4,
                4u8 => rudo_bonus = 0.5,
                _ => (),
            }
        }

        // Check team for certain classes with special effects
        for hero in self.team.heroes {
            // Ninja Check
            if hero.class == "Ninja" || hero.class == "Sensei" {
                hero.ninja_bonus = 0.1 + std::cmp::min(hero.innate_tier, 4) * 0.1;
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

            hero.consecutive_crit_bonus = 0;
        }

        // Generate Random Attack Order
        let attack_order: Vec<usize> = (0..self.team.heroes.len()).collect();
        let mut rng = thread_rng();
        attack_order.shuffle(&mut rng);

        // Set barrier mod
        if self.encounter.barrier_hp == 0 {
            self.encounter.barrier_modifier = 1.0;
        } else {
            self.encounter.barrier_modifier = 0.2;
        }

        // Define targetting variables
        let mut target_chance_total = 0f64;
        let mut target_chance_heroes = [0f64; 4];

        // Define heroes alive
        let mut heroes_alive = u8::from(self.team.heroes.len())

        // START QUEST
        while cont_fight {
            round += 1;

            // Targeting Chances (lines 567-604)
            if update_target {
                target_chance_total = 0;
                target_chance_heroes = [0f64; 4];
                // Compute hero chance to get targeted
                for i in 0..5 {
                    if self.team.heroes[i].hp > 0 {
                        if i < 4 {
                            for ii in i..4 {
                                target_chance_heroes[ii] += self.team.heroes[i].threat;
                            }
                        }
                        target_chance_total += self.team.heroes[i].threat;
                    }
                }

                for i in 0..target_chance_heroes.len() {
                    target_chance_heroes[i] /= target_chance_total
                }
                update_target = false
            }

            // Check for sensei bonus and extreme crit bonus
            for hero in self.team.heroes {
                if hero.class == "Sensei" && hero.lost_innate == round - 2 {
                    hero.ninja_bonus = 0.1 + std::cmp::min(hero.innate_tier, 4) * 0.1;
                    hero.ninja_evasion = 0.15;
                    if hero.innate_tier == 3 {
                        hero.ninja_evasion = 0.20;
                    }
                    if hero.innate_tier == 4 {
                        hero.ninja_evasion = 0.25;
                    }
                }

                hero.extreme_crit_bonus = 0.0;

                if is_extreme && hero.evasion + hero.berserker_stage * 0.1 + hero.ninja_evasion < 0 {
                    hero.extreme_crit_bonus = -25 * (hero.evasion + hero.berserker_stage * 0.1 + hero.ninja_evasion);
                }
            }

            // Mob Attacks

            // Mob AOE
            if rng.gen::<f64>() < self.encounter.aoe_chance && heroes_alive > 1 {
                for hero in self.team.heroes {
                    if hero.hp > 0 {
                        if hero.guaranteed_evade || rng.gen::<f64>() < std::cmp::min(hero.evasion + hero.berserker_stage * 0.1 + hero.ninja_evasion, hero.evasion_cap) {
                            if hero.class == "Dancer" || hero.class == "Acrobat" {
                                hero.guaranteed_crit = true;
                            }
                        } else {
                            hero.hp -= (hero.damage_taken * self.encounter.aoe_damage).ceil();
                            if hero.hp <= 0 {
                                if rng.gen::<f64>() >= hero.survive_chance {
                                    // Surviving Fatal Blow did not activate
                                    if lord_present && lord_save && hero.class != "Lord" && self.team.heroes[lord_index] > 0 {
                                        // Lord Saves
                                        lord_save = false;
                                        hero.hp += (hero.damage_taken * self.encounter.aoe_damage).ceil();
                                        self.team.heroes[lord_index].hp -= (self.team.heroes[lord_index].damage_taken * self.encounter.aoe_damage).ceil();
                                        if self.team.heroes[lord_index].hp <= 0 {
                                            // Lord Dies in Saving
                                            if rng.gen::<f64>() >= self.team.heroes[lord_index].survive_chance {
                                                // Surviving Fatal Blow did not activate
                                                self.team.heroes[lord_index].hp = 0;
                                                heroes_alive -= 1;
                                                update_target = true;
                                            } else {
                                                // Surviving Fatal Blow Activated
                                                self.team.heroes[lord_index].hp = 1;
                                                self.team.heroes[lord_index].survive_chance = 0.0;
                                            }
                                        }
                                    } else {
                                        // Surviving Fatal Blow Activated
                                        hero.hp = 1;
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
                let target_rng = rng.gen::<f64>();
                for i in (0..target_chance_heroes).rev() {
                    if target_rng > target_chance_heroes[i] && self.team.heroes[i].hp > 0 {
                        // Hero i targeted
                        target = 4;
                        break;
                    }
                }

                // check hit/evade
                if hero.guaranteed_evade || rng.gen::<f64>() < std::cmp::min(hero.evasion + hero.berserker_stage * 0.1 + hero.ninja_evasion) {
                    if hero.class == "Danger" || hero.class == "Acrobat" {
                        hero.guaranteed_crit = true;
                    }
                } else {
                    // Hit, check crit
                    if rng.gen::<f64>() > self.encounter.crit_chance * self.encounter.crit_chance_modifier + hero.extreme_crit_bonus {
                        // not crit
                        hero.hp -= hero.damage_taken;
                    } else {
                        hero.hp -= hero.crit_damage_taken;
                    }

                    if hero.hp <= 0 {
                        if rng.gen::<f64>() >= hero.survive_chance {
                            // surviving fatal blow did not activate
                            if lord_present && lord_save && hero.class != "Lord" && self.team.heroes[lord_index] > 0 {
                                // Lord Saves
                                lord_save = false;
                                hero.hp += hero.damage_taken;
                                self.team.heroes[lord_index].hp -= self.team.heroes[lord_index].damage_taken;
                                if self.team.heroes[lord_index].hp <= 0 {
                                    // lord dies in saving
                                    if rng.gen::<f64>() >= self.team.heroes[lord_index].survive_chance {
                                        // surviving fatal blow did not activate
                                        self.team.heroes[lord_index].hp = 0;
                                        heroes_alive -= 1;
                                        update_target = true;
                                    } else {
                                        // survive fatal blow
                                        self.team.heroes[lord_index].hp = 1;
                                        self.team.heroes[lord_index].survive_chance = 0.0;
                                    }
                                }
                            } else {
                                // lord doesnt save
                                hero.hp = 0;
                                heroes_alive -= 1;
                                update_target = true;
                            }
                        } else {
                            // surviving fatal blow activated
                            hero.hp = 1;
                            hero.survive_chance = 0.0;
                        }
                    }

                    // check sensei lost innate
                    if hero.class = "Sensei" && hero.lost_innate != round - 1 {
                        hero.lost_innate = round;
                    }
                }
            }

            if self.team.champion == "Hemma" && self.team.heroes[hemma_index].hp > 0 {
                for (i, hero) in self.team.heroes.iter().enumerate() {
                    if i != hemma_index && hero.hp > (0.11 - 0.01 * self.team.champion_innate_tier) * hero.hp_max {
                        self.team.heroes[hemma_index].hemma_bonus += self.team.heroes[hemma_index] * hemma_mult;
                        hero.hp = hero.hp - (0.11 - 0.01 * self.team.champion_innate_tier) * hero.hp_max;
                        if hero.class == "Sensei" && hero.lost_innate != round - 1 {
                            hero.lost_innate = round;
                        }
                    }
                }
                self.team.heroes[hemma_index].hp = std::cmp::min(self.team.heroes[hemma_index] + 1 + self.team.champion_innate_tier + std::cmp::min(self.team.champion_innate_tier - 2, 0) + std::cmp::min(self.team.champion_innate_tier - 3, 0), self.team.heroes[hemma_index].hp_max);
            }

            for hero in self.team.heroes {
                // Check Berserker Activation
                if hero.class == "Berserker" || hero.class == "Jarl" {
                    if hero.hp >= hero.jarl_hp_stage_1 * hero.hp_max {
                        hero.berserker_stage = 0;
                    } else if hero.hp >= hero.jarl_hp_stage_2 * hero.hp_max {
                        hero.berserker_stage = 1;
                    } else if hero.hp >= hero.jarl_hp_stage_3 * hero.hp_max {
                        hero.berserker_stage = 2;
                    } else if hero.hp > 0 {
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

            // Heroes Attack
            for i in 0..self.team.heroes.len() {
                jj = attack_order[i];
                let hero = self.team.heroes[jj];

                if hero.hp > 0 {
                    if rng.gen::<f64>() > self.encounter.evasion {
                        // hit mob, check crit
                        if hero.guaranteed_crit || rng.gen::<f64>() < hero.crit_chance + hero.ninja_bonus + rudo_bonus {
                            // crit, if samurai variant ignore barrier else reduce damage by barrier mod
                            let mut damage = (hero.attack * (hero.attack_modifier + 0.2 * hero.mundra_qty + shark_active * 0.01 * hero.shark_qty + dinosaur_active * hero.dinosaur_qty * 0.01 + 0.1 * (1 + hero.berserker_level) * hero.berserker_stage) + hero.hemma_bonus) * (hero.critical_multiplier + hero.consecutive_crit_bonus);
                            if round != 1 || (hero.class != "Samurai" && hero.class != "Damiyo") {
                                damage *= self.encounter.barrier_modifier;
                            }
                            self.encounter.hp -= damage;
                            self.hero.damage_dealt += damage;
                            if hero.class == "Conquistador" {
                                hero.consecutive_crit_bonus = std::cmp::min(hero.consecutive_crit_bonus + 0.25, 1);
                            }
                        } else {
                            // not crit, deal damage
                            let mut damage = (hero.attack * (hero.attack_modifier + 0.2 * hero.mundra_qty + shark_active * 0.01 * hero.shark_qty + dinosaur_active * hero.dinosaur_qty * 0.01 + 0.1 * (1 + hero.berserker_level) * hero.berserker_stage) + hero.hemma_bonus) * self.encounter.barrier_modifier;
                            if hero.class == "Conquistador" {
                                hero.consecutive_crit_bonus = 0;
                            }
                            if count_loot {
                                if rng.gen::<f64>() < loot_chance {
                                    polonia_loot ++;
                                }
                            }

                            if self.encounter.barrier_hp > 0 && self.encounter.barrier_type == hero.element_type {
                                self.encounter.barrier_hp -= hero.element_qty;
                            } else if self.encounter.barrier_hp > 0 && hero.element_type == ElementType::Any {
                                self.encounter.barrier_hp -= hero.element_qty * 0.3;
                            }
                        }
                    }
                }

                if self.encounter.barrier_hp <= 0 {
                    self.encounter.barrier_modifier = 1.0;
                } else if self.encounter.barrier_hp <= 0.25 * self.encounter.barrier_hp_max {
                    self.encounter.barrier_modifier = 0.8;
                } else if self.encounter.barrier_hp <= 0.5 * self.encounter.barrier_hp_max {
                    self.encounter.barrier_modifier = 0.6;
                } else if self.encounter.barrier_hp <= 0.75 * self.encounter.barrier_hp_max {
                    self.encounter.barrier_modifier = 0.4;
                }

                if self.encounter.hp < self.encounter.hp_max / 2 {
                    shark_active = 1;
                }

                hero.guaranteed_crit = false;
            }

            dinosaur_active = 0;

            // Check won
            if self.encounter.hp <= 0 {
                cont_fight = false;
                won_fight = true;
            }

            // Check lost
            if heroes_alive == 0 {
                cont_fight = false;
            }

            // Calculate polonia loot
            if cont_fight == false {
                polonia_loot_total += std::cmp::min(polonia_loot, polonia_loot_cap);
                if polonia_loot >= polonia_loot_cap {
                    polonia_loot_cap_hit += 1;
                }
            }

            if self.team.champion_innate_tier == 1 && round == 2 {
                rudo_bonus = 0.0;
            }
            if (self.team.champion_innate_tier == 2 || self.team.champion_innate_tier == 3) && round == 3 {
                rudo_bonus = 0.0;
            }
            if self.team.champion_innate_tier == 4 && round == 4 {
                rudo_bonus = 0.0;
            }

            // Healing from Lizard, Cleric, and Lilo
            if cont_fight {
                for hero in self.team.heroes {
                    if hero.hp > 0 {
                        hero.hp = std::cmp::min(hero.hp + hero.lizard_qty * 3, hero.hp_max);
                        if hero.class == "Cleric" {
                            hero.hp = std::cmp::min(hero.hp + std::cmp::min(hero.innate_tier, 3) * 5 - 5, hero.hp_max);
                        } else if hero.class == "Bishop" {
                            if hero.innate_tier == 2 {
                                hero.hp = std::cmp::min(hero.hp + 5, hero.hp_max);
                            } else if hero.innate_tier >= 3 {
                                hero.hp = std::cmp::min(hero.hp + 20, hero.hp_max);
                            }
                        }

                        if self.team.champion == "Lilu" {
                            match self.team.champion_innate_tier == 1 {
                                1 => hero.hp = std::cmp::min(hero.hp + 3, hero.hp_max),
                                2 => hero.hp = std::cmp::min(hero.hp + 5, hero.hp_max),
                                3 => hero.hp = std::cmp::min(hero.hp + 10, hero.hp_max),
                                4 => hero.hp = std::cmp::min(hero.hp + 20, hero.hp_max),
                                _ => (),
                            }
                        }
                    }
                }
            }

            // Check Berserker Activation
            for hero in self.team.heroes {
                if hero.class == "Berserker" || hero.class == "Jarl" {
                    if hero.hp >= hero.jarl_hp_stage_1 * hero.hp_max {
                        hero.berserker_stage = 0;
                    } else if hero.hp >= hero.jarl_hp_stage_2 * hero.hp_max {
                        hero.berserker_stage = 1;
                    } else if hero.hp >= hero.jarl_hp_stage_3 * hero.hp_max {
                        hero.berserker_stage = 2;
                    } else if hero.hp > 0 {
                        hero.berserker_stage = 3;
                    }
                }
            }
        }

        // TODO If key in metrics then add else skip

        let res = SimResult {
            success: true,
            rounds_elapsed: 0i16,
            team_dmg_taken: vec![0i16],
            team_dmg_dealt: vec![0i16],
            team_dmg_dodged: vec![0i16],
            team_bonus_loot_qty: 0i8,
            team_rest_times: vec![0i32],
        };

        return Ok(res);
    }
}

/// Create a simulation performing type validation and calculating certain fields
pub fn create_simulation(
    team: &Team,
    encounter: Encounter,
    metrics: Vec<String>,
) -> Result<Simulation, &'static str> {
    let simulation = Simulation {
        team: team.clone(),
        encounter,
        metrics,
    };

    return Ok(simulation);
}

/// The result of a simulation
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct SimResult {
    success: bool,
    rounds_elapsed: i16,
    team_dmg_taken: Vec<i16>,
    team_dmg_dealt: Vec<i16>,
    team_dmg_dodged: Vec<i16>,
    team_bonus_loot_qty: i8,
    team_rest_times: Vec<i32>,
    // line 226+ for each hero:
    times_survived: Vec<u32>,
    damage_fight: Vec<u32>,
    damage_dealt_avg: Vec<u32>,
    damage_dealt_max: Vec<u32>,
    damage_dealt_min: Vec<u32>,
    hp_remaining_avg: Vec<u32>,
    hp_remaining_max: Vec<u32>,
    hp_remaining_min: Vec<u32>,
}

impl SimResult {
    pub fn is_success(&self) -> bool {
        return self.success;
    }
}
