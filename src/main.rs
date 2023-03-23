// use std::thread;
// use std::time::Duration;

#[macro_use]
extern crate fstrings;

mod equipment;

mod heroes;

mod dungeons;

mod simulations;

mod trials;
use log::info;

use crate::sheet_processing::{
    _get_hero_equipment_data, _get_hero_skills_data, _get_innate_skills_data,
};
use crate::studies::HeroBuilderInformation;

mod inputs;
use crate::inputs::{
    load_dungeons_from_yaml, load_hero_classes_from_yaml, load_heroes_as_sim_heroes_from_csv,
    load_heroes_from_csv, load_skill_abbreviation_map, load_study_docket,
};

mod decimals;

mod skills;

mod hero_builder;

mod sheet_processing;

mod studies;

mod combinations;

mod dockets;

fn main() {
    // Create new log file each run
    let mut trial_logs_path: String;
    let mut i = 0;
    loop {
        trial_logs_path = f!("target/logs/log_{}.log", i);
        // Increment until file not exist, then create and break
        if std::path::Path::new(&trial_logs_path).exists() {
            i += 1;
            continue;
        }
        break;
    }
    fast_log::init(fast_log::Config::new().file(&trial_logs_path)).unwrap();
    info!("Start of Log File");

    // let study_identifier = String::from("Daimyo_Atk_Main_Single");

    let hero_classes = load_hero_classes_from_yaml(String::from("input/hero_classes.yaml"));

    let (hero_skill_tier_1_name_map, hero_skill_any_tier_to_tier_1_name_map, hero_skill_map) =
        _get_hero_skills_data(String::from(
            "data_sheets/greensim_hero_skills_v_10.2.1_slash_1.0.1.773.tsv",
        ));

    let (
        _innate_skill_tier_1_name_map,
        innate_skill_any_tier_to_tier_1_name_nap,
        class_innate_skill_names_map,
        innate_skill_map,
    ) = _get_innate_skills_data(String::from(
        "data_sheets/greensim_innate_skills_v_10.2.1_slash_1.0.1.773.tsv",
    ));

    let bp_map = _get_hero_equipment_data(String::from(
        "data_sheets/blueprints_v_11.1.1_slash_1.0.1.868.tsv",
    ));
    let loaded_heroes = load_heroes_as_sim_heroes_from_csv(
        String::from("input/hero_builder.csv"),
        bp_map.clone(),
        hero_classes.clone(),
        hero_skill_tier_1_name_map.clone(),
        hero_skill_map.clone(),
        class_innate_skill_names_map.clone(),
        innate_skill_map.clone(),
    );

    let loaded_dungeons = load_dungeons_from_yaml(String::from("input/dungeons.yaml"));

    let (hero_skill_abbreviation_map, hero_abbreviation_skill_map) =
        load_skill_abbreviation_map(String::from("data_sheets/skill_abbreviation_map.csv"));

    let loaded_heroes_from_builder = load_heroes_from_csv(
        String::from("input/hero_builder.csv"),
        bp_map.clone(),
        hero_classes.clone(),
    );

    let mut loaded_valid_skills: Vec<String> = Default::default();
    for (k, v) in &hero_skill_tier_1_name_map {
        let ksplit: Vec<&str> = k.split(' ').collect();
        if ksplit[ksplit.len() - 1] == "T4" {
            loaded_valid_skills.push(v.to_string());
        }
    }

    let loaded_hero_builder_information = HeroBuilderInformation {
        bp_map,
        hero_classes,
        hero_skill_tier_1_name_map,
        hero_skill_any_tier_to_tier_1_name_map,
        hero_skill_abbreviation_map,
        hero_abbreviation_skill_map,
        hero_skill_map,
        class_innate_skill_names_map,
        innate_skill_any_tier_to_tier_1_name_nap,
        innate_skill_map,
    };

    /* STUDIES */
    println!("Loading Docket");
    let mut docket = load_study_docket(&String::from("input/study_docket.csv"));
    if docket.get_num_studies() == 0 {
        println!(
            "Docket Loaded with {} Studies: Program Closing",
            docket.get_num_studies()
        );
        return;
    }
    println!("Docket Loaded with {} Studies", docket.get_num_studies());
    docket.commence(
        loaded_heroes,
        loaded_dungeons,
        loaded_valid_skills,
        loaded_heroes_from_builder,
        loaded_hero_builder_information,
    );
    println!("Program Closing");
}
