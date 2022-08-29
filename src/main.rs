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
        "Tammara".to_string(),
        "Arch Druid".to_string(),
        HeroArchetype::BlueSpellcaster,
        35,
        0,
        3,
        628.0,
        17485,
        2869,
        10,
        7,
        340.0,
        0,
        100,
        ElementType::Earth,
        0,
        0,
        1,
        0,
        0,
        330,
        80,
    )
    .unwrap();

    let champion = create_hero(
        "Champion".to_string(),
        "Argon".to_string(),
        HeroArchetype::Champion,
        36,
        10,
        2,
        769.0,
        2845,
        4708,
        90,
        5,
        2.0,
        0,
        80,
        ElementType::Light,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    )
    .unwrap();

    let team = create_team(vec![hero, champion], None).unwrap();

    let dungeon = create_dungeon(
        "Sun God's Tomb".to_string(),
        4,
        [25000.0, 40000.0, 80000.0, 430000.0],
        [40.0, 50.0, 60.0, 130.0],
        [12000.0, 14400.0, 16200.0, 42000.0],
        [20.0, 25.0, 30.0, 65.0],
        [25, 25, 25, 25],
        [1200, 1500, 2500, 7500],
        [ElementType::Dark, ElementType::Water, ElementType::Air],
        250.0,
        [60000.0, 90000.0, 130000.0, 750000.0],
        [60.0, 70.0, 80.0, 175.0],
        [16800.0, 20400.0, 22800.0, 47000.0],
        [30.0, 35.0, 40.0, 80.0],
        [25, 25, 25, 25],
        [1800, 2500, 4000, 10000],
        ElementType::Light,
        350.0,
    )
    .unwrap();

    let mut trial =
        create_trial("".to_string(), 50000, team, dungeon, vec![3, 4, 7, 8], None).unwrap();

    trial.run_simulations_single_threaded();

    let res = trial.get_results_unranked();

    let mut successes = 0;
    for sr in &res {
        if sr.is_success() {
            successes += 1;
        }
    }

    println!(
        "Completed. {:#?} successes of {:#?} simulations",
        successes,
        res.len()
    );
    // println!("Example: {:#?} {:#?}", res[0].is_success(), res[0])
}
