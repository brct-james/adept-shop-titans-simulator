use super::dungeons::Encounter;
use super::heroes::Team;

use serde::{Deserialize, Serialize};

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
    team: Team,
    encounter: Encounter,
    metrics: Vec<String>,
) -> Result<Simulation, &'static str> {
    let simulation = Simulation {
        team,
        encounter,
        metrics,
    };

    return Ok(simulation);
}

/// The result of a simulation
pub struct SimResult {
    success: bool,
    rounds_elapsed: i16,
    team_dmg_taken: Vec<i16>,
    team_dmg_dealt: Vec<i16>,
    team_dmg_dodged: Vec<i16>,
    team_bonus_loot_qty: i8,
    team_rest_times: Vec<i32>,
}

impl SimResult {
    pub fn is_success(&self) -> bool {
        return self.success;
    }
}
