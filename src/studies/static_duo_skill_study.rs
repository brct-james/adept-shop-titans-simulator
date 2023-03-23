use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};
use log::info;

use crate::{
    dungeons::TrialDungeon, heroes::Team, inputs::convert_loaded_heroes_to_sim_heroes, studies::*,
    trials::create_trial,
};

/// An extension of Study for generating and ranking Trials for each combination of skills for a single hero with a static Duo partner
pub struct StaticDuoSkillStudy {
    study: Study,
    base_team: Team,
    subject_hero_identifier: String, // The identifier of the hero to vary upon, and whose performance will be analyzed for the purposes of this study
    subject_hero_builder: crate::hero_builder::Hero, // The hero builder representation of the subject hero, to be converted to a simhero for variation
    valid_skills: Vec<String>,                       // The vector of all skills to be varied upon
    valid_skills_count: i64,                         // The number of valid skills to vary upon
    preset_skills: Vec<String>, // A vector containing 0-3 innate skills that are preset and unchanging
    varying_skill_slot_count: i64, // The number of skill slots to vary
    skill_combination_index: i64, // The current index of the combinations of the valid_skills list being trialed
    dungeons: Vec<TrialDungeon>, // The dungeons to be tested in the study. Only the first will be used unless automatic_rank_difficulty_optimization is enabled
    _automatic_rank_difficulty_optimization: bool, // Whether to optimize ranking by testing skills above a certain rank on additional dungeons
    skill_abbreviation_map: HashMap<String, String>, // The map used to translate Skill Tier 1 names to Peetee's DuoSkillz Abbreviations
}

pub fn create_static_duo_skill_study(
    identifier: String,
    description: String,
    simulation_qty: i32,
    runoff_scoring_threshold: f64,
    base_team: Team,
    valid_skills: Vec<String>,
    preset_skills: Vec<String>,
    subject_hero_identifier: String,
    subject_hero_builder: crate::hero_builder::Hero,
    dungeons: Vec<TrialDungeon>,
    automatic_rank_difficulty_optimization: bool,
    hero_builder_information: HeroBuilderInformation,
    skill_abbreviation_map: HashMap<String, String>,
) -> StaticDuoSkillStudy {
    let mut vs = valid_skills.clone();
    vs.retain(|x| !preset_skills.contains(x));
    vs.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    return StaticDuoSkillStudy {
        study: create_study(
            identifier,
            description,
            simulation_qty,
            runoff_scoring_threshold,
            hero_builder_information,
        ),
        base_team,
        subject_hero_identifier,
        subject_hero_builder,
        valid_skills_count: vs.len() as i64,
        valid_skills: vs,
        varying_skill_slot_count: 4 - preset_skills.len() as i64,
        preset_skills,
        skill_combination_index: 0,
        dungeons,
        _automatic_rank_difficulty_optimization: automatic_rank_difficulty_optimization,
        skill_abbreviation_map,
    };
}

impl Runnable for StaticDuoSkillStudy {
    /// Handle running trials for the study
    fn run(&mut self) {
        info!("Start Study: {}", self.study.identifier);

        self.study.status = StudyStatus::Running;

        let pb = ProgressBar::new(self._count_skill_variations_total().try_into().unwrap());
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{len} ({eta_precise})")
            .unwrap()
            .progress_chars("#>-"));

