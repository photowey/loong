// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{Local, NaiveDateTime};
use chronounit::formatter::pattern::DateTimePattern;
use chronounit::formatter::{DateTimeFormatter, DefaultDateTimeFormatter};

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
