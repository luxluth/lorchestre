// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mu::Media;
use tauri::Manager;

#[tauri::command]
fn index(app: tauri::AppHandle) -> Media {
    Media::new(app.path().app_cache_dir().unwrap())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![index])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
