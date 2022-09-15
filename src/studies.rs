use crate::{dungeons::Dungeon, heroes::Team, trials::Trial};

use itertools::Itertools;

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

/// An extension of Study for generating and ranking Trials for each combination of skills for a single hero on a team
pub struct SingleHeroSkillStudy {
    study: Study,
    base_team: Team,
    subject_hero_identifier: String, // The identifier of the hero to vary upon, and whose performance will be analyzed for the purposes of this study
    valid_skills: Vec<String>,
    skill_variations: Vec<Vec<usize>>, // A collection of vectors containing the indices corresponding to elements in valid_skills
    dungeons: Vec<Dungeon>,
}

pub fn create_single_hero_skill_study(
    identifier: String,
    description: String,
    simulation_qty: i32,
    runoff_scoring_threshold: f64,
    base_team: Team,
    subject_hero_identifier: String,
    valid_skills: Vec<String>,
    dungeons: Vec<Dungeon>,
) -> SingleHeroSkillStudy {
    return SingleHeroSkillStudy {
        study: create_study(
            identifier,
            description,
            simulation_qty,
            runoff_scoring_threshold,
        ),
        base_team,
        subject_hero_identifier,
        skill_variations: Default::default(),
        valid_skills,
        dungeons,
    };
}

impl SingleHeroSkillStudy {
    pub fn count_skill_variations_remaining(&self) -> usize {
        return self.skill_variations.len();
    }

    pub fn populate_skill_variations(&mut self) {
        /* Optimizing Variation Generation
        1. Identify the class of the subject hero and filter out incompatible skills
        2. Identify the build of the subject hero and filter out the mismatched weapon skills
        3. Consider the feasibility of restricting the first two skills to epic/rare and all remaining valid skills for the rest

        Limitations
        1. Must be able to resume from mid-generation by saving combination index, the current step, and the current list
        2. Combination generator is naive to the skills themselves, only works on indices of the current list
        2. a. Because of this, must skip incompatible_with skills while executing trials rather than before generation
        */




        // println!("Starting Population, Len: {}", self.valid_skills.len());
        // let combinations = (0..self.valid_skills.len()).combinations(4);
        // let mut count = 0;
        // // combinations.for_each(|c| {
        // //     count += 1;
        // //     print!("\r{}", count)
        // // });
        // println!("Done");
        println!(
            "Combinations: {}",
            crate::combinations::count_combinations(self.valid_skills.len() as i64, 4)
        );
        println!(
            "C[0]: {:#?}",
            crate::combinations::iter_combination(0, self.valid_skills.len() as i64, 4)
        );
        println!(
            "C[330000000]: {:#?}",
            crate::combinations::iter_combination(330000000, self.valid_skills.len() as i64, 4)
        );
        // self.skill_variations = combinations.collect::<Vec<Vec<usize>>>();
    }
}
