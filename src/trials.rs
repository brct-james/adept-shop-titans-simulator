use super::dungeons::Dungeon;
use super::heroes::Team;
use super::simulations::{create_simulation, SimResult};

use serde::{Deserialize, Serialize};

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
}

impl Trial {
    pub fn run_simulations_single_threaded(&mut self) {
        while self.results.len() < self.simulation_qty {
            print!("Running simulation iteration:  # {:#?}", self.results.len());
            let encounter = self
                .dungeon
                .generate_encounter_from_dungeon(&self.difficulty_settings, self.force_minibosses)
                .unwrap();
            let mut simulation = create_simulation(&self.team, encounter, vec![]).unwrap();
            let sim_res = simulation.run().unwrap();
            print!(
                "\rRunning simulation iteration: # {:#?} | Success: {:#?}\n",
                self.results.len(),
                sim_res.is_success()
            );
            self.results.push(sim_res);
        }
    }
    pub fn get_results_unranked(&self) -> Vec<SimResult> {
        return self.results.clone();
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
    };

    return Ok(trial);
}
