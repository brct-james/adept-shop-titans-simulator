use std::collections::HashMap;

use crate::decimals::{round_to_2, round_to_4};

use super::dungeons::Dungeon;
use super::heroes::Team;
use super::simulations::{create_simulation, SimResult};

use log::info;
use serde::{Deserialize, Serialize};

extern crate csv;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct TrialCSVRecord {
    trial_identifier: String,
    simulation_identifier: String,
    success: bool,
    rounds: i16,
    encounter_hp_remaining: f64,

    hp_remaining_hero_1: f64,
    dmg_dealt_hero_1: f64,
    crits_taken_hero_1: u8,
    crits_dealt_hero_1: u8,
    dodges_hero_1: u8,
    attacks_missed_hero_1: u8,

    hp_remaining_hero_2: f64,
    dmg_dealt_hero_2: f64,
    crits_taken_hero_2: u8,
    crits_dealt_hero_2: u8,
    dodges_hero_2: u8,
    attacks_missed_hero_2: u8,

    hp_remaining_hero_3: f64,
    dmg_dealt_hero_3: f64,
    crits_taken_hero_3: u8,
    crits_dealt_hero_3: u8,
    dodges_hero_3: u8,
    attacks_missed_hero_3: u8,

    hp_remaining_hero_4: f64,
    dmg_dealt_hero_4: f64,
    crits_taken_hero_4: u8,
    crits_dealt_hero_4: u8,
    dodges_hero_4: u8,
    attacks_missed_hero_4: u8,

    hp_remaining_hero_5: f64,
    dmg_dealt_hero_5: f64,
    crits_taken_hero_5: u8,
    crits_dealt_hero_5: u8,
    dodges_hero_5: u8,
    attacks_missed_hero_5: u8,
}

impl TrialCSVRecord {
    pub fn _round_floats_for_display(&self) -> TrialCSVRecord {
        let mut tcr2 = self.clone();
        tcr2.encounter_hp_remaining = round_to_2(tcr2.encounter_hp_remaining);

        tcr2.hp_remaining_hero_1 = round_to_2(tcr2.hp_remaining_hero_1);
        tcr2.dmg_dealt_hero_1 = round_to_2(tcr2.dmg_dealt_hero_1);

        tcr2.hp_remaining_hero_2 = round_to_2(tcr2.hp_remaining_hero_2);
        tcr2.dmg_dealt_hero_2 = round_to_2(tcr2.dmg_dealt_hero_2);

        tcr2.hp_remaining_hero_3 = round_to_2(tcr2.hp_remaining_hero_3);
        tcr2.dmg_dealt_hero_3 = round_to_2(tcr2.dmg_dealt_hero_3);

        tcr2.hp_remaining_hero_4 = round_to_2(tcr2.hp_remaining_hero_4);
        tcr2.dmg_dealt_hero_4 = round_to_2(tcr2.dmg_dealt_hero_4);

        tcr2.hp_remaining_hero_5 = round_to_2(tcr2.hp_remaining_hero_5);
        tcr2.dmg_dealt_hero_5 = round_to_2(tcr2.dmg_dealt_hero_5);

        return tcr2;
    }
}

