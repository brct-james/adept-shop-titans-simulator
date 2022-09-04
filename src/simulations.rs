use super::dungeons::Encounter;
use super::heroes::Team;

use serde::{Deserialize, Serialize};

use rand::seq::SliceRandom;
use rand::thread_rng;

use log::info;

/// A simulated encounter between a Team and a Dungeon
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Simulation {
    team: Team,
    encounter: Encounter,
    metrics: Vec<String>,
    log_all: bool,
}

impl Simulation {
    pub fn run(&mut self) -> Result<SimResult, &'static str> {
        let mut log_queue: Vec<String> = vec![];
        log_queue.push("Start of Simulation".to_string());
        // If encounter.is_boss then ignore Mundra
        // Error if more heroes in team than encounter allows

        // Normalize %s
        let (is_extreme, is_boss) = self.encounter.is_extreme_or_boss();
        self.team.normalize_percents(is_extreme, is_boss);

        // Polonia Loot
        let mut polonia_loot_cap_hit = 0;
        let mut polonia_loot_total = 0;

        let (champion, champion_innate_tier) = self.team.get_champion_info();

        let (hemma_mult, count_loot, loot_chance, polonia_loot_cap) =
            self.team.apply_champion_and_booster_bonuses(is_boss);

        let encounter_defense_cap = self.encounter.get_defense_cap();
        let (encounter_damage, _) = self.encounter.get_damage_info();
        self.team
            .calculate_damage_from_encounter(encounter_defense_cap, encounter_damage);

        // PREVIOUS TO THIS IS SETUP, NOT RUN EACH SIMULATION, CONSIDER MOVING TO TRIALS CODE

        // Simulate Encounter
        let mut cont_fight = true;
        let mut won_fight = false;

        self.team
            .initialize_survive_chance_hemma_guaranteed_crit_and_berserker_stage();

        let mut update_target = true;
        let mut round = 0;
        let mut shark_active = 0;
        let mut dinosaur_active = 1;
        let mut lord_save = true;
        let mut rudo_bonus = 0f64;

        if champion == "Rudo" {
            match champion_innate_tier {
                1u8 => rudo_bonus = 0.3,
                2u8 => rudo_bonus = 0.4,
                3u8 => rudo_bonus = 0.4,
                4u8 => rudo_bonus = 0.5,
                _ => (),
            }
        }

        self.team.apply_class_special_effects();

        // Generate Random Attack Order
        let mut attack_order: Vec<usize> = (0..self.team.get_heroes_len()).collect();
        let mut rng = thread_rng();
        attack_order.shuffle(&mut rng);

        self.encounter.init_barrier_modifier();

        // Define targetting variables
        let mut target_chance_heroes = [0f64; 4];

        // Define heroes alive
        let mut heroes_alive = self.team.get_heroes_len();

        log_queue.push("Ready to start quest with:".to_string());
        log_queue.push(f!("{:#?}", self.encounter.round_floats_for_display()));
        log_queue.push(f!("{:#?}", self.team.round_floats_for_display()));

