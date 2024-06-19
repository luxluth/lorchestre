// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use muconf::Config;
use std::env::consts::OS;
use tauri::Manager;

#[tauri::command]
fn platform() -> String {
    OS.to_string()
}

#[tauri::command]
fn locale(app: tauri::AppHandle) -> String {
    let path = app.path().app_config_dir().unwrap().join("config.toml");
    let config = Config::get(&path);

    if let Some(global) = config.global {
        if let Some(lang) = global.lang {
            return lang;
        }
    }

    "en-GB".to_string()
}

#[tauri::command]
fn set_locale(app: tauri::AppHandle, locale: String) {
    let path = app.path().app_config_dir().unwrap().join("config.toml");
    Config::load_update_save(&path, |config| {
        config.update_global_field(|global| {
            global.lang = Some(locale);
        });
    });
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![platform, locale, set_locale,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
