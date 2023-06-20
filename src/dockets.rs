use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::info;
use serde::{Deserialize, Serialize};

use rayon::prelude::*;

use crate::dungeons::create_trial_dungeon;
use crate::dungeons::{Dungeon, TrialDungeon};
use crate::equipment::BoosterType;
use crate::hero_builder::Hero;
use crate::heroes::{create_team, Team};
use crate::inputs::save_study_docket;
use crate::simdata::SimData;
use crate::studies::{HeroBuilderInformation, Runnable};
use crate::{heroes::SimHero, studies::static_duo_skill_study::create_static_duo_skill_study};

/// Holds info for generating a study, defines format for deserialization from CSV
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct DocketStudy {
    #[serde(rename(serialize = "Completed", deserialize = "Completed"))]
    completed: bool,
    #[serde(rename(serialize = "Identifier", deserialize = "Identifier"))]
    identifier: String,
    #[serde(rename(serialize = "Description", deserialize = "Description"))]
    description: String,
    #[serde(rename(serialize = "Type", deserialize = "Type"))]
    type_: DocketStudyType,
    #[serde(rename(serialize = "Skill Name Format", deserialize = "Skill Name Format"))]
    skill_name_format: DocketStudySkillNameFormat,
    #[serde(rename(serialize = "Simulation Qty", deserialize = "Simulation Qty"))]
    sim_qty: i32,
    #[serde(rename(
        serialize = "Runoff Scoring Threshold",
        deserialize = "Runoff Scoring Threshold"
    ))]
    runoff_scoring_threshold: f64,
    #[serde(rename(
        serialize = "Team Hero Identifiers",
        deserialize = "Team Hero Identifiers"
    ))]
    team_hero_identifiers: String,
    #[serde(rename(serialize = "Team Booster", deserialize = "Team Booster"))]
    team_booster: String,
    #[serde(rename(
        serialize = "Static Preset Skills",
        deserialize = "Static Preset Skills"
    ))]
    preset_skills: String,
    #[serde(rename(
        serialize = "Dungeon Specifications",
        deserialize = "Dungeon Specifications"
    ))]
    dungeon_specifications: String,
    #[serde(rename(
        serialize = "Automatic Rank Difficulty Optimization",
        deserialize = "Automatic Rank Difficulty Optimization"
    ))]
    automatic_rank_difficulty_optimization: bool,
    #[serde(rename(serialize = "Excluded Skills", deserialize = "Excluded Skills"))]
    excluded_skills: String,
}

impl DocketStudy {
    pub fn is_valid(&self, result_index: usize) -> bool {
        if self.identifier.len() == 0 {
            info!("\tSkipping Record {}: Identifier Is Required", result_index);
            return false;
        }
        if self.sim_qty <= 0 || self.sim_qty > 50000 {
            info!(
                "\tSkipping Record {}: Sim Qty Must Be In Range [1,50000]",
                result_index
            );
            return false;
        }
        if self.runoff_scoring_threshold <= 0.0 || self.runoff_scoring_threshold > 100.0 {
            info!(
                "\tSkipping Record {}: Sim Qty Must Be In Range [1,100]",
                result_index
            );
            return false;
        }
        if self.team_hero_identifiers.len() == 0 {
            info!(
                "\tSkipping Record {}: Team Hero Identifiers Are Required",
                result_index
            );
            return false;
        }
        if self.team_booster.len() == 0 {
            info!(
                "\tSkipping Record {}: Team Booster Is Required",
                result_index
            );
            return false;
        }
        // Preset Skills are NOT required, can be empty to vary all skill slots
        if self.dungeon_specifications.len() == 0 {
            info!(
                "\tSkipping Record {}: Dungeon Specifications Are Required",
                result_index
            );
            return false;
        }
        // Excluded Skills are NOT required, can be empty to exclude no skills

        return true;
    }
}

/// Defines available study types for DocketStudy
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    strum::Display,
    strum::EnumString,
    Default,
)]
pub enum DocketStudyType {
    #[strum(serialize = "StaticDuoSkillStudy")]
    #[default]
    StaticDuoSkillStudy,
}