/// Create a trial csv record performing type validation and calculating certain fields
fn _create_trial_csv_record(
    trial_identifier: String,
    simulation_identifier: String,
    result: bool,
    rounds: i16,
    encounter_hp_remaining: f64,
    team_hp_remaining: [f64; 5],
    team_dmg_dealt: [f64; 5],
    team_crits_taken: [u8; 5],
    team_crits_dealt: [u8; 5],
    team_dodges: [u8; 5],
    team_attacks_missed: [u8; 5],
) -> TrialCSVRecord {
    let t_csv_rec = TrialCSVRecord {
        trial_identifier,
        simulation_identifier,
        success: result,
        rounds,
        encounter_hp_remaining,

        hp_remaining_hero_1: team_hp_remaining[0],
        dmg_dealt_hero_1: team_dmg_dealt[0],
        crits_taken_hero_1: team_crits_taken[0],
        crits_dealt_hero_1: team_crits_dealt[0],
        dodges_hero_1: team_dodges[0],
        attacks_missed_hero_1: team_attacks_missed[0],

        hp_remaining_hero_2: team_hp_remaining[1],
        dmg_dealt_hero_2: team_dmg_dealt[1],
        crits_taken_hero_2: team_crits_taken[1],
        crits_dealt_hero_2: team_crits_dealt[1],
        dodges_hero_2: team_dodges[1],
        attacks_missed_hero_2: team_attacks_missed[1],

        hp_remaining_hero_3: team_hp_remaining[2],
        dmg_dealt_hero_3: team_dmg_dealt[2],
        crits_taken_hero_3: team_crits_taken[2],
        crits_dealt_hero_3: team_crits_dealt[2],
        dodges_hero_3: team_dodges[2],
        attacks_missed_hero_3: team_attacks_missed[2],

        hp_remaining_hero_4: team_hp_remaining[3],
        dmg_dealt_hero_4: team_dmg_dealt[3],
        crits_taken_hero_4: team_crits_taken[3],
        crits_dealt_hero_4: team_crits_dealt[3],
        dodges_hero_4: team_dodges[3],
        attacks_missed_hero_4: team_attacks_missed[3],

        hp_remaining_hero_5: team_hp_remaining[4],
        dmg_dealt_hero_5: team_dmg_dealt[4],
        crits_taken_hero_5: team_crits_taken[4],
        crits_dealt_hero_5: team_crits_dealt[4],
        dodges_hero_5: team_dodges[4],
        attacks_missed_hero_5: team_attacks_missed[4],
    };

    return t_csv_rec;
}

/// Defines instructions for running one or more Simulations
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Trial {
    identifier: String,
    description: String,
    simulation_qty: usize,
    team: Team,
    dungeon: Dungeon,
    difficulty_settings: Vec<usize>,
    force_minibosses: Option<bool>,
    results: Vec<SimResult>,
    log_all: bool,
}

impl Trial {
    pub fn run_simulations_single_threaded(&mut self) {
        while self.results.len() < self.simulation_qty {
            // let timer = Instant::now();
            // print!("Running simulation iteration:  # {:#?}", self.results.len());
            if self.log_all == true {
                info!(
                    "\n\nRunning simulation iteration: # {}\n",
                    self.results.len()
                );
            }
            let encounter = self
                .dungeon
                .generate_encounter_from_dungeon(&self.difficulty_settings, self.force_minibosses)
                .unwrap();
            let mut simulation =
                create_simulation(&self.team, encounter, vec![], self.log_all).unwrap();
            let sim_res = simulation.run().unwrap();
            // print!(
            //     "\rRunning simulation iteration: # {:#?} | Success: {:#?} in {:#?} rounds | Took {:#?}ms\n",
            //     self.results.len(),
            //     sim_res.is_success(),
            //     sim_res.get_rounds(),
            //     timer.elapsed().as_nanos() as f32 / 1000000.0f32,
            // );
            self.results.push(sim_res);
        }
    }
    pub fn _get_results_unranked(&self) -> Vec<SimResult> {
        return self.results.clone();
    }
    pub fn _save_results_to_csv(&self, path: String) -> Result<(), std::io::Error> {
        let mut wtr = csv::Writer::from_path(path)?;

        for (i, res) in self.results.iter().enumerate() {
            let record = _create_trial_csv_record(
                self.identifier.to_string(),
                i.to_string(),
                res.is_success(),
                res.get_rounds(),
                res.get_encounter_hp_remaining(),
                res.get_team_hp_remaining(),
                res.get_team_damage_dealt(),
                res.get_team_crits_taken(),
                res.get_team_crits_dealt(),
                res.get_team_dodges(),
                res.get_team_attacks_missed(),
            );
            wtr.serialize(record._round_floats_for_display())?;
        }

        wtr.flush()?;
        return Ok(());
    }

