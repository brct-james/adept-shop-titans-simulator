use eframe::egui;
use indexmap::IndexMap;
use log::{error, info};
use std::{
    sync::mpsc::{Receiver, Sender},
    time::Instant,
};

use crate::{
    dockets::Docket,
    init,
    inputs::{
        load_dungeons_from_yaml, load_hero_classes_from_yaml, load_heroes_as_sim_heroes_from_csv,
        load_heroes_from_csv, load_skill_abbreviation_map, load_study_docket,
    },
    sheet_processing::{get_hero_equipment_data, get_hero_skills_data, get_innate_skills_data},
    simdata::SimData,
};

pub struct AdeptApp {
    pub tx: Sender<(String, u32, u32, Instant)>,
    pub rx: Receiver<(String, u32, u32, Instant)>,
    pub started: bool,
    pub docket: Docket,
    pub selected_study: String,
    pub required_files: IndexMap<String, (String, bool)>,
    pub sim_data: SimData,
    pub sim_running: bool,
    pub progress: IndexMap<String, (u32, u32, Instant)>, // (identifier, (progress, total, start_time))
}

impl Default for AdeptApp {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            tx,
            rx,
            started: false,
            docket: Default::default(),
            selected_study: String::from("None"),
            required_files: IndexMap::<String, (String, bool)>::from([
                (
                    String::from("Dungeons"),
                    (String::from("adept_data/bundle/dungeons.yaml"), false),
                ),
                (
                    String::from("Blueprints"),
                    (String::from("adept_data/bundle/blueprints.tsv"), false),
                ),
                (
                    String::from("Innate Skills"),
                    (String::from("adept_data/bundle/innate_skills.tsv"), false),
                ),
                (
                    String::from("Hero Skills"),
                    (String::from("adept_data/bundle/hero_skills.tsv"), false),
                ),
                (
                    String::from("Hero Classes"),
                    (String::from("adept_data/bundle/hero_classes.yaml"), false),
                ),
                (
                    String::from("Skill Abbreviations"),
                    (
                        String::from("adept_data/bundle/skill_abbreviation_map.csv"),
                        false,
                    ),
                ),
                (
                    String::from("Hero Builder"),
                    (String::from("adept_data/config/hero_builder.csv"), false),
                ),
                (
                    String::from("Study Docket"),
                    (String::from("adept_data/config/study_docket.csv"), false),
                ),
            ]),
            sim_data: Default::default(),
            sim_running: false,
            progress: Default::default(),
        }
    }
}

impl eframe::App for AdeptApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.started {
            // Not started, attempt to start

            init::create_logfile();