        // START QUEST
        while cont_fight {
            round += 1;
            let heroes_hp_strings = self.team.get_heroes_hp_as_strings();
            let (temp_ehp, temp_mehp) = self.encounter.get_hp_info();
            log_queue.push(f!(
                "\n\n--Round #{:#?}--\nEncounter HP: {:.2} ({:.2}%)  Heroes HP: {}\n",
                round,
                temp_ehp,
                temp_ehp / temp_mehp * 100.0,
                heroes_hp_strings
            ));

            if update_target {
                target_chance_heroes = self.team.calculate_targeting_chances();
                update_target = false;
            }
            log_queue.push(f!("Team Target Chances: {:?}", target_chance_heroes));

            // Check for sensei bonus and extreme crit bonus
            log_queue.push("Updating Ninja Bonus and Extreme Crit Bonus for Team".to_string());
            let update_ninja_extreme_bonuses_logs = self
                .team
                .update_ninja_bonus_and_extreme_crit_bonus(round, is_extreme);
            log_queue.extend(update_ninja_extreme_bonuses_logs);

            // Mob Attacks

            // Mob AOE
            let (aoe_chance, aoe_damage) = self.encounter.get_aoe_info();
            let (crit_chance, crit_chance_modifier) = self.encounter.get_crit_info();
            let (temp1, temp2, temp3, temp4) = self.team.calculate_mob_attack(
                aoe_chance,
                aoe_damage,
                heroes_alive,
                lord_save,
                round,
                update_target,
                target_chance_heroes,
                crit_chance,
                crit_chance_modifier,
            );
            heroes_alive = temp1;
            lord_save = temp2;
            update_target = temp3;
            log_queue.extend(temp4);

            if champion == "Hemma" {
                let hemma_log_queue =
                    self.team
                        .calculate_hemma_drain(champion_innate_tier, hemma_mult, round);
                log_queue.extend(hemma_log_queue);
            }

            let bnsroundeffects_log_queue = self
                .team
                .calculate_berserker_ninja_samurai_round_effects(round);
            log_queue.extend(bnsroundeffects_log_queue);

            // Heroes Attack
            let (barrier_hp, barrier_hp_max, barrier_modifier, barrier_type) =
                self.encounter.get_barrier_info();
            let encounter_evasion = self.encounter.get_evasion();
            let (encounter_hp, encounter_hp_max) = self.encounter.get_hp_info();
            let (
                polonia_loot,
                barrier_modifier,
                barrier_hp,
                encounter_hp,
                temp1,
                hero_attack_log_queue,
            ) = self.team.calculate_heroes_attack(
                attack_order.clone(),
                round,
                rudo_bonus,
                shark_active,
                dinosaur_active,
                barrier_modifier,
                count_loot,
                loot_chance,
                encounter_evasion,
                encounter_hp,
                barrier_hp,
                barrier_hp_max,
                encounter_hp_max,
                barrier_type,
            );
            shark_active = temp1;
            log_queue.extend(hero_attack_log_queue);

            self.encounter
                .set_barrier_hp_and_modifier(barrier_hp, barrier_modifier);
            self.encounter.set_hp(encounter_hp);
            log_queue.push("(Meta-Info) Barrier HP, Modifier and Encounter HP have been applied back to their objects".to_string());

            dinosaur_active = 0;

            // Check won
            if encounter_hp <= 0.0 {
                cont_fight = false;
                won_fight = true;
                log_queue.push("Mob reduced to 0 HP".to_string());
            }

            // Check lost
            if heroes_alive == 0 {
                cont_fight = false;
                log_queue.push("No heroes remain alive".to_string());
            }

            // Calculate polonia loot
            if cont_fight == false {
                polonia_loot_total += std::cmp::min(polonia_loot, polonia_loot_cap);
                if polonia_loot >= polonia_loot_cap {
                    polonia_loot_cap_hit += 1;
                }
                log_queue.push(f!(
                    "Polonia loot received {} of {}",
                    polonia_loot,
                    polonia_loot_cap
                ));
            }

            if champion_innate_tier == 1 && round == 2 {
                rudo_bonus = 0.0;
            }
            if (champion_innate_tier == 2 || champion_innate_tier == 3) && round == 3 {
                rudo_bonus = 0.0;
            }
            if champion_innate_tier == 4 && round == 4 {
                rudo_bonus = 0.0;
            }
            if champion == "Rudo" {
                log_queue.push(f!(
                    "Round is {}, Rudo bonus to break chance is: {}",
                    round,
                    rudo_bonus
                ));
            }

            // Healing from Lizard, Cleric, and Lilo
            if cont_fight {
                let healing_log_queue = self
                    .team
                    .calculate_healing(champion.clone(), champion_innate_tier);
                log_queue.extend(healing_log_queue);
            }

            // Check Berserker Activation
            let berserker_log_queue = self.team.check_berserker_activation();
            log_queue.extend(berserker_log_queue);
        }

        // TODO If key in metrics then add else skip
        let (ehprem, emaxhp) = self.encounter.get_hp_info();
        let (team_crits_taken, team_crits_dealt, team_dodges, team_attacks_missed) =
            self.team.get_heroes_accuracy_stats();
        let res = SimResult {
            success: won_fight,
            rounds_elapsed: round,
            team_dmg_taken: vec![0i16],
            team_dmg_dealt: vec![0i16],
            team_dmg_dodged: vec![0i16],
            team_bonus_loot_qty: 0i8,
            team_rest_times: vec![0i32],
            times_survived: vec![0u32],
            damage_dealt_during_fight: self.team.get_team_damage_dealt_total(),
            damage_dealt_avg: vec![0u32],
            damage_dealt_max: vec![0u32],
            damage_dealt_min: vec![0u32],
            hp_remaining_avg: vec![0u32],
            hp_remaining_max: vec![0u32],
            hp_remaining_min: vec![0u32],
            team: self.team.clone(),
            encounter: self.encounter.clone(),
            polonia_loot_total,
            polonia_loot_cap_hit,
            encounter_hp_remaining: ehprem,
            encounter_max_hp: emaxhp,
            team_crits_taken,
            team_crits_dealt,
            team_dodges,
            team_attacks_missed,
        };

