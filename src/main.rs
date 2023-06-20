#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// use std::thread;
// use std::time::Duration;

#[macro_use]
extern crate fstrings;

mod equipment;

mod heroes;

mod dungeons;

mod simulations;

mod trials;

mod inputs;

mod decimals;

mod skills;

mod hero_builder;

mod sheet_processing;

mod studies;

mod combinations;

mod dockets;

mod gui;

mod init;

mod simdata;

fn main() -> Result<(), eframe::Error> {
    let rt = tokio::runtime::Runtime::new().expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    // // Execute the runtime in its own thread.
    // // The future doesn't have to do anything. In this example, it just sleeps forever.
    // std::thread::spawn(move || {
    //     rt.block_on(async {
    //         loop {
    //             tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    //         }
    //     })
    // });

    // UI
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 400.0)),
        min_window_size: Some(egui::vec2(600.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Adept - Shop Titans Combat Simulator",
        options,
        Box::new(|_cc| Box::<gui::AdeptApp>::default()),
    )
}