/// Defines valid skill name types for DocketStudy
/// All skill names in this DocketStudy setting must conform to this format
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    strum::Display,
    strum::EnumString,
    Default,
)]
pub enum DocketStudySkillNameFormat {
    #[strum(serialize = "Abbreviated")]
    Abbreviated,

    #[strum(serialize = "FullTierOne")]
    FullTierOne,

    #[strum(serialize = "FullAnyTier")]
    #[default]
    FullAnyTier,
}

pub fn commence_from_gui(
    docket: &mut Docket,
    sim_data: &mut SimData,
    tx: Sender<(String, u32, u32)>,
) {
    let loaded_hero_builder_information = HeroBuilderInformation {
        bp_map: sim_data.bp_map.clone(),
        hero_classes: sim_data.hero_classes.clone(),
        hero_skill_tier_1_name_map: sim_data.hero_skill_tier_1_name_map.clone(),
        hero_skill_any_tier_to_tier_1_name_map: sim_data
            .hero_skill_any_tier_to_tier_1_name_map
            .clone(),
        hero_skill_abbreviation_map: sim_data.hero_skill_abbreviation_map.clone(),
        hero_abbreviation_skill_map: sim_data.hero_abbreviation_skill_map.clone(),
        hero_skill_map: sim_data.hero_skill_map.clone(),
        class_innate_skill_names_map: sim_data.class_innate_skill_names_map.clone(),
        innate_skill_any_tier_to_tier_1_name_nap: sim_data
            .innate_skill_any_tier_to_tier_1_name_nap
            .clone(),
        innate_skill_map: sim_data.innate_skill_map.clone(),
    };
    docket.commence(
        sim_data.loaded_heroes.clone(),
        sim_data.loaded_dungeons.clone(),
        sim_data.loaded_valid_skills.clone(),
        sim_data.loaded_heroes_from_builder.clone(),
        loaded_hero_builder_information,
        tx,
    );
}

/// Defines a plan for generating and running Studies
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Docket {
    studies: Vec<DocketStudy>,
    path: String,
}

