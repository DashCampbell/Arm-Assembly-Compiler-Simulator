// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::arm7::{Processor, Program};
use app::backend_api;
use app::backend_api::{GlobalKillSwitch, GlobalProcessor, GlobalProgram};
use app::fc::Folder;
use std::sync::Mutex;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn open_folder(folder_path: &str) -> Result<Folder, ()> {
    Ok(app::fc::read_directory(folder_path))
}

#[tauri::command]
fn get_file_content(file_path: &str) -> Result<String, String> {
    app::fc::read_file(file_path)
}

#[tauri::command]
fn write_file(file_path: &str, content: &str) -> String {
    app::fc::write_file(file_path, content);
    String::from("OK")
}

fn main() {
    println!("Starting App");
    tauri::Builder::default()
        .manage(GlobalProcessor(Mutex::new(Processor::new())))
        .manage(GlobalProgram(Mutex::new(Program::new())))
        .manage(GlobalKillSwitch(Mutex::new(false)))
        .invoke_handler(tauri::generate_handler![
            greet,
            open_folder,
            get_file_content,
            write_file,
            backend_api::compile,
            backend_api::run,
            backend_api::debug_run,
            backend_api::display_cpu,
            backend_api::display_memory,
            backend_api::kill_process,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
