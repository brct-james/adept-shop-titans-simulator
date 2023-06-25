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
        load_dungeons_from_yaml, load_hero_classes_from_yaml, load_heroes_as_sim_heroes_from_tsv,
        load_heroes_from_tsv, load_skill_abbreviation_map, load_study_docket,
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
    pub progress: IndexMap<String, (u32, u32, Instant, Instant)>, // (identifier, (progress, total, start_time, end_time))
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
                    (String::from("adept_data/config/hero_builder.tsv"), false),
                ),
                (
                    String::from("Study Docket"),
                    (String::from("adept_data/config/study_docket.tsv"), false),
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
                        let loaded_heroes = load_heroes_as_sim_heroes_from_tsv(
                            &path,
                            self.sim_data.bp_map.clone(),
                            self.sim_data.hero_classes.clone(),
                            self.sim_data.hero_skill_tier_1_name_map.clone(),
                            self.sim_data.hero_skill_map.clone(),
                            self.sim_data.class_innate_skill_names_map.clone(),
                            self.sim_data.innate_skill_map.clone(),
                        );
                        let loaded_heroes_from_builder = load_heroes_from_tsv(
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
            *self
                .progress
                .entry(msg.0)
                .or_insert((msg.1, msg.2, msg.3, Instant::now())) =
                (msg.1, msg.2, msg.3, Instant::now());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(egui::RichText::new("Adept - Shop Titans Combat Simulator").strong());
            ui.vertical(|ui| {
                ui.heading(egui::RichText::new("Loaded Files").strong());
                egui::Grid::new("sim_stats_grid").striped(true).show(ui, |ui| {
                    ui.label(egui::RichText::new("Loaded?").strong());
                    ui.label(egui::RichText::new("Identifier").strong());
                    ui.label(egui::RichText::new("Expected Path").strong());
                    ui.end_row();
                    for (file_id, (file_location, mut file_loaded)) in self.required_files.iter() {
                        ui.add_enabled(false, egui::widgets::Checkbox::new(&mut file_loaded, ""));
                        ui.label(file_id);
                        ui.label(file_location);
                        ui.end_row()
                    }
                });
            });
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Select the Study to Run:").strong());
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
                egui::Grid::new("sim_stats_grid").striped(true).show(ui, |ui| {
                    ui.label(egui::RichText::new("").strong());
                    ui.label(egui::RichText::new("Identifier").strong());
                    ui.label(egui::RichText::new("Completed Variants").strong());
                    ui.label(egui::RichText::new("Total Variants").strong());
                    ui.label(egui::RichText::new("Time Elapsed").strong());
                    ui.label(egui::RichText::new("Est. Time Remaining").strong());
                    ui.horizontal(|ui| {
                        ui.set_width(200.0);
                        ui.label(egui::RichText::new("").strong());
                    });
                    ui.end_row();
                    for (study, (progress, total, start, end)) in self.progress.iter() {
                        if progress < total {
                            let elapsed = start.elapsed().as_secs();
                            let seconds_elapsed = elapsed % 60;
                            let minutes_elapsed = (elapsed / 60) % 60;
                            let hours_elapsed = (elapsed / 60) / 60;
                            let mut estimated: u64;
                            if *progress != 0u32 {
                                estimated = elapsed * *total as u64 / *progress as u64;
                            } else {
                                estimated = elapsed * *total as u64 / 1 as u64;
                            }
                            if *study == String::from("DOCKET OVERALL PROGRESS") {
                                let max_estimated: u64 = self.progress.iter().map(|(_, (v_prog, v_tot, v_start, v_end))| {
                                    if v_prog >= v_tot {
                                        return v_end.duration_since(*v_start).as_secs();
                                    }
                                    let v_elapsed = v_start.elapsed().as_secs();
                                    let v_estimated: u64;
                                    if *v_prog != 0u32 {
                                        v_estimated = v_elapsed * *v_tot as u64 / *v_prog as u64;
                                    } else {
                                        v_estimated = v_elapsed * *v_tot as u64 / 1 as u64;
                                    }
                                    return v_estimated;
                                }).max().unwrap();
                                estimated = max_estimated;
                            }
                            let seconds_remaining = (estimated - elapsed) % 60;
                            let minutes_remaining = ((estimated - elapsed) / 60) % 60;
                            let hours_remaining = ((estimated - elapsed) / 60) / 60;
                            ui.add_visible(*study != String::from("DOCKET OVERALL PROGRESS"), egui::widgets::Spinner::new());
                            if *study == String::from("DOCKET OVERALL PROGRESS") {
                                ui.label(egui::RichText::new("Docket Overall").strong().underline());
                            } else {
                                ui.label(study);
                            }
                            ui.label(format!("{}", progress));
                            ui.label(format!("{}", total));
                            ui.label(format!("{:0>2}:{:0>2}:{:0>2}", hours_elapsed, minutes_elapsed, seconds_elapsed));
                            ui.label(format!("{:0>2}:{:0>2}:{:0>2}", hours_remaining, minutes_remaining, seconds_remaining));
                            ui.add(
                                egui::widgets::ProgressBar::new(*progress as f32 / *total as f32)
                                    .show_percentage(),
                            );
                            ui.end_row();
                        } else {
                            ui.horizontal(|ui| {
                                let elapsed = end.duration_since(*start).as_secs();
                                let seconds_elapsed = elapsed % 60;
                                let minutes_elapsed = (elapsed / 60) % 60;
                                let hours_elapsed = (elapsed / 60) / 60;
                                ui.label("");
                                ui.label(study);
                                ui.label(format!("{}", progress));
                                ui.label(format!("{}", total));
                                ui.label(format!("{:0>2}:{:0>2}:{:0>2}", hours_elapsed, minutes_elapsed, seconds_elapsed));
                                ui.label(format!("00:00:00"));
                                ui.add(
                                    egui::widgets::ProgressBar::new(*progress as f32 / *total as f32)
                                        .show_percentage(),
                                );
                                ui.end_row();
                                ui.add(
                                    egui::widgets::ProgressBar::new(*progress as f32 / *total as f32)
                                        .show_percentage(),
                                );
                            });
                        }
                    }
                });
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
