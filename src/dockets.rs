use std::collections::{HashMap, HashSet};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};

use rayon::prelude::*;

use crate::dungeons::create_trial_dungeon;
use crate::dungeons::{Dungeon, TrialDungeon};
use crate::equipment::BoosterType;
use crate::hero_builder::Hero;
use crate::heroes::{create_team, Team};
use crate::inputs::save_study_docket;
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
            println!("\tSkipping Record {}: Identifier Is Required", result_index);
            return false;
        }
        if self.sim_qty <= 0 || self.sim_qty > 50000 {
            println!(
                "\tSkipping Record {}: Sim Qty Must Be In Range [1,50000]",
                result_index
            );
            return false;
        }
        if self.runoff_scoring_threshold <= 0.0 || self.runoff_scoring_threshold > 100.0 {
            println!(
                "\tSkipping Record {}: Sim Qty Must Be In Range [1,100]",
                result_index
            );
            return false;
        }
        if self.team_hero_identifiers.len() == 0 {
            println!(
                "\tSkipping Record {}: Team Hero Identifiers Are Required",
                result_index
            );
            return false;
        }
        if self.team_booster.len() == 0 {
            println!(
                "\tSkipping Record {}: Team Booster Is Required",
                result_index
            );
            return false;
        }
        // Preset Skills are NOT required, can be empty to vary all skill slots
        if self.dungeon_specifications.len() == 0 {
            println!(
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
    ) {
        println!("Commencing Docket");
        let num_dockets: usize = self.get_num_studies();
        let m = MultiProgress::new();
        let m_sty = ProgressStyle::with_template("{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{len} ({eta_precise})")
        .unwrap()
        .progress_chars("#>-");

        let pb = m.add(ProgressBar::new(num_dockets.try_into().unwrap()));
        pb.set_style(m_sty.clone());
        pb.set_message("DOCKET OVERALL PROGRESS");
        pb.set_position(0);

        self.studies.par_iter_mut().for_each(|docket_study| {
            // println!(
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
                return;
            }

            // Parse Team
            let parse_team_option = parse_team(&docket_study, &loaded_heroes);
            let team: Team;
            match parse_team_option {
                Some(parsed_team) => team = parsed_team,
                None => {
                    println!("\tFailed to Parse Team: Skipping to Next Study");
                    return;
                }
            }
            let team_heroes = team.get_heroes();
            // println!("\tParsed Team");

            // Parse Dungeons
            let parse_dungeons_option = parse_dungeons(&docket_study, &loaded_dungeons);
            let dungeons: Vec<TrialDungeon>;
            match parse_dungeons_option {
                Some(parsed_dungeon) => dungeons = parsed_dungeon,
                None => {
                    println!("\tFailed to Parse Dungeons: Skipping to Next Study");
                    return;
                }
            }
            // println!("\tParsed Dungeons");

            // Parse Excluded/Valid Skills
            let parse_valid_skills_option = parse_valid_skills(
                &docket_study,
                &loaded_valid_skills,
                &loaded_hero_builder_information,
            );
            let valid_skills: Vec<String>;
            match parse_valid_skills_option {
                Some(parsed_vs) => valid_skills = parsed_vs,
                None => {
                    println!("\tFailed to Parse Excluded Skills (Did not conform to expected format): Skipping to Next Study");
                    return;
                }
            }
            // println!("\tParsed Valid/Excluded Skills");

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
                    println!("\tFailed to Parse Static/Preset Skills (Did not conform to expected format): Skipping to Next Study");
                    return;
                }
            }
            // println!("\tParsed Preset Skills");

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

                    // println!("\tCreated Study (StaticDuoSkillStudy)");

                    // println!(
                    //     "\tSkill Variations Remaining to Test: {}",
                    //     study.count_skill_variations_remaining()
                    // );

                    study.run(&m, &m_sty);

                    // docket_completion_tracker.studies[docket_index - 1].completed = true;

                    docket_study.completed = true;

                    println!("\n\tStudy Completed");
                    pb.inc(1);
                }
            }
            println!("Docket Study Completed");
            // TODO: Reimplement save_study_docket during execution...
            // save_study_docket(&self.path, &self).unwrap();
        });
        save_study_docket(&self.path, &self).unwrap();
        println!("Docket Completed");
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
) -> Option<Vec<String>> {
    let translation_option = translate_skillset_based_on_skill_name_format(
        &docket_study.skill_name_format,
        docket_study
            .excluded_skills
            .split(";")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>(),
        loaded_hero_builder_information,
    );

    let translated_excluded_skills: Vec<String>;
    match translation_option {
        Some(translated_skills) => translated_excluded_skills = translated_skills,
        None => return None,
    }
    let valid_skillset: HashSet<String> = HashSet::from_iter(loaded_valid_skills.iter().cloned());
    let excluded_skillset: HashSet<String> = HashSet::from_iter(translated_excluded_skills);
    let difference: Vec<String> = valid_skillset
        .difference(&excluded_skillset)
        .cloned()
        .collect();
    return Some(difference);
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