impl Docket {
    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }
    pub fn add_study(&mut self, study: DocketStudy) {
        self.studies.push(study);
    }

    pub fn get_studies(&self) -> Vec<DocketStudy> {
        return self.studies.clone();
    }

    pub fn get_study_names(&self) -> Vec<String> {
        let mut res: Vec<String> = Default::default();
        for study in self.studies.iter() {
            res.push(study.identifier.to_string());
        }
        return res;
    }

    pub fn get_num_studies(&self) -> usize {
        return self.studies.len();
    }

    pub fn commence(
        &mut self,
        loaded_heroes: HashMap<String, SimHero>,
        loaded_dungeons: HashMap<String, Dungeon>,
        loaded_valid_skills: Vec<String>,
        loaded_heroes_from_builder: HashMap<String, Hero>,
        loaded_hero_builder_information: HeroBuilderInformation,
        tx: Sender<(String, u32, u32)>,
    ) {
        info!("Commencing Docket");
        let num_dockets: usize = self.get_num_studies();
        let m = MultiProgress::new();
        let m_sty = ProgressStyle::with_template("{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{len} ({eta_precise})")
        .unwrap()
        .progress_chars("#>-");

        let pb = m.add(ProgressBar::new(num_dockets.try_into().unwrap()));
        pb.set_style(m_sty.clone());
        pb.set_message("DOCKET OVERALL PROGRESS");
        pb.set_position(0);

        tx.send((
            String::from("DOCKET OVERALL PROGRESS"),
            0u32,
            num_dockets as u32,
        ))
        .unwrap();

        let completed_study_count: Arc<Mutex<u32>> = Arc::new(Mutex::new(0u32));

        let mut studies_with_tx: Vec<(&mut DocketStudy, Sender<(String, u32, u32)>)> = self
            .studies
            .iter_mut()
            .map(|study| (study, tx.clone()))
            .collect();

        studies_with_tx.par_iter_mut().for_each(|(docket_study, tx)| {
            // info!(
            //     "Docket Study '{}' [{}]: {} of {}: {}",
            //     docket_study.identifier,
            //     docket_study.type_,
            //     docket_index,
            //     num_dockets,
            //     if docket_study.completed {
            //         "Skipping (Completed)"
            //     } else {
            //         "Starting"
            //     }
            // );
            // Skip completed studies
            if docket_study.completed == true {
                *completed_study_count.lock().unwrap() += 1;
                tx.send((
                    String::from("DOCKET OVERALL PROGRESS"),
                    *completed_study_count.lock().unwrap(), num_dockets as u32
                )).unwrap();
                info!("Skipping study {} since it is already completed.", docket_study.identifier);
                return;
            }

            // Parse Team
            let parse_team_option = parse_team(&docket_study, &loaded_heroes);
            let team: Team;
            match parse_team_option {
                Some(parsed_team) => team = parsed_team,
                None => {
                    info!("\tFailed to Parse Team: Skipping to Next Study");
                    return;
                }
            }
            let team_heroes = team.get_heroes();
            // info!("\tParsed Team");

            // Parse Dungeons
            let parse_dungeons_option = parse_dungeons(&docket_study, &loaded_dungeons);
            let dungeons: Vec<TrialDungeon>;
            match parse_dungeons_option {
                Some(parsed_dungeon) => dungeons = parsed_dungeon,
                None => {
                    info!("\tFailed to Parse Dungeons: Skipping to Next Study");
                    return;
                }
            }
            // info!("\tParsed Dungeons");

            // Parse Excluded/Valid Skills
            let parse_valid_skills_option = parse_valid_skills(
                &docket_study,
                &loaded_valid_skills,
                &loaded_hero_builder_information,
                &team_heroes,
            );
            let valid_skills: Vec<String>;
            match parse_valid_skills_option {
                Some(parsed_vs) => valid_skills = parsed_vs,
                None => {
                    info!("\tFailed to Parse Excluded Skills (Did not conform to expected format): Skipping to Next Study");
                    return;
                }
            }
            // info!("\tParsed Valid/Excluded Skills");

            // Parse Static/Preset Skills
            let mut preset_skills: Vec<String> = Default::default();
            for skill in docket_study
                .preset_skills
                .split(";")
                .map(|s| s.trim())
                .collect::<Vec<&str>>()
            {
                preset_skills.push(skill.to_string());
            }
            let parse_static_skills_option = translate_skillset_based_on_skill_name_format(
                &docket_study.skill_name_format,
                preset_skills,
                &loaded_hero_builder_information,
            );
            let static_skills: Vec<String>;
            match parse_static_skills_option {
                Some(parsed_static_skills) => static_skills = parsed_static_skills,
                None => {
                    info!("\tFailed to Parse Static/Preset Skills (Did not conform to expected format): Skipping to Next Study");
                    return;
                }
            }
            // info!("\tParsed Preset Skills");

            // Determine correct create function based on study type
            match docket_study.type_ {
                DocketStudyType::StaticDuoSkillStudy => {
                    let mut study = create_static_duo_skill_study(
                        docket_study.identifier.to_string(),
                        docket_study.description.to_string(),
                        docket_study.sim_qty,
                        docket_study.runoff_scoring_threshold,
                        team,
                        valid_skills.clone(),
                        static_skills,
                        team_heroes[0].get_identifier().to_string(),
                        loaded_heroes_from_builder[&team_heroes[0].get_identifier()].clone(),
                        dungeons,
                        docket_study.automatic_rank_difficulty_optimization,
                        loaded_hero_builder_information.clone(),
                        loaded_hero_builder_information
                            .hero_skill_abbreviation_map
                            .clone(),
                    );

                    // info!("\tCreated Study (StaticDuoSkillStudy)");

                    // info!(
                    //     "\tSkill Variations Remaining to Test: {}",
                    //     study.count_skill_variations_remaining()
                    // );

                    study.run(&m, &m_sty, tx.clone());

                    // docket_completion_tracker.studies[docket_index - 1].completed = true;

                    *completed_study_count.lock().unwrap() += 1;
                    tx.send((
                        String::from("DOCKET OVERALL PROGRESS"),
                        *completed_study_count.lock().unwrap(), num_dockets as u32
                    )).unwrap();
                    docket_study.completed = true;

                    info!("\n\tStudy Completed");
                    pb.inc(1);
                }
            }
            info!("Docket Study Completed");
            // TODO: Reimplement save_study_docket during execution...
            // save_study_docket(&self.path, &self).unwrap();
        });
        save_study_docket(&self.path, &self).unwrap();
        info!("Docket Completed");
    }
}