        while self.count_skill_variations_remaining() > 0 {
            pb.set_position(self.skill_combination_index.try_into().unwrap());

            // Create the combination of skills to test
            let skill_variation = self.get_full_translated_skillset_at_current_combination_index();

            // Vary the target hero in the team
            let mut new_team = self.base_team.clone();
            let target_hero_index = new_team
                .get_index_of_hero_with_identifier(&self.subject_hero_identifier)
                .unwrap();
            let mut new_hero = self.subject_hero_builder.clone();
            new_hero.set_hero_skills(skill_variation.clone());
            let heroes_hashmap: HashMap<String, crate::hero_builder::Hero> =
                HashMap::from([(self.subject_hero_identifier.to_string(), new_hero)]);
            let new_sim_heroes = convert_loaded_heroes_to_sim_heroes(
                heroes_hashmap,
                self.study.hero_builder_information.bp_map.clone(),
                self.study
                    .hero_builder_information
                    .hero_skill_tier_1_name_map
                    .clone(),
                self.study.hero_builder_information.hero_skill_map.clone(),
                self.study
                    .hero_builder_information
                    .class_innate_skill_names_map
                    .clone(),
                self.study.hero_builder_information.innate_skill_map.clone(),
            );
            new_team.set_hero_at_index(
                target_hero_index,
                new_sim_heroes[&self.subject_hero_identifier].clone(),
            );

            // TODO: Per-trial logging
            // Configure trial log file
            // let mut i = 0;
            // while std::path::Path::new(&f!(
            //     "target/simulations/{}/logs/trial_{}.log",
            //     self.study.identifier,
            //     i
            // ))
            // .exists()
            // {
            //     // Create new log file each run
            //     i += 1;
            // }
            // fast_log::init(fast_log::Config::new().file(&f!(
            //     "target/simulations/{}/logs/trial_{}.log",
            //     self.study.identifier,
            //     i
            // )))
            // .unwrap();
            // info!("Start of Log File");

            // Create new trial with new team
            let mut trial = create_trial(
                format!("{}", self.study.identifier),
                format!("{:?}", skill_variation),
                self.study.simulation_qty as usize,
                new_team,
                self.dungeons[0].dungeon.clone(),
                [self.dungeons[0].difficulty].to_vec(),
                self.dungeons[0].force_minibosses,
                false,
            )
            .unwrap();

            // Run simulations
            let timer = Instant::now();
            trial.run_simulations_single_threaded();
            let timer_duration = timer.elapsed().as_nanos() as f32 / 1000000.0f32;
            info!("Completed trial in {:#?}ms.", timer_duration,);

            // Save Duo Skillz Results
            let duo_skillz_result_csv_path = f!(
                "target/simulations/{}/csvs/duo_skillz_results.csv",
                self.study.identifier
            );
            if let Some(p) = std::path::Path::new(&duo_skillz_result_csv_path).parent() {
                std::fs::create_dir_all(p).unwrap();
            }
            // Save Trial Results
            let trial_result_csv_path = f!(
                "target/simulations/{}/csvs/trial_results.csv",
                self.study.identifier
            );
            if let Some(p) = std::path::Path::new(&trial_result_csv_path).parent() {
                std::fs::create_dir_all(p).unwrap();
            }
            trial
                .save_duo_skillz_and_trial_result_to_csv(
                    duo_skillz_result_csv_path,
                    trial_result_csv_path,
                    self.skill_abbreviation_map.clone(),
                )
                .unwrap();
            self.increment_combination_index();
        }

        // Outside While, this is assumed but check anyways because why not...
        if self.count_skill_variations_remaining() == 0 {
            // TODO: Any other tasks that must be done once finished
            self.study.status = StudyStatus::Finished;
            pb.finish_with_message("Study Complete");
        } else {
            panic!("This should not occur, while running study managed to escape while loop without study being finished status...")
        }
    }
}

impl StaticDuoSkillStudy {
    pub fn _count_skill_variations_completed(&self) -> i64 {
        return self.skill_combination_index;
    }
    pub fn _count_skill_variations_total(&self) -> i64 {
        return crate::combinations::count_combinations(
            self.valid_skills_count,
            self.varying_skill_slot_count,
        );
    }
    pub fn count_skill_variations_remaining(&self) -> i64 {
        return crate::combinations::count_combinations(
            self.valid_skills_count,
            self.varying_skill_slot_count,
        ) - self.skill_combination_index;
    }
    pub fn _get_skillset_at_specific_combination_index(&self, combination_index: i64) -> Vec<i64> {
        return crate::combinations::iter_combination(
            combination_index,
            self.valid_skills_count,
            self.varying_skill_slot_count,
        );
    }
    pub fn get_skillset_at_current_combination_index(&self) -> Vec<i64> {
        return crate::combinations::iter_combination(
            self.skill_combination_index,
            self.valid_skills_count,
            self.varying_skill_slot_count,
        );
    }
    pub fn increment_combination_index(&mut self) {
        self.skill_combination_index += 1;
    }
    pub fn translate_skillset_from_indices(&self, indices_array: Vec<i64>) -> Vec<String> {
        let mut res = vec![];
        for idx in indices_array {
            res.push(self.valid_skills[idx as usize].clone());
        }
        return res;
    }
    pub fn get_full_translated_skillset_at_current_combination_index(&self) -> Vec<String> {
        let mut res = self.preset_skills.clone();
        let mut translated_skillset =
            self.translate_skillset_from_indices(self.get_skillset_at_current_combination_index());
        res.append(&mut translated_skillset);
        return res;
    }
}
