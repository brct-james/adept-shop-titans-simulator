use std::collections::HashMap;

use std::time::Instant;

use equipment::Blueprint;
use hero_builder::HeroClass;
// use std::thread;
// use std::time::Duration;
use log::info;
use skills::{HeroSkill, InnateSkill};

#[macro_use]
extern crate fstrings;

mod equipment;

mod heroes;
use crate::hero_builder::_create_hero_class;
use crate::heroes::{create_team, SimHero};

mod dungeons;

mod simulations;

mod trials;
use crate::sheet_processing::{
    _get_hero_equipment_data, _get_hero_skills_data, _get_innate_skills_data,
};
use crate::trials::create_trial;

mod inputs;
use crate::inputs::{
    _save_hero_classes_to_yaml, load_dungeons_from_yaml, load_hero_classes_from_yaml,
    load_heroes_as_sim_heroes_from_csv, load_sim_heroes_from_csv,
};

mod decimals;

mod skills;

mod hero_builder;

mod sheet_processing;

mod studies;
use studies::single_hero_skill_study::create_single_hero_skill_study;

mod combinations;

fn load_sim_heroes(
    bp_map: HashMap<String, Blueprint>,
    hero_classes: HashMap<String, HeroClass>,
    hero_skill_tier_1_name_map: HashMap<String, String>,
    hero_skill_map: HashMap<String, HeroSkill>,
    class_innate_skill_names_map: HashMap<String, String>,
    innate_skill_map: HashMap<String, InnateSkill>,
) -> HashMap<String, SimHero> {
    let heroes_from_builder = load_heroes_as_sim_heroes_from_csv(
        String::from("input/hero_builder.csv"),
        bp_map,
        hero_classes,
        hero_skill_tier_1_name_map,
        hero_skill_map,
        class_innate_skill_names_map,
        innate_skill_map,
    );
    // let heroes_loaded_from_builder = heroes_from_builder
    //     .values()
    //     .map(|v| v.clone())
    //     .collect::<Vec<SimHero>>();
    // inputs::_save_sim_heroes_to_csv(String::from("input/heroes.csv"), heroes_loaded_from_builder)
    //     .unwrap();

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
        String::from("Jarl"),
        _create_hero_class(
            String::from("Jarl"),
            String::from("Titan Soul (Berserker)"),
            0,
            0,
            vec![
                100.0, 105.0, 110.0, 115.0, 120.0, 125.0, 130.0, 135.0, 140.0, 150.0, 160.0, 170.0,
                180.0, 190.0, 200.0, 210.0, 220.0, 230.0, 240.0, 250.0, 265.0, 280.0, 295.0, 310.0,
                325.0, 340.0, 355.0, 370.0, 385.0, 400.0, 420.0, 440.0, 460.0, 480.0, 500.0, 520.0,
                540.0, 560.0, 580.0, 600.0,
            ],
            vec![
                75.0, 81.0, 87.0, 94.0, 100.0, 106.0, 112.0, 119.0, 125.0, 137.0, 150.0, 162.0,
                175.0, 187.0, 200.0, 212.0, 225.0, 237.0, 250.0, 262.0, 281.0, 300.0, 319.0, 337.0,
                356.0, 375.0, 394.0, 412.0, 431.0, 450.0, 475.0, 500.0, 525.0, 550.0, 575.0, 600.0,
                625.0, 650.0, 675.0, 700.0,
            ],
            vec![
                90.0, 95.0, 100.0, 105.0, 110.0, 115.0, 121.0, 126.0, 131.0, 141.0, 151.0, 161.0,
                172.0, 182.0, 192.0, 202.0, 212.0, 223.0, 233.0, 243.0, 258.0, 274.0, 289.0, 304.0,
                319.0, 335.0, 350.0, 365.0, 381.0, 396.0, 416.0, 437.0, 457.0, 478.0, 498.0, 518.0,
                539.0, 559.0, 580.0, 600.0,
            ],
            0.0,
            0.05,
            2.0,
            90,
            String::from("Fire"),
            [
                vec![
                    String::from("Mace"),
                    String::from("Axe"),
                    String::from("Gun"),
                ],
                vec![String::from("Heavy Armor")],
                vec![String::from("Gauntlets"), String::from("Helmet")],
                vec![String::from("Heavy Footwear")],
                vec![String::from("Shield"), String::from("Cloak")],
                vec![String::from("Herbal Medicine"), String::from("Potion")],
            ],
            [
                String::from("Berserk Rage"),
                String::from("Anger Point"),
                String::from("The Beast Within"),
                String::from("The Beast Unleashed"),
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

    let (hero_skill_tier_1_name_map, hero_skill_map) = _get_hero_skills_data(String::from(
        "data_sheets/greensim_hero_skills_v_10.2.1_slash_1.0.1.773.tsv",
    ));

    let (_innate_skill_tier_1_name_map, class_innate_skill_names_map, innate_skill_map) =
        _get_innate_skills_data(String::from(
            "data_sheets/greensim_innate_skills_v_10.2.1_slash_1.0.1.773.tsv",
        ));

    let bp_map = _get_hero_equipment_data(String::from(
        "data_sheets/blueprints_v_10.2.1_slash_1.0.1.773.tsv",
    ));
    let heroes = load_sim_heroes(
        bp_map,
        hero_classes,
        hero_skill_tier_1_name_map.clone(),
        hero_skill_map,
        class_innate_skill_names_map,
        innate_skill_map,
    );

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

    let timer = Instant::now();
    trial.run_simulations_single_threaded();
    let timer_duration = timer.elapsed().as_nanos() as f32 / 1000000.0f32;

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
        "Completed in {:#?}ms. {:#?} successes of {:#?} simulations. Success Rate: {:.2}%. Rounds Min/Avg/Max: {:#?}/{:.2}/{:#?}. Avg Dmg Dealt By Hero 0: {:.2} leaving avg remaining of {:.2}",
        timer_duration,
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
        "Completed in {:#?}ms. {:#?} successes of {:#?} simulations. Success Rate: {:.2}%. Rounds Min/Avg/Max: {:#?}/{:.2}/{:#?}. Avg Dmg Dealt By Hero 0: {:.2} leaving avg remaining of {:.2}",
        timer_duration,
        successes,
        res.len(),
        successes as f64 / res.len() as f64 * 100.0,
        min_rounds,
        avg_rounds,
        max_rounds,
        avg_dmg_dealt_0,
        avg_encounter_hp_remaining,
    );

    // println!("{:#?}", _innate_skill_map);
    // println!("{:#?}", _innate_skill_tier_1_name_map);

    let mut valid_skills: Vec<String> = Default::default();
    for (k, v) in hero_skill_tier_1_name_map {
        let ksplit: Vec<&str> = k.split(' ').collect();
        if ksplit[ksplit.len() - 1] == "T4" {
            valid_skills.push(v.to_string());
        }
    }

    let mut study = create_single_hero_skill_study(
        String::from("OptimizeLord"),
        String::from("Optimize lord class"),
        100,
        100.0,
        create_team(vec![heroes["Akana"].clone()], None).unwrap(),
        valid_skills,
        vec!["Marksman".into()],
        String::from("Akana"),
        vec![dungeons["Bleakspire Peak"].clone()],
    );
    println!(
        "Skill Variations Remaining to Test: {}",
        study.count_skill_variations_remaining()
    );
    println!(
        "Skillset at 100: {:#?}",
        study
            .translate_skillset_from_indices(study.get_skillset_at_specific_combination_index(100))
    );
    study.increment_combination_index();
    study.count_skill_variations_completed();
    study.count_skill_variations_total();
    println!(
        "Skillset at current: {:#?}",
        study.get_full_translated_skillset_at_current_combination_index()
    );
}
