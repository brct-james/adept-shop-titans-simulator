use serde::{Deserialize, Serialize};
// use std::thread;
// use std::time::Duration;

mod equipment;
use crate::equipment::ElementType;

mod heroes;
use crate::heroes::{create_hero, create_team, HeroArchetype, Team};

mod dungeons;
use crate::dungeons::{create_dungeon, Dungeon};

mod simulations;

mod trials;
use crate::trials::{create_trial, Trial};

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

    let dungeon = create_dungeon(
        "Sun God's Tomb".to_string(),
        4,
        [25000, 40000, 80000, 430000],
        [40, 50, 60, 130],
        [12000, 14400, 16200, 42000],
        [20, 25, 30, 65],
        [25, 25, 25, 25],
        [1200, 1500, 2500, 7500],
        [ElementType::Dark, ElementType::Water, ElementType::Air],
        250,
        [60000, 90000, 130000, 750000],
        [60, 70, 80, 175],
        [16800, 20400, 22800, 47000],
        [30, 35, 40, 80],
        [25, 25, 25, 25],
        [1800, 2500, 4000, 10000],
        ElementType::Light,
        350,
    )
    .unwrap();

    let mut trial = create_trial(
        "".to_string(),
        100000,
        team,
        dungeon,
        vec![3, 4, 7, 8],
        None,
    )
    .unwrap();

    trial.run_simulations_single_threaded();

    let res = trial.get_results_unranked();

    println!("Completed. # of results: {:#?}", res.len());
    // println!("Example: {:#?} {:#?}", res[0].is_success(), res[0])
}