        if won_fight {
            log_queue.push("Won Simulation".to_string());
        } else {
            log_queue.push("Lost Simulation".to_string());
        }

        if self.log_all || !won_fight {
            for item in log_queue {
                info!("{}", item);
            }
        }
        return Ok(res);
    }
}

/// Create a simulation performing type validation and calculating certain fields
/// If log_all is false simulation actions are only logged when the simulation fails, else all actions are logged
pub fn create_simulation(
    team: &Team,
    encounter: Encounter,
    metrics: Vec<String>,
    log_all: bool,
) -> Result<Simulation, &'static str> {
    let simulation = Simulation {
        team: team.clone(),
        encounter,
        metrics,
        log_all,
    };

    return Ok(simulation);
}

/// The result of a simulation
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    damage_dealt_during_fight: Vec<f64>,
    damage_dealt_avg: Vec<u32>,
    damage_dealt_max: Vec<u32>,
    damage_dealt_min: Vec<u32>,
    hp_remaining_avg: Vec<u32>,
    hp_remaining_max: Vec<u32>,
    hp_remaining_min: Vec<u32>,
    // other
    team: Team,
    encounter: Encounter,
    polonia_loot_total: u8,
    polonia_loot_cap_hit: i32,
    encounter_hp_remaining: f64,
    encounter_max_hp: f64,
    // team accuracy stats
    team_crits_taken: Vec<u8>,
    team_crits_dealt: Vec<u8>,
    team_dodges: Vec<u8>,
    team_attacks_missed: Vec<u8>,
}

impl SimResult {
    pub fn is_success(&self) -> bool {
        return self.success;
    }

    pub fn get_rounds(&self) -> i16 {
        return self.rounds_elapsed;
    }

    // pub fn print_team(&self) {
    //     println!("{:#?}", self.team);
    // }

    // pub fn print_encounter(&self) {
    //     println!("{:#?}", self.encounter);
    // }

    // Todo: Deprecate
    pub fn get_damage_dealt_during_fight(&self) -> Vec<f64> {
        return self.damage_dealt_during_fight.clone();
    }

    pub fn get_encounter_hp_remaining(&self) -> f64 {
        return self.encounter_hp_remaining;
    }

    pub fn get_team_hp_remaining(&self) -> [f64; 5] {
        return convert_vec_to_max_team_sized_array(self.team.get_heroes_hp());
    }

    pub fn get_team_damage_dealt(&self) -> [f64; 5] {
        return convert_vec_to_max_team_sized_array(self.get_damage_dealt_during_fight());
    }

    pub fn get_team_crits_taken(&self) -> [u8; 5] {
        return convert_vec_to_max_team_sized_array(self.team.get_heroes_accuracy_stats().0);
    }

    pub fn get_team_crits_dealt(&self) -> [u8; 5] {
        return convert_vec_to_max_team_sized_array(self.team.get_heroes_accuracy_stats().1);
    }

    pub fn get_team_dodges(&self) -> [u8; 5] {
        return convert_vec_to_max_team_sized_array(self.team.get_heroes_accuracy_stats().2);
    }

    pub fn get_team_attacks_missed(&self) -> [u8; 5] {
        return convert_vec_to_max_team_sized_array(self.team.get_heroes_accuracy_stats().3);
    }
}

/// input_vector is converted to an array sized to match the max team size.
/// Excess values are truncated
fn convert_vec_to_max_team_sized_array<T: Default + Clone>(input_vector: Vec<T>) -> [T; 5] {
    let mut res_array: [T; 5] = Default::default();
    for i in 0..res_array.len() {
        if i >= input_vector.len() {
            // ran out of input values, initialize rest as 0s
            res_array[i] = T::default();
        } else {
            res_array[i] = input_vector[i].clone();
        }
    }
    return res_array;
}