    /// Create a trial result, performing type validation and calculating certain fields
    pub fn create_trial_result(&self) -> TrialResult {
        let all_results: Vec<SimResult> = self.results.clone();
        let miniboss_results: Vec<SimResult> = self
            .results
            .iter()
            .filter(|res| res.get_encounter().is_miniboss())
            .cloned()
            .collect();

        let mut all_results_length = all_results.len();
        let mut miniboss_results_length = miniboss_results.len();

        if all_results_length == 0 {
            all_results_length = 1
        }
        if miniboss_results_length == 0 {
            miniboss_results_length = 1
        }

        let mut vec_hero_survival_rate: [Vec<f64>; 5] = Default::default();
        let mut vec_hero_avg_hp_remaining: [Vec<f64>; 5] = Default::default();
        let mut vec_hero_avg_dmg: [Vec<f64>; 5] = Default::default();
        let mut vec_hero_avg_dodges: [Vec<f64>; 5] = Default::default();
        let mut vec_hero_avg_atk_accuracy: [Vec<f64>; 5] = Default::default();
        let mut vec_hero_avg_crits_dealt: [Vec<f64>; 5] = Default::default();
        let mut vec_hero_avg_crits_taken: [Vec<f64>; 5] = Default::default();

        for res in all_results.iter() {
            let sim_rounds = res.get_rounds() as f64;

            let team_hp_rem = res.get_team_hp_remaining();
            let team_dmg_dealt = res.get_team_damage_dealt();
            let team_dodges = res.get_team_dodges();
            let team_atks_missed = res.get_team_attacks_missed();
            let team_crits_dealt = res.get_team_crits_dealt();
            let team_crits_taken = res.get_team_crits_taken();
            for i in 0..5 {
                let mut survived: f64 = 0.0;
                if team_hp_rem[i] > 0.0 {
                    survived = 1.0;
                }
                vec_hero_survival_rate[i].push(survived);
                vec_hero_avg_hp_remaining[i].push(team_hp_rem[i]);
                vec_hero_avg_dmg[i].push(team_dmg_dealt[i]);
                vec_hero_avg_dodges[i].push((sim_rounds - (team_dodges[i] as f64)) / sim_rounds);
                vec_hero_avg_atk_accuracy[i]
                    .push((sim_rounds - (team_atks_missed[i] as f64)) / sim_rounds);
                vec_hero_avg_crits_dealt[i]
                    .push((sim_rounds - (team_crits_dealt[i] as f64)) / sim_rounds);
                vec_hero_avg_crits_taken[i]
                    .push((sim_rounds - (team_crits_taken[i] as f64)) / sim_rounds);
            }
        }

        let hero_names: Vec<String> = all_results[0].get_team().get_team_hero_names();
        let hero_survival_rate: [f64; 5] = vec_hero_survival_rate
            .iter()
            .map(|sr| sr.iter().sum::<f64>() / sr.len() as f64)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        let hero_avg_hp_remaining: [f64; 5] = vec_hero_avg_hp_remaining
            .iter()
            .map(|hp| hp.iter().sum::<f64>() / hp.len() as f64)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        let hero_avg_dmg: [f64; 5] = vec_hero_avg_dmg
            .iter()
            .map(|dmg| dmg.iter().sum::<f64>() / dmg.len() as f64)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        let hero_avg_dodge_rate: [f64; 5] = vec_hero_avg_dodges
            .iter()
            .map(|dg| dg.iter().sum::<f64>() / dg.len() as f64)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        let hero_avg_atk_hit_rate: [f64; 5] = vec_hero_avg_atk_accuracy
            .iter()
            .map(|acc| acc.iter().sum::<f64>() / acc.len() as f64)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        let hero_avg_crit_dealt_rate: [f64; 5] = vec_hero_avg_crits_dealt
            .iter()
            .map(|cd| cd.iter().sum::<f64>() / cd.len() as f64)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        let hero_avg_crit_taken_rate: [f64; 5] = vec_hero_avg_crits_taken
            .iter()
            .map(|ct| ct.iter().sum::<f64>() / ct.len() as f64)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();

        let trial_result = TrialResult {
            trial_identifier: self.identifier.to_string(),
            trial_description: self.description.to_string(),
            trial_simulation_qty: self.simulation_qty,
            dungeon_identifier: self.dungeon._get_zone(),
            difficulty_settings: self.difficulty_settings.clone(),
            force_minibosses: self.force_minibosses,
            trial_num_minibosses: miniboss_results.len(),
            success_rate: (all_results
                .iter()
                .map(|res| res.is_success() as u32 as f64)
                .sum::<f64>()
                / all_results_length as f64),
            success_rate_vs_miniboss: (miniboss_results
                .iter()
                .map(|res| res.is_success() as u32 as f64)
                .sum::<f64>()
                / miniboss_results_length as f64),
            average_rounds: (all_results
                .iter()
                .map(|res| res.get_rounds() as u32 as f64)
                .sum::<f64>()
                / all_results_length as f64),
            avg_rounds_vs_miniboss: (miniboss_results
                .iter()
                .map(|res| res.get_rounds() as u32 as f64)
                .sum::<f64>()
                / miniboss_results_length as f64),
            avg_encounter_hp_remaining: (all_results
                .iter()
                .map(|res| res.get_encounter_hp_remaining() as f64)
                .sum::<f64>()
                / all_results_length as f64),
            avg_encounter_hp_remaining_vs_miniboss: (miniboss_results
                .iter()
                .map(|res| res.get_encounter_hp_remaining() as f64)
                .sum::<f64>()
                / miniboss_results_length as f64),

            hero_names,
            hero_survival_rate,
            hero_avg_hp_remaining,
            hero_avg_dmg,

            hero_avg_dodge_rate,
            hero_avg_atk_hit_rate,
            hero_avg_crit_dealt_rate,
            hero_avg_crit_taken_rate,
        };

        return trial_result;
    }

