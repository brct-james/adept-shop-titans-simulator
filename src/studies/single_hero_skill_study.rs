use crate::{dungeons::Dungeon, heroes::Team, studies::*};

/// An extension of Study for generating and ranking Trials for each combination of skills for a single hero on a team
pub struct SingleHeroSkillStudy {
    study: Study,
    base_team: Team,
    subject_hero_identifier: String, // The identifier of the hero to vary upon, and whose performance will be analyzed for the purposes of this study
    valid_skills: Vec<String>,       // The vector of all skills to be varied upon
    valid_skills_count: i64,         // The number of valid skills to vary upon
    preset_skills: Vec<String>, // A vector containing 0-3 innate skills that are preset and unchanging
    varying_skill_slot_count: i64,   // The number of skill slots to vary
    skill_combination_index: i64, // The current index of the combinations of the valid_skills list being trialed
    dungeons: Vec<Dungeon>,
}

pub fn create_single_hero_skill_study(
    identifier: String,
    description: String,
    simulation_qty: i32,
    runoff_scoring_threshold: f64,
    base_team: Team,
    valid_skills: Vec<String>,
    preset_skills: Vec<String>,
    subject_hero_identifier: String,
    dungeons: Vec<Dungeon>,
) -> SingleHeroSkillStudy {
    let mut vs = valid_skills.clone();
    vs.retain(|x| !preset_skills.contains(x));
    vs.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    return SingleHeroSkillStudy {
        study: create_study(
            identifier,
            description,
            simulation_qty,
            runoff_scoring_threshold,
        ),
        base_team,
        subject_hero_identifier,
        valid_skills_count: vs.len() as i64,
        valid_skills: vs,
        varying_skill_slot_count: 4 - preset_skills.len() as i64,
        preset_skills,
        skill_combination_index: 0,
        dungeons,
    };
}

/* Optimizing Variation Generation
1. Identify the class of the subject hero and filter out incompatible skills
2. Identify the build of the subject hero and filter out the mismatched weapon skills
3. Consider the feasibility of restricting the first two skills to epic/rare and all remaining valid skills for the rest

Limitations
1. Must be able to resume from mid-generation by saving combination index, the current step, and the current list
2. Combination generator is naive to the skills themselves, only works on indices of the current list
2. a. Because of this, must skip incompatible_with skills while executing trials rather than before generation
2. b. OR Decide a subset of these beforehand, prefiltering the list and yielding far fewer combinations! - In what universe is the bronze ever better than the gold?
2. b. i. Note however that this does somewhat defeat the purpose of an objective ranking system for all skills...
*/

impl SingleHeroSkillStudy {
    pub fn count_skill_variations_completed(&self) -> i64 {
        return self.skill_combination_index;
    }
    pub fn count_skill_variations_total(&self) -> i64 {
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
    pub fn get_skillset_at_specific_combination_index(&self, combination_index: i64) -> Vec<i64> {
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
