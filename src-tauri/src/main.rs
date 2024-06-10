// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod Process;
mod arm7;
mod error;
mod fc;
mod helpers;
mod instructions;

use fc::Folder;
use std::sync::Mutex;

use arm7::Processor;
use arm7::Program;
use Process::GlobalProcessor;
use Process::GlobalProgram;

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
        .manage(GlobalProgram(Mutex::new(Program::new())))
        .invoke_handler(tauri::generate_handler![
            greet,
            open_folder,
            get_file_content,
            write_file,
            Process::compile,
            Process::run,
            Process::display_CPU,
            Process::display_Memory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