    pub fn save_trial_result_to_csv(&self, string_path: String) -> Result<(), std::io::Error> {
        let path = std::path::Path::new(&string_path);
        let path_exists = path.exists();

        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(path)
            .unwrap();

        let mut wtr: csv::Writer<std::fs::File>;
        if path_exists {
            wtr = csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(file);
        } else {
            wtr = csv::WriterBuilder::new()
                .has_headers(true)
                .from_writer(file);
        }

        let trial_result = self.create_trial_result();

        let record = create_trial_result_csv_record_from_trial_result(trial_result);

        wtr.serialize(record.round_floats_for_display())?;

        wtr.flush()?;
        return Ok(());
    }

    pub fn save_duo_skillz_and_trial_result_to_csv(
        &self,
        duo_skillz_path: String,
        trial_results_path: String,
        skill_abbreviation_map: HashMap<String, String>,
    ) -> Result<(), std::io::Error> {
        // Save Trial Result
        self.save_trial_result_to_csv(trial_results_path).unwrap();

        // Save Duo Skillz Result
        let path = std::path::Path::new(&duo_skillz_path);
        let path_exists = path.exists();

        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(path)
            .unwrap();

        let mut wtr: csv::Writer<std::fs::File>;
        if path_exists {
            wtr = csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(file);
        } else {
            wtr = csv::WriterBuilder::new()
                .has_headers(true)
                .from_writer(file);
        }

        let trial_result = self.create_trial_result();

        let record = create_peetee_duoskillz_trial_result_csv_record_from_trial_result(
            trial_result,
            skill_abbreviation_map,
        );

        wtr.serialize(record.round_floats_for_display())?;

        wtr.flush()?;
        return Ok(());
    }
}

/// Create a trial performing type validation and calculating certain fields
pub fn create_trial(
    identifier: String,
    description: String,
    simulation_qty: usize,
    team: Team,
    dungeon: Dungeon,
    difficulty_settings: Vec<usize>,
    force_minibosses: Option<bool>,
    log_all: bool,
) -> Result<Trial, &'static str> {
    if simulation_qty < 1 {
        return Err("simulation_qty must be > 0");
    }

    let trial = Trial {
        identifier,
        description,
        simulation_qty,
        team,
        dungeon,
        difficulty_settings,
        force_minibosses,
        results: Vec::with_capacity(simulation_qty),
        log_all,
    };

    return Ok(trial);
}

