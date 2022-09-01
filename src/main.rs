use serde::{Deserialize, Serialize};
// use std::thread;
// use std::time::Duration;
use log::info;

#[macro_use]
extern crate fstrings;

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
    let mut i = 0;
    while std::path::Path::new(&f!("target/logs/simulation_{}.log", i)).exists() {
        // Create new log file each run
        i += 1;
    }
    fast_log::init(fast_log::Config::new().file(&f!("target/logs/simulation_{}.log", i))).unwrap();
    info!("Start of Log File");
    // println!("Hello, world!");
    // let hero = HeroTrial {
    //     class: Class::Knight,
    // };

    // if hero.get_class() == Class::Knight {
    //     println!("Knight");
    //     println!("{:?}", hero.get_class());
    // }

    let _tammara = create_hero(
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

    let _argon = create_hero(
        "argon".to_string(),
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

    let _dormammu = create_hero(
        "Dormammu".to_string(),
        "Berserker".to_string(),
        HeroArchetype::RedFighter,
        23,
        0,
        2,
        470.0,
        1849,
        1658,
        90,
        5,
        2.0,
        8,
        60,
        ElementType::Fire,
        0,
        0,
        0,
        0,
        0,
        95,
        80,
    )
    .unwrap();

    let team = create_team(vec![_dormammu], None).unwrap();

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

    // Difficulty settings (include all that should apply):
    // 1 - Easy, 2 - Medium, 3 - Hard, 4 - Extreme,
    // 5 - Boss Easy, 6 - Boss Medium, 7 - Boss Hard, 8 - Boss Extreme

    let mut trial =
        create_trial("".to_string(), 5000, team, dungeon, vec![1], Some(false)).unwrap();

    trial.run_simulations_single_threaded();

    let res = trial.get_results_unranked();

    let mut successes = 0;
    let mut min_rounds = i16::max_value();
    let mut avg_rounds = 0.0;
    let mut max_rounds = i16::min_value();
    let mut dmg_dealt: [Vec<f64>; 5] = Default::default();
    let mut hp_remaining: Vec<f64> = vec![];
    for sr in &res {
        if sr.is_success() {
            successes += 1;
        }
        min_rounds = std::cmp::min(min_rounds, sr.get_rounds());
        avg_rounds += sr.get_rounds() as f64;
        max_rounds = std::cmp::max(max_rounds, sr.get_rounds());
        let dmg_fight = sr.get_damage_dealt_during_fight();
        for (i, hero_damage) in dmg_fight.iter().enumerate() {
            dmg_dealt[i].push(*hero_damage);
        }
        hp_remaining.push(sr.get_encounter_hp_remaining());
    }

    avg_rounds = avg_rounds / res.len() as f64;
    let avg_dmg_dealt_0 = dmg_dealt[0].iter().sum::<f64>() / dmg_dealt[0].len() as f64;
    let avg_encounter_hp_remaining = hp_remaining.iter().sum::<f64>() / hp_remaining.len() as f64;

    println!(
        "Completed. {:#?} successes of {:#?} simulations. Success Rate: {:.2}%. Rounds Min/Avg/Max: {:#?}/{:.2}/{:#?}. Avg Dmg Dealt By Hero 0: {:.2} leaving avg remaining of {:.2}",
        successes,
        res.len(),
        successes as f64 / res.len() as f64 * 100.0,
        min_rounds,
        avg_rounds,
        max_rounds,
        avg_dmg_dealt_0,
        avg_encounter_hp_remaining,
    );

    res[0].print_team();
    res[0].print_encounter();
    // println!("Example: {:#?} {:#?}", res[0].is_success(), res[0])
}