            // Attempt to Load Files
            for (key, (path, _)) in self.required_files.clone() {
                let mut load_success = false;
                match key.as_str() {
                    "Dungeons" => {
                        info!("Loading Dungeons");
                        let loaded_dungeons = load_dungeons_from_yaml(&path);
                        if loaded_dungeons.len() > 0 {
                            self.sim_data.loaded_dungeons = loaded_dungeons;
                            load_success = true;
                        }
                    }
                    "Hero Builder" => {
                        info!("Loading Hero Builder");
                        if self.sim_data.bp_map.len() == 0
                            || self.sim_data.hero_classes.len() == 0
                            || self.sim_data.hero_skill_tier_1_name_map.len() == 0
                            || self.sim_data.hero_skill_map.len() == 0
                            || self.sim_data.class_innate_skill_names_map.len() == 0
                            || self.sim_data.innate_skill_map.len() == 0
                        {
                            error!("Hero builder could not be loaded because one or more of the files it depends on was not loaded");
                            log::logger().flush();
                            panic!("Hero builder could not be loaded because one or more of the files it depends on was not loaded");
                        }
                        let loaded_heroes = load_heroes_as_sim_heroes_from_csv(
                            &path,
                            self.sim_data.bp_map.clone(),
                            self.sim_data.hero_classes.clone(),
                            self.sim_data.hero_skill_tier_1_name_map.clone(),
                            self.sim_data.hero_skill_map.clone(),
                            self.sim_data.class_innate_skill_names_map.clone(),
                            self.sim_data.innate_skill_map.clone(),
                        );
                        let loaded_heroes_from_builder = load_heroes_from_csv(
                            &path,
                            self.sim_data.bp_map.clone(),
                            self.sim_data.hero_classes.clone(),
                        );
                        if loaded_heroes.len() > 0 && loaded_heroes_from_builder.len() > 0 {
                            self.sim_data.loaded_heroes = loaded_heroes;
                            self.sim_data.loaded_heroes_from_builder = loaded_heroes_from_builder;
                            load_success = true;
                        }
                    }
                    "Blueprints" => {
                        info!("Loading Blueprints");
                        let bp_map = get_hero_equipment_data(&path);
                        if bp_map.len() > 0 {
                            self.sim_data.bp_map = bp_map;
                            load_success = true;
                        }
                    }
                    "Innate Skills" => {
                        info!("Loading Innate Skills");
                        let (
                            innate_skill_tier_1_name_map,
                            innate_skill_any_tier_to_tier_1_name_nap,
                            class_innate_skill_names_map,
                            innate_skill_map,
                        ) = get_innate_skills_data(&path);
                        if innate_skill_tier_1_name_map.len() > 0
                            && innate_skill_any_tier_to_tier_1_name_nap.len() > 0
                            && class_innate_skill_names_map.len() > 0
                            && innate_skill_map.len() > 0
                        {
                            self.sim_data.innate_skill_tier_1_name_map =
                                innate_skill_tier_1_name_map;
                            self.sim_data.innate_skill_any_tier_to_tier_1_name_nap =
                                innate_skill_any_tier_to_tier_1_name_nap;
                            self.sim_data.class_innate_skill_names_map =
                                class_innate_skill_names_map;
                            self.sim_data.innate_skill_map = innate_skill_map;
                            load_success = true;
                        }
                    }
                    "Hero Skills" => {
                        info!("Loading Hero Skills");
                        let (
                            hero_skill_tier_1_name_map,
                            hero_skill_any_tier_to_tier_1_name_map,
                            hero_skill_map,
                        ) = get_hero_skills_data(&path);
                        let mut loaded_valid_skills: Vec<String> = Default::default();
                        for (k, v) in &hero_skill_tier_1_name_map {
                            let ksplit: Vec<&str> = k.split(' ').collect();
                            if ksplit[ksplit.len() - 1] == "T4" {
                                loaded_valid_skills.push(v.to_string());
                            }
                        }
                        if hero_skill_tier_1_name_map.len() > 0
                            && hero_skill_any_tier_to_tier_1_name_map.len() > 0
                            && hero_skill_map.len() > 0
                            && loaded_valid_skills.len() > 0
                        {
                            self.sim_data.hero_skill_tier_1_name_map = hero_skill_tier_1_name_map;
                            self.sim_data.hero_skill_any_tier_to_tier_1_name_map =
                                hero_skill_any_tier_to_tier_1_name_map;
                            self.sim_data.hero_skill_map = hero_skill_map;
                            self.sim_data.loaded_valid_skills = loaded_valid_skills;
                            load_success = true;
                        }
                    }
                    "Hero Classes" => {
                        info!("Loading Hero Classes");
                        let hero_classes = load_hero_classes_from_yaml(&path);
                        if hero_classes.len() > 0 {
                            self.sim_data.hero_classes = hero_classes;
                            load_success = true;
                        }
                    }
                    "Skill Abbreviations" => {
                        info!("Loading Skill Abbreviations");
                        let (hero_skill_abbreviation_map, hero_abbreviation_skill_map) =
                            load_skill_abbreviation_map(&path);
                        if hero_skill_abbreviation_map.len() > 0
                            && hero_abbreviation_skill_map.len() > 0
                        {
                            self.sim_data.hero_skill_abbreviation_map = hero_skill_abbreviation_map;
                            self.sim_data.hero_abbreviation_skill_map = hero_abbreviation_skill_map;
                            load_success = true;
                        }
                    }
                    "Study Docket" => {
                        info!("Loading Docket");
                        let docket = load_study_docket(&path);
                        // if docket.get_num_studies() == 0 {
                        //     info!(
                        //         "Docket Loaded with {} Studies: Program Closing",
                        //         docket.get_num_studies()
                        //     );
                        //     return;
                        // }
                        info!("Docket Loaded with {} Studies", docket.get_num_studies());
                        if docket.get_num_studies() > 0 {
                            self.docket = docket;
                            load_success = true;
                        }
                    }
                    _ => info!("Unhandled Required Files Key: {}", key),
                }
                if load_success {
                    self.required_files
                        .entry(key)
                        .and_modify(|e| *e = (path.to_string(), true));
                }
            }

