#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::io::Read;
use std::io::Write;
use tauri_plugin_log::LogTarget;

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet, write_file, read_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    let mut user_name = name;
    if user_name.is_empty() {
        user_name = "Anonymous";
    }
    format!("Haluuuu {} I'm from Rust", user_name)
}

#[tauri::command]
fn write_file(path: &str, data: &str) -> String {
    let mut file = std::fs::File::create(path).expect("create failed");
    file.write_all(data.as_bytes()).expect("write failed");
    return "data written to file".to_string();
}

#[tauri::command]
fn read_file(path: &str) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}
