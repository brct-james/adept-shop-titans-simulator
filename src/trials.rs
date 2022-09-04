use crate::decimals::round_to_2;

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
    pub fn round_floats_for_display(&self) -> TrialCSVRecord {
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
fn create_trial_csv_record(
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
            print!("Running simulation iteration:  # {:#?}", self.results.len());
            info!(
                "\n\nRunning simulation iteration: # {}\n",
                self.results.len()
            );
            let encounter = self
                .dungeon
                .generate_encounter_from_dungeon(&self.difficulty_settings, self.force_minibosses)
                .unwrap();
            let mut simulation =
                create_simulation(&self.team, encounter, vec![], self.log_all).unwrap();
            let sim_res = simulation.run().unwrap();
            print!(
                "\rRunning simulation iteration: # {:#?} | Success: {:#?} in {:#?} rounds\n",
                self.results.len(),
                sim_res.is_success(),
                sim_res.get_rounds(),
            );
            self.results.push(sim_res);
        }
    }
    pub fn get_results_unranked(&self) -> Vec<SimResult> {
        return self.results.clone();
    }
    pub fn save_results_to_csv(&self, path: String) -> Result<(), std::io::Error> {
        let mut wtr = csv::Writer::from_path(path)?;

        for (i, res) in self.results.iter().enumerate() {
            let record = create_trial_csv_record(
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
            wtr.serialize(record.round_floats_for_display())?;
        }

        wtr.flush()?;
        return Ok(());
    }
}

/// Create a trial performing type validation and calculating certain fields
pub fn create_trial(
    identifier: String,
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