fn parse_team(
    docket_study: &DocketStudy,
    loaded_heroes: &HashMap<String, SimHero>,
) -> Option<Team> {
    // Parse Team
    // Parse Heroes
    let mut team_heroes: Vec<SimHero> = vec![];
    for hero_identifier in docket_study
        .team_hero_identifiers
        .split(";")
        .map(|s| s.trim())
        .collect::<Vec<&str>>()
    {
        // Lookup the specified hero identifier
        let hero = loaded_heroes.get(hero_identifier);
        match hero {
            Some(simhero) => team_heroes.push(simhero.clone()),
            None => {
                return None;
            }
        }
    }
    // Parse Booster
    let booster_map: HashMap<String, Option<BoosterType>> = HashMap::from([
        ("None".into(), None),
        ("Power Booster".into(), Some(BoosterType::PowerBooster)),
        (
            "Super Power Booster".into(),
            Some(BoosterType::SuperPowerBooster),
        ),
        (
            "Mega Power Booster".into(),
            Some(BoosterType::MegaPowerBooster),
        ),
    ]);
    let loaded_booster = booster_map.get(&docket_study.team_booster);
    let team_booster: Option<BoosterType>;
    match loaded_booster {
        Some(someteambooster) => team_booster = someteambooster.clone(),
        None => {
            return None;
        }
    }

    return Some(create_team(team_heroes.clone(), team_booster).unwrap());
}

fn parse_dungeons(
    docket_study: &DocketStudy,
    loaded_dungeons: &HashMap<String, Dungeon>,
) -> Option<Vec<TrialDungeon>> {
    let mut dungeons: Vec<TrialDungeon> = Default::default();
    for dungeon_str in docket_study
        .dungeon_specifications
        .split("|")
        .map(|s| s.trim())
        .collect::<Vec<&str>>()
    {
        let dungeon_str_split = dungeon_str
            .split(":")
            .map(|s| s.trim())
            .collect::<Vec<&str>>();
        let dungeon_name = dungeon_str_split[0].to_string();
        let dungeon_difficulty = dungeon_str_split[1].clone();
        let dungeon_miniboss_setting = dungeon_str_split[2].clone();
        let loaded_dungeon = loaded_dungeons.get(&dungeon_name);
        let dun: Dungeon;
        match loaded_dungeon {
            Some(somedun) => dun = somedun.clone(),
            None => {
                return None;
            }
        }
        let diff_map: HashMap<&str, usize> = HashMap::from([
            ("Easy", 1 as usize),
            ("Medium", 2 as usize),
            ("Hard", 3 as usize),
            ("Extreme", 4 as usize),
            ("Boss Easy", 5 as usize),
            ("Boss Medium", 6 as usize),
            ("Boss Hard", 7 as usize),
            ("Boss Extreme", 8 as usize),
        ]);
        let loaded_diff = diff_map.get(dungeon_difficulty);
        let dundiff: usize;
        match loaded_diff {
            Some(somedundiff) => dundiff = somedundiff.clone(),
            None => {
                return None;
            }
        }
        let miniboss_map: HashMap<&str, Option<bool>> = HashMap::from([
            ("No Minibosses", Some(false)),
            ("Only Minibosses", Some(true)),
            ("Random Minibosses", None),
        ]);
        let loaded_miniboss_setting = miniboss_map.get(dungeon_miniboss_setting);
        let dunmb: Option<bool>;
        match loaded_miniboss_setting {
            Some(somedunmb) => dunmb = somedunmb.clone(),
            None => {
                return None;
            }
        }
        dungeons.push(create_trial_dungeon(dun, dundiff, dunmb));
    }

    return Some(dungeons);
}

