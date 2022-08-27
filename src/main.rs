use serde::{Deserialize, Serialize};
// use std::thread;
// use std::time::Duration;

mod equipment;
use crate::equipment::ElementType;

mod heroes;
use crate::heroes::{create_hero, create_team, HeroArchetype, Team};

mod dungeons;
use crate::dungeons::{create_encounter, Dungeon, MiniBossType};

mod simulation;
use crate::simulation::create_simulation;

/// Defines valid study types:
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
enum StudyType {
    SingleHeroPerformance,
}

/// Defines a plan for generating and ranking Trials
/// A trial is run for each permutation of team/dungeon variation
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Study {
    identifier: String,
    description: String,
    team_variations: Vec<Team>,
    dungeon_variations: Vec<Dungeon>,
    simulation_qty: i32,
    ranking_method: StudyType,
    trials: Vec<Trial>,
}

/// Defines instructions for running one or more Simulations
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Trial {
    identifier: String,
    simulation_qty: i32,
    team: Team,
    dungeon: Dungeon,
    results: Option<Vec<String>>,
}

// // Envisioning you would have 3 cases: trialing just skills with static gear, trialing just gear with static skills, and trialing both
// #[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
// struct HeroTrial {
//     hero: Hero,
//     permute_skills: Boolean,
//     permute_gear: Boolean,
//     permute_both: Boolean,
//     trial_skillset: Vec<String>,
//     trial_gearset: Vec<String>,
// }

// impl HeroTrial {
//     fn get_class(&self) -> Class {
//         self.class
//     }
// }

fn main() {
    // println!("Hello, world!");
    // let hero = HeroTrial {
    //     class: Class::Knight,
    // };

    // if hero.get_class() == Class::Knight {
    //     println!("Knight");
    //     println!("{:?}", hero.get_class());
    // }

    let hero = create_hero(
        "HeroID".to_string(),
        "Thief".to_string(),
        HeroArchetype::GreenRogue,
        1u8,
        0u8,
        10u32,
        10u32,
        10u32,
        10u16,
        10u16,
        1.0f64,
        10u16,
        10u16,
        ElementType::Fire,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        10u16,
        10u16,
    )
    .unwrap();

    let team = create_team(vec![hero], None).unwrap();

    let encounter = create_encounter(
        "Custom".to_string(),
        10u32,
        10u32,
        10u32,
        10u16,
        10u16,
        false,
        false,
        Some(MiniBossType::Legendary),
        None,
        0u32,
        1u8,
    )
    .unwrap();

    let mut simulation = create_simulation(team, encounter, vec![]).unwrap();

    let sim_res = simulation.run().unwrap();

    println!("Complete {:#?}", sim_res.is_success());
}
