// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time};

use chrono::{Local, NaiveDateTime};
use chronounit::formatter::pattern::DateTimePattern;
use chronounit::formatter::{DateTimeFormatter, DefaultDateTimeFormatter};
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let now = Local::now();
    let ndt: NaiveDateTime = now.naive_local();

    let dtf = DefaultDateTimeFormatter::new(DateTimePattern::YyyyMmDd);
    let today = dtf.format_naive_date_time_default(&ndt);

    format!(
        "Hello, {}! You've been greeted from Rust! today is: [{}]",
        name, today
    )
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                thread::sleep(time::Duration::from_millis(2000));

                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, close_splashscreen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
