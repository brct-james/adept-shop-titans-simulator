pub mod single_hero_skill_study;

use crate::{trials::Trial};

use serde::{Deserialize, Serialize};

extern crate csv;

/// Defines a plan for generating and ranking Trials
/// A trial is run for each permutation of team/dungeon variation
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Study {
    identifier: String,
    description: String,
    simulation_qty: i32,
    runoff_scoring_threshold: f64, // The top X% of the results will be re-tested on the n+1 dungeon in the dungeons vec until either there are no successes or the vec is exhausted. Pass 100.0 to disable runoff scoring
    trials: Vec<Trial>,
}

pub fn create_study(
    identifier: String,
    description: String,
    simulation_qty: i32,
    runoff_scoring_threshold: f64,
) -> Study {
    return Study {
        identifier,
        description,
        simulation_qty,
        runoff_scoring_threshold,
        trials: Default::default(),
    };
}

