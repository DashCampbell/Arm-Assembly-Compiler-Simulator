// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fc;
mod arm7;
mod Process;

use fc::Folder;
use std::sync::Mutex;
use arm7::Processor;
use Process::GlobalProcessor;


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn open_folder(folder_path: &str) -> Result<Folder, ()> {
    Ok(fc::read_directory(folder_path))
}

#[tauri::command]
fn get_file_content(file_path: &str) -> Result<String, String> {
    fc::read_file(file_path)
}

#[tauri::command]
fn write_file(file_path: &str, content: &str) -> String {
    fc::write_file(file_path, content);
    String::from("OK")
}

fn main() {
    println!("Starting App");
    tauri::Builder::default()
        .manage(GlobalProcessor(Mutex::new(Processor::new())))
        .invoke_handler(tauri::generate_handler![
            greet,
            open_folder,
            get_file_content,
            write_file,
            Process::compile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
