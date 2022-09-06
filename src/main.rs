use std::collections::HashMap;

use equipment::Blueprint;
use hero_builder::HeroClass;
use serde::{Deserialize, Serialize};
// use std::thread;
// use std::time::Duration;
use log::info;

#[macro_use]
extern crate fstrings;

mod equipment;

mod heroes;
use crate::hero_builder::_create_hero_class;
use crate::heroes::{create_team, SimHero, Team};

mod dungeons;
use crate::dungeons::Dungeon;

mod simulations;

mod trials;
use crate::sheet_processing::_get_hero_equipment_data;
use crate::trials::{create_trial, Trial};

mod inputs;
use crate::inputs::{
    _save_hero_classes_to_yaml, load_dungeons_from_yaml, load_hero_classes_from_yaml,
    load_heroes_as_sim_heroes_from_csv, load_sim_heroes_from_csv,
};

mod decimals;

mod hero_builder;

mod sheet_processing;

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

fn load_sim_heroes(
    bp_map: HashMap<String, Blueprint>,
    hero_classes: HashMap<String, HeroClass>,
) -> HashMap<String, SimHero> {
    let heroes_from_builder = load_heroes_as_sim_heroes_from_csv(
        String::from("input/hero_builder.csv"),
        bp_map,
        hero_classes,
    );
    // _save_sim_heroes_to_csv(String::from("input/heroes.csv"), loaded_heroes_from_builder).unwrap();

    let mut loaded_heroes = load_sim_heroes_from_csv(String::from("input/heroes.csv"))
        .iter()
        .map(|hero| (hero._get_identifier(), hero.clone()))
        .collect::<HashMap<String, SimHero>>();
    loaded_heroes.extend(heroes_from_builder);
    return loaded_heroes;
}

fn main() {
    let mut i = 0;
    while std::path::Path::new(&f!("target/logs/trial_{}.log", i)).exists() {
        // Create new log file each run
        i += 1;
    }
    fast_log::init(fast_log::Config::new().file(&f!("target/logs/trial_{}.log", i))).unwrap();
    info!("Start of Log File");

    let hc_hm = HashMap::from([(
        String::from("Knight"),
        _create_hero_class(
            String::from("Knight"),
            String::from("Merchant Lv. 22"),
            50000,
            150,
            vec![
                42.0, 46.0, 50.0, 53.0, 57.0, 61.0, 65.0, 68.0, 72.0, 80.0, 87.0, 95.0, 102.0,
                110.0, 118.0, 125.0, 133.0, 140.0, 148.0, 155.0, 167.0, 178.0, 189.0, 201.0, 212.0,
                223.0, 235.0, 246.0, 257.0, 269.0, 284.0, 299.0, 314.0, 329.0, 344.0, 360.0, 374.0,
                390.0, 405.0, 420.0,
            ],
            vec![
                10.0, 13.0, 17.0, 20.0, 24.0, 27.0, 30.0, 34.0, 37.0, 44.0, 51.0, 58.0, 64.0, 71.0,
                78.0, 85.0, 92.0, 98.0, 105.0, 112.0, 122.0, 132.0, 143.0, 153.0, 163.0, 173.0,
                183.0, 194.0, 204.0, 214.0, 228.0, 241.0, 255.0, 268.0, 282.0, 296.0, 309.0, 323.0,
                336.0, 350.0,
            ],
            vec![
                25.0, 30.0, 34.0, 39.0, 44.0, 49.0, 53.0, 58.0, 63.0, 72.0, 82.0, 91.0, 101.0,
                110.0, 120.0, 129.0, 139.0, 148.0, 158.0, 167.0, 182.0, 196.0, 210.0, 224.0, 239.0,
                253.0, 267.0, 281.0, 296.0, 310.0, 329.0, 348.0, 367.0, 386.0, 405.0, 424.0, 443.0,
                462.0, 481.0, 500.0,
            ],
            0.0,
            0.05,
            2.0,
            90,
            String::from("Light"),
            [
                vec![String::from("Spear"), String::from("Axe")],
                vec![String::from("Heavy Armor")],
                vec![String::from("Gauntlets")],
                vec![String::from("Helmet")],
                vec![String::from("Heavy Footwear")],
                vec![String::from("Shield")],
            ],
            [
                String::from("Fortress"),
                String::from("Citadel"),
                String::from("Bastion"),
                String::from(""),
            ],
        ),
    )]);

    _save_hero_classes_to_yaml(String::from("input/hero_classes.yaml"), hc_hm).unwrap();

    let hero_classes = load_hero_classes_from_yaml(String::from("input/hero_classes.yaml"));

    // let new_hero = create_hero(
    //     String::from("Tammy"),
    //     String::from("Arch Druid"),
    //     35,
    //     6,
    //     40.0,
    //     90.0,
    //     35.0,
    //     0.0,
    //     0.05,
    //     2.0,
    //     10,
    //     String::from("Earth"),
    //     40,
    //     0,
    //     0,
    //     [
    //         String::from("Primal Magic"),
    //         String::from("Shining Blade"),
    //         String::from("Mana Shield"),
    //         String::from("Electric Arc"),
    //         String::from("Deadly Criticals"),
    //     ],
    //     [
    //         String::from("Evergreen Wand"),
    //         String::from("Astravestimenta"),
    //         String::from("Yggdrasil Branch"),
    //         String::from("Luckiest Clover"),
    //         String::from("Ursa Totem"),
    //         String::from("Grimoire Aeternum"),
    //     ],
    //     [
    //         String::from("Superior"),
    //         String::from("Superior"),
    //         String::from("Normal"),
    //         String::from("Epic"),
    //         String::from("Epic"),
    //         String::from("Flawless"),
    //     ],
    //     [
    //         String::from("Primal"),
    //         String::from("Primal"),
    //         String::from("Primal"),
    //         String::from("Primal"),
    //         String::from("Primal"),
    //         String::from("Primal"),
    //     ],
    //     [
    //         String::from("Bear"),
    //         String::from("Walrus"),
    //         String::from("Walrus"),
    //         String::from("Shark"),
    //         String::from("Bear"),
    //         String::from("Eagle"),
    //     ],
    // );

    // _save_heroes_to_csv(
    //     String::from("input/hero_builder.csv"),
    //     HashMap::from([(String::from("Tammy"), new_hero)]),
    // )
    // .unwrap();

    let bp_map = _get_hero_equipment_data(String::from(
        "data_sheets/blueprints_v_10.2.1_slash_1.0.1.773.tsv",
    ));
    let heroes = load_sim_heroes(bp_map, hero_classes);

    let team = create_team(vec![heroes["Tammy"].clone()], None).unwrap();

    let dungeons = load_dungeons_from_yaml(String::from("input/dungeons.yaml"));
    let dungeon = dungeons["Bleakspire Peak"].clone();

    // Difficulty settings (include all that should apply):
    // 1 - Easy, 2 - Medium, 3 - Hard, 4 - Extreme,
    // 5 - Boss Easy, 6 - Boss Medium, 7 - Boss Hard, 8 - Boss Extreme

    let mut trial = create_trial(
        "debugging".to_string(),
        100,
        team,
        dungeon,
        vec![6],
        Some(false),
        true,
    )
    .unwrap();

    trial.run_simulations_single_threaded();

    let trial_csv_path = f!("target/csvs/trial_{}.csv", i);
    if let Some(p) = std::path::Path::new(&trial_csv_path).parent() {
        std::fs::create_dir_all(p).unwrap();
    }
    trial.save_results_to_csv(trial_csv_path).unwrap();

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
    info!(
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
}