/// The result of a trial
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TrialResult {
    trial_identifier: String,
    trial_description: String,
    trial_simulation_qty: usize,
    dungeon_identifier: String,
    difficulty_settings: Vec<usize>,
    force_minibosses: Option<bool>,
    trial_num_minibosses: usize,
    success_rate: f64,
    success_rate_vs_miniboss: f64,
    average_rounds: f64,
    avg_rounds_vs_miniboss: f64,
    avg_encounter_hp_remaining: f64,
    avg_encounter_hp_remaining_vs_miniboss: f64,

    hero_names: Vec<String>,
    hero_survival_rate: [f64; 5],
    hero_avg_hp_remaining: [f64; 5],
    hero_avg_dmg: [f64; 5],

    hero_avg_dodge_rate: [f64; 5],
    hero_avg_atk_hit_rate: [f64; 5],
    hero_avg_crit_dealt_rate: [f64; 5],
    hero_avg_crit_taken_rate: [f64; 5],
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct TrialResultCSVRecord {
    trial_identifier: String,
    trial_description: String,
    trial_simulation_qty: usize,
    dungeon_identifier: String,
    difficulty_settings: String,
    force_minibosses: String,
    trial_num_minibosses: usize,
    success_rate: String,
    success_rate_vs_miniboss: String,
    average_rounds: f64,
    avg_rounds_vs_miniboss: f64,
    avg_encounter_hp_remaining: f64,
    avg_encounter_hp_remaining_vs_miniboss: f64,

    hero_1_identifier: String,
    hero_1_survival_rate: String,
    hero_1_avg_hp_remaining: f64,
    hero_1_avg_dmg: f64,
    hero_1_avg_dodge_rate: f64,
    hero_1_avg_atk_hit_rate: f64,
    hero_1_avg_crit_dealt_rate: f64,
    hero_1_avg_crit_taken_rate: f64,

    hero_2_identifier: String,
    hero_2_survival_rate: String,
    hero_2_avg_hp_remaining: f64,
    hero_2_avg_dmg: f64,
    hero_2_avg_dodge_rate: f64,
    hero_2_avg_atk_hit_rate: f64,
    hero_2_avg_crit_dealt_rate: f64,
    hero_2_avg_crit_taken_rate: f64,

    hero_3_identifier: String,
    hero_3_survival_rate: String,
    hero_3_avg_hp_remaining: f64,
    hero_3_avg_dmg: f64,
    hero_3_avg_dodge_rate: f64,
    hero_3_avg_atk_hit_rate: f64,
    hero_3_avg_crit_dealt_rate: f64,
    hero_3_avg_crit_taken_rate: f64,

    hero_4_identifier: String,
    hero_4_survival_rate: String,
    hero_4_avg_hp_remaining: f64,
    hero_4_avg_dmg: f64,
    hero_4_avg_dodge_rate: f64,
    hero_4_avg_atk_hit_rate: f64,
    hero_4_avg_crit_dealt_rate: f64,
    hero_4_avg_crit_taken_rate: f64,

    hero_5_identifier: String,
    hero_5_survival_rate: String,
    hero_5_avg_hp_remaining: f64,
    hero_5_avg_dmg: f64,
    hero_5_avg_dodge_rate: f64,
    hero_5_avg_atk_hit_rate: f64,
    hero_5_avg_crit_dealt_rate: f64,
    hero_5_avg_crit_taken_rate: f64,
}

impl TrialResultCSVRecord {
    pub fn round_floats_for_display(&self) -> TrialResultCSVRecord {
        let mut tcr2 = self.clone();

        tcr2.average_rounds = round_to_4(tcr2.average_rounds);
        tcr2.avg_rounds_vs_miniboss = round_to_4(tcr2.avg_rounds_vs_miniboss);
        tcr2.avg_encounter_hp_remaining = round_to_2(tcr2.avg_encounter_hp_remaining);
        tcr2.avg_encounter_hp_remaining_vs_miniboss =
            round_to_2(tcr2.avg_encounter_hp_remaining_vs_miniboss);

        tcr2.hero_1_avg_hp_remaining = round_to_2(tcr2.hero_1_avg_hp_remaining);
        tcr2.hero_1_avg_dmg = round_to_2(tcr2.hero_1_avg_dmg);
        tcr2.hero_1_avg_dodge_rate = round_to_2(tcr2.hero_1_avg_dodge_rate);
        tcr2.hero_1_avg_atk_hit_rate = round_to_2(tcr2.hero_1_avg_atk_hit_rate);
        tcr2.hero_1_avg_crit_dealt_rate = round_to_2(tcr2.hero_1_avg_crit_dealt_rate);
        tcr2.hero_1_avg_crit_taken_rate = round_to_2(tcr2.hero_1_avg_crit_taken_rate);

        tcr2.hero_2_avg_hp_remaining = round_to_2(tcr2.hero_2_avg_hp_remaining);
        tcr2.hero_2_avg_dmg = round_to_2(tcr2.hero_2_avg_dmg);
        tcr2.hero_2_avg_dodge_rate = round_to_2(tcr2.hero_2_avg_dodge_rate);
        tcr2.hero_2_avg_atk_hit_rate = round_to_2(tcr2.hero_2_avg_atk_hit_rate);
        tcr2.hero_2_avg_crit_dealt_rate = round_to_2(tcr2.hero_2_avg_crit_dealt_rate);
        tcr2.hero_2_avg_crit_taken_rate = round_to_2(tcr2.hero_2_avg_crit_taken_rate);

        tcr2.hero_3_avg_hp_remaining = round_to_2(tcr2.hero_3_avg_hp_remaining);
        tcr2.hero_3_avg_dmg = round_to_2(tcr2.hero_3_avg_dmg);
        tcr2.hero_3_avg_dodge_rate = round_to_2(tcr2.hero_3_avg_dodge_rate);
        tcr2.hero_3_avg_atk_hit_rate = round_to_2(tcr2.hero_3_avg_atk_hit_rate);
        tcr2.hero_3_avg_crit_dealt_rate = round_to_2(tcr2.hero_3_avg_crit_dealt_rate);
        tcr2.hero_3_avg_crit_taken_rate = round_to_2(tcr2.hero_3_avg_crit_taken_rate);

        tcr2.hero_4_avg_hp_remaining = round_to_2(tcr2.hero_4_avg_hp_remaining);
        tcr2.hero_4_avg_dmg = round_to_2(tcr2.hero_4_avg_dmg);
        tcr2.hero_4_avg_dodge_rate = round_to_2(tcr2.hero_4_avg_dodge_rate);
        tcr2.hero_4_avg_atk_hit_rate = round_to_2(tcr2.hero_4_avg_atk_hit_rate);
        tcr2.hero_4_avg_crit_dealt_rate = round_to_2(tcr2.hero_4_avg_crit_dealt_rate);
        tcr2.hero_4_avg_crit_taken_rate = round_to_2(tcr2.hero_4_avg_crit_taken_rate);

        tcr2.hero_5_avg_hp_remaining = round_to_2(tcr2.hero_5_avg_hp_remaining);
        tcr2.hero_5_avg_dmg = round_to_2(tcr2.hero_5_avg_dmg);
        tcr2.hero_5_avg_dodge_rate = round_to_2(tcr2.hero_5_avg_dodge_rate);
        tcr2.hero_5_avg_atk_hit_rate = round_to_2(tcr2.hero_5_avg_atk_hit_rate);
        tcr2.hero_5_avg_crit_dealt_rate = round_to_2(tcr2.hero_5_avg_crit_dealt_rate);
        tcr2.hero_5_avg_crit_taken_rate = round_to_2(tcr2.hero_5_avg_crit_taken_rate);

        return tcr2;
    }
}

/// Create a trial csv record performing type validation and calculating certain fields
fn create_trial_result_csv_record_from_trial_result(result: TrialResult) -> TrialResultCSVRecord {
    let mut new_diff_settings: Vec<&str> = Default::default();
    let diff_map: std::collections::HashMap<usize, &str> = std::collections::HashMap::from([
        (1 as usize, "Easy"),
        (2 as usize, "Medium"),
        (3 as usize, "Hard"),
        (4 as usize, "Extreme"),
        (5 as usize, "Boss Easy"),
        (6 as usize, "Boss Medium"),
        (7 as usize, "Boss Hard"),
        (8 as usize, "Boss Extreme"),
    ]);

    for diff in result.difficulty_settings {
        new_diff_settings.push(diff_map[&diff]);
    }

    let new_force_miniboss: String;

    match result.force_minibosses {
        Some(setting) => {
            if setting == true {
                new_force_miniboss = String::from("Force Only Minibosses")
            } else {
                new_force_miniboss = String::from("No Minibosses Allowed")
            }
        }
        None => new_force_miniboss = String::from("Minibosses Allowed with Random Chance"),
    }

    let t_csv_rec = TrialResultCSVRecord {
        trial_identifier: result.trial_identifier,
        trial_description: result.trial_description,
        trial_simulation_qty: result.trial_simulation_qty,
        dungeon_identifier: result.dungeon_identifier,
        difficulty_settings: format!("{:?}", new_diff_settings),
        force_minibosses: new_force_miniboss,
        trial_num_minibosses: result.trial_num_minibosses,
        success_rate: f!("{:.4}", round_to_4(result.success_rate)),
        success_rate_vs_miniboss: f!("{:.4}", round_to_4(result.success_rate_vs_miniboss)),
        average_rounds: result.average_rounds,
        avg_rounds_vs_miniboss: result.avg_rounds_vs_miniboss,
        avg_encounter_hp_remaining: result.avg_encounter_hp_remaining,
        avg_encounter_hp_remaining_vs_miniboss: result.avg_encounter_hp_remaining_vs_miniboss,

        hero_1_identifier: result
            .hero_names
            .get(0)
            .unwrap_or(&String::from(""))
            .to_string(),
        hero_1_survival_rate: f!("{:.4}", round_to_4(result.hero_survival_rate[0])),
        hero_1_avg_hp_remaining: result.hero_avg_hp_remaining[0],
        hero_1_avg_dmg: result.hero_avg_dmg[0],
        hero_1_avg_dodge_rate: result.hero_avg_dodge_rate[0],
        hero_1_avg_atk_hit_rate: result.hero_avg_atk_hit_rate[0],
        hero_1_avg_crit_dealt_rate: result.hero_avg_crit_dealt_rate[0],
        hero_1_avg_crit_taken_rate: result.hero_avg_crit_taken_rate[0],

        hero_2_identifier: result
            .hero_names
            .get(1)
            .unwrap_or(&String::from(""))
            .to_string(),
        hero_2_survival_rate: f!("{:.4}", round_to_4(result.hero_survival_rate[1])),
        hero_2_avg_hp_remaining: result.hero_avg_hp_remaining[1],
        hero_2_avg_dmg: result.hero_avg_dmg[1],
        hero_2_avg_dodge_rate: result.hero_avg_dodge_rate[1],
        hero_2_avg_atk_hit_rate: result.hero_avg_atk_hit_rate[1],
        hero_2_avg_crit_dealt_rate: result.hero_avg_crit_dealt_rate[1],
        hero_2_avg_crit_taken_rate: result.hero_avg_crit_taken_rate[1],

        hero_3_identifier: result
            .hero_names
            .get(2)
            .unwrap_or(&String::from(""))
            .to_string(),
        hero_3_survival_rate: f!("{:.4}", round_to_4(result.hero_survival_rate[2])),
        hero_3_avg_hp_remaining: result.hero_avg_hp_remaining[2],
        hero_3_avg_dmg: result.hero_avg_dmg[2],
        hero_3_avg_dodge_rate: result.hero_avg_dodge_rate[2],
        hero_3_avg_atk_hit_rate: result.hero_avg_atk_hit_rate[2],
        hero_3_avg_crit_dealt_rate: result.hero_avg_crit_dealt_rate[2],
        hero_3_avg_crit_taken_rate: result.hero_avg_crit_taken_rate[2],

        hero_4_identifier: result
            .hero_names
            .get(3)
            .unwrap_or(&String::from(""))
            .to_string(),
        hero_4_survival_rate: f!("{:.4}", round_to_4(result.hero_survival_rate[3])),
        hero_4_avg_hp_remaining: result.hero_avg_hp_remaining[3],
        hero_4_avg_dmg: result.hero_avg_dmg[3],
        hero_4_avg_dodge_rate: result.hero_avg_dodge_rate[3],
        hero_4_avg_atk_hit_rate: result.hero_avg_atk_hit_rate[3],
        hero_4_avg_crit_dealt_rate: result.hero_avg_crit_dealt_rate[3],
        hero_4_avg_crit_taken_rate: result.hero_avg_crit_taken_rate[3],

        hero_5_identifier: result
            .hero_names
            .get(4)
            .unwrap_or(&String::from(""))
            .to_string(),
        hero_5_survival_rate: f!("{:.4}", round_to_4(result.hero_survival_rate[4])),
        hero_5_avg_hp_remaining: result.hero_avg_hp_remaining[4],
        hero_5_avg_dmg: result.hero_avg_dmg[4],
        hero_5_avg_dodge_rate: result.hero_avg_dodge_rate[4],
        hero_5_avg_atk_hit_rate: result.hero_avg_atk_hit_rate[4],
        hero_5_avg_crit_dealt_rate: result.hero_avg_crit_dealt_rate[4],
        hero_5_avg_crit_taken_rate: result.hero_avg_crit_taken_rate[4],
    };

    return t_csv_rec;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct PeeteeDuoSkillzTrialResultCSVRecord {
    skill_1: String,     // 3-letter Code, not T1 Name
    skill_2: String,     // 3-letter Code, not T1 Name
    skill_3: String,     // 3-letter Code, not T1 Name
    skill_4: String,     // 3-letter Code, not T1 Name
    atk: String,         // Not used
    def: String,         // Not used
    hp: String,          // Not used
    eva: String,         // Not used
    crit_damage: String, // Not used
    crit_chance: String, // Not used
    attack_mod: String,  // Not used
    def_mod: String,     // Not used
    exp_atk: String,     // Not Used
    quest_success_rate_percent: String,
    avg_rounds: f64,
    target_hero_survival_rate: String,
    target_hero_avg_hp_remaining: f64,
    target_hero_avg_dmg: f64,
    control_hero_survival_rate: String,

    blank_column: String,

    trial_num_minibosses: usize,
    force_minibosses: String,
    difficulty_settings: String,
    dungeon_identifier: String,
    trial_simulation_qty: usize,
    trial_description: String,
    trial_identifier: String,
}

impl PeeteeDuoSkillzTrialResultCSVRecord {
    pub fn round_floats_for_display(&self) -> PeeteeDuoSkillzTrialResultCSVRecord {
        let mut tcr2 = self.clone();

        tcr2.avg_rounds = round_to_4(tcr2.avg_rounds);
        tcr2.target_hero_avg_hp_remaining = round_to_2(tcr2.target_hero_avg_hp_remaining);
        tcr2.target_hero_avg_dmg = round_to_2(tcr2.target_hero_avg_dmg);

        return tcr2;
    }
}

/// Create a trial csv record performing type validation and calculating certain fields
/// ASSUMES THAT THE TARGET HERO IS HERO 1 AND THE CONTROL HERO IS HERO 2
fn create_peetee_duoskillz_trial_result_csv_record_from_trial_result(
    result: TrialResult,
    skill_abbreviation_map: HashMap<String, String>,
) -> PeeteeDuoSkillzTrialResultCSVRecord {
    let mut new_diff_settings: Vec<&str> = Default::default();
    let diff_map: std::collections::HashMap<usize, &str> = std::collections::HashMap::from([
        (1 as usize, "Easy"),
        (2 as usize, "Medium"),
        (3 as usize, "Hard"),
        (4 as usize, "Extreme"),
        (5 as usize, "Boss Easy"),
        (6 as usize, "Boss Medium"),
        (7 as usize, "Boss Hard"),
        (8 as usize, "Boss Extreme"),
    ]);

    for diff in result.difficulty_settings {
        new_diff_settings.push(diff_map[&diff]);
    }

    let new_force_miniboss: String;

    match result.force_minibosses {
        Some(setting) => {
            if setting == true {
                new_force_miniboss = String::from("Force Only Minibosses")
            } else {
                new_force_miniboss = String::from("No Minibosses Allowed")
            }
        }
        None => new_force_miniboss = String::from("Minibosses Allowed with Random Chance"),
    }

    let skills_split: Vec<String> = result
        .trial_description
        .replace(&['[', ']', '"'][..], "")
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let mut skills_abbr: Vec<String> = vec![];
    for skill in skills_split {
        let abbr = skill_abbreviation_map.get(&skill);
        match abbr {
            Some(abbreviation) => skills_abbr.push(abbreviation.to_string()),
            None => skills_abbr.push(skill),
        }
    }

    let t_csv_rec = PeeteeDuoSkillzTrialResultCSVRecord {
        skill_1: skills_abbr[0].to_string(), // 3-letter Code, not T1 Name
        skill_2: skills_abbr[1].to_string(), // 3-letter Code, not T1 Name
        skill_3: skills_abbr[2].to_string(), // 3-letter Code, not T1 Name
        skill_4: skills_abbr[3].to_string(), // 3-letter Code, not T1 Name
        atk: String::from(""),               // Not used
        def: String::from(""),               // Not used
        hp: String::from(""),                // Not used
        eva: String::from(""),               // Not used
        crit_damage: String::from(""),       // Not used
        crit_chance: String::from(""),       // Not used
        attack_mod: String::from(""),        // Not used
        def_mod: String::from(""),           // Not used
        exp_atk: String::from(""),           // Not Used
        quest_success_rate_percent: f!("{:.4}", round_to_4(result.success_rate)),
        avg_rounds: result.average_rounds,
        target_hero_survival_rate: f!("{:.4}", round_to_4(result.hero_survival_rate[0])),
        target_hero_avg_hp_remaining: result.hero_avg_hp_remaining[0],
        target_hero_avg_dmg: result.hero_avg_dmg[0],
        control_hero_survival_rate: f!("{:.4}", round_to_4(result.hero_survival_rate[1])),

        blank_column: String::from(""),

        trial_num_minibosses: result.trial_num_minibosses,
        force_minibosses: new_force_miniboss,
        difficulty_settings: format!("{:?}", new_diff_settings),
        dungeon_identifier: result.dungeon_identifier,
        trial_simulation_qty: result.trial_simulation_qty,
        trial_description: result.trial_description,
        trial_identifier: result.trial_identifier,
    };

    return t_csv_rec;
}