            // Started
            self.started = true;
        }

        // Handle receiving from channel
        if let Ok(msg) = self.rx.try_recv() {
            *self.progress.entry(msg.0).or_insert((msg.1, msg.2, msg.3)) = (msg.1, msg.2, msg.3);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Adept - Shop Titans Combat Simulator");
            ui.vertical(|ui| {
                ui.heading("Loaded Files:");
                for (file_id, (file_location, mut file_loaded)) in self.required_files.iter() {
                    ui.horizontal(|ui| {
                        ui.add_enabled(false, egui::widgets::Checkbox::new(&mut file_loaded, ""));
                        ui.label(format!(
                            "{} | Expected Location: {}",
                            file_id, file_location
                        ))
                    });
                }
            });
            ui.horizontal(|ui| {
                ui.label("Select the Study to Run:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{}", self.selected_study))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_study, String::from("None"), "None");
                        ui.selectable_value(&mut self.selected_study, String::from("All"), "All");
                        for study in self.docket.get_study_names() {
                            ui.selectable_value(
                                &mut self.selected_study,
                                study.to_string(),
                                study.to_string(),
                            );
                        }
                    });
            });
            ui.horizontal(|ui| {
                ui.label("NOTICE: Currently the filter is not working, so the sim defaults to 'All' regardless");
                if ui
                    .add_enabled(
                        self.selected_study != String::from("None") && !self.sim_running,
                        egui::widgets::Button::new("START SIMULATION"),
                    )
                    .clicked()
                    && !self.sim_running
                {
                    self.sim_running = true;
                    start_docket(self);
                    // self.sim_running = false;
                }
            });
            ui.add_visible_ui(self.sim_running, |ui| {
                for (study, (progress, total, start)) in self.progress.iter() {
                    if progress < total {
                        ui.horizontal(|ui| {
                            ui.add(egui::widgets::Spinner::new());
                            let elapsed = start.elapsed().as_secs();
                            let seconds_elapsed = elapsed % 60;
                            let minutes_elapsed = (elapsed / 60) % 60;
                            let hours_elapsed = (elapsed / 60) / 60;
                            let estimated: u64;
                            if *progress != 0u32 {
                                estimated = elapsed * *total as u64 / *progress as u64;
                            } else {
                                estimated = elapsed * *total as u64 / 1 as u64;
                            }
                            let seconds_remaining = (estimated - elapsed) % 60;
                            let minutes_remaining = ((estimated - elapsed) / 60) % 60;
                            let hours_remaining = ((estimated - elapsed) / 60) / 60;
                            let formatted_string: String;
                            if *study == String::from("DOCKET OVERALL PROGRESS") {
                                formatted_string = format!("{} [{} / {}]", study, progress, total);
                            } else {
                                formatted_string = format!("{} [{} / {}] ({:0>2}:{:0>2}:{:0>2} elapsed, {:0>2}:{:0>2}:{:0>2} est. remaining)", study, progress, total, hours_elapsed, minutes_elapsed, seconds_elapsed, hours_remaining, minutes_remaining, seconds_remaining);
                            }
                            ui.label(formatted_string);
                            ui.add(
                                egui::widgets::ProgressBar::new(*progress as f32 / *total as f32)
                                    .show_percentage(),
                            );
                        });
                    } else {
                        ui.horizontal(|ui| {
                            ui.label(format!("FINISHED | {} [{} / {}]", study, progress, total));
                            ui.add(
                                egui::widgets::ProgressBar::new(*progress as f32 / *total as f32)
                                    .show_percentage(),
                            );
                        });
                    }
                }
            });
        });
    }
}

// fn start_docket(tx: Sender<u32>) {
fn start_docket(adept_app: &mut AdeptApp) {
    let mut docket = adept_app.docket.clone();
    let mut sim_data = adept_app.sim_data.clone();
    let tx = adept_app.tx.clone();
    tokio::spawn(async move {
        crate::dockets::commence_from_gui(&mut docket, &mut sim_data, tx);
    });
}