fn parse_valid_skills(
    docket_study: &DocketStudy,
    loaded_valid_skills: &Vec<String>,
    loaded_hero_builder_information: &HeroBuilderInformation,
    team_heroes: &Vec<SimHero>,
) -> Option<Vec<String>> {
    let translated_excluded_skills_option = translate_skillset_based_on_skill_name_format(
        &docket_study.skill_name_format,
        docket_study
            .excluded_skills
            .split(";")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>(),
        loaded_hero_builder_information,
    );

    let translated_excluded_skills: Vec<String>;
    match translated_excluded_skills_option {
        Some(translated_skills) => translated_excluded_skills = translated_skills,
        None => return None,
    }

    let translated_preset_skills_option = translate_skillset_based_on_skill_name_format(
        &docket_study.skill_name_format,
        docket_study
            .preset_skills
            .split(";")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>(),
        loaded_hero_builder_information,
    );

    let translated_preset_skills: Vec<String>;
    match translated_preset_skills_option {
        Some(translated_skills) => translated_preset_skills = translated_skills,
        None => return None,
    }

    let incompatible_skills =
        loaded_hero_builder_information.get_incompatible_skills(&translated_preset_skills);

    let hero_class = team_heroes[0].get_class();
    let skills_incompatible_with_hero_class =
        loaded_hero_builder_information.get_skills_incompatible_with_hero(hero_class);

    let valid_skillset: HashSet<String> = HashSet::from_iter(loaded_valid_skills.iter().cloned());
    let excluded_skillset: HashSet<String> = HashSet::from_iter(translated_excluded_skills);
    let incompatible_skillset: HashSet<String> = HashSet::from_iter(incompatible_skills);
    let hero_incompatible_skillset: HashSet<String> =
        HashSet::from_iter(skills_incompatible_with_hero_class);
    let preset_skillset: HashSet<String> =
        HashSet::from_iter(translated_preset_skills.iter().cloned());

    let diff_1: HashSet<String> = valid_skillset
        .difference(&excluded_skillset)
        .cloned()
        .collect();
    let diff_2: HashSet<String> = diff_1.difference(&preset_skillset).cloned().collect();
    let diff_3: HashSet<String> = diff_2.difference(&incompatible_skillset).cloned().collect();
    let diff_4: Vec<String> = diff_3
        .difference(&hero_incompatible_skillset)
        .cloned()
        .collect();
    return Some(diff_4);
}

fn translate_skillset_based_on_skill_name_format(
    format: &DocketStudySkillNameFormat,
    skillset: Vec<String>,
    loaded_hero_builder_information: &HeroBuilderInformation,
) -> Option<Vec<String>> {
    if skillset.len() == 0 || skillset[0] == "" {
        return Some(skillset);
    }

    let res: Vec<String>;
    let mut success = true;
    match format {
        DocketStudySkillNameFormat::Abbreviated => {
            res = skillset
                .iter()
                .map(|s| {
                    let translated = loaded_hero_builder_information
                        .hero_abbreviation_skill_map
                        .get(s);
                    match translated {
                        Some(skill) => return skill.to_string(),
                        None => {
                            success = false;
                            return s.to_string();
                        }
                    }
                })
                .collect::<Vec<String>>();
        }
        DocketStudySkillNameFormat::FullAnyTier => {
            res = skillset
                .iter()
                .map(|s| {
                    let translated = loaded_hero_builder_information
                        .hero_skill_any_tier_to_tier_1_name_map
                        .get(s);
                    match translated {
                        Some(skill) => return skill.to_string(),
                        None => {
                            success = false;
                            return s.to_string();
                        }
                    }
                })
                .collect::<Vec<String>>();
        }
        DocketStudySkillNameFormat::FullTierOne => {
            res = skillset;
        }
    }
    return if success { Some(res) } else { None };
}
