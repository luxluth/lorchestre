// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lorconf::Config;
mod daemon;
use crate::daemon::entry::start;
use std::env::consts::OS;
use tauri::Manager;
use tracing_subscriber::FmtSubscriber;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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

fn daemon_(path: std::path::PathBuf) -> String {
    let config = Config::get(&path);
    let default_net = lorconf::Network::default();
    let mut port = default_net.port.unwrap();
    let mut host = default_net.host.unwrap();

    if let Some(net) = config.network {
        if let Some(h) = net.host {
            host = h;
        }

        if let Some(p) = net.port {
            port = p;
        }
    }

    format!("{host}:{port}")
}

#[tauri::command]
fn daemon_endpoint(app: tauri::AppHandle) -> String {
    let path = app.path().app_config_dir().unwrap().join("config.toml");
    daemon_(path)
}

#[tauri::command]
fn version() -> String {
    VERSION.to_string()
}

#[tauri::command]
async fn sync_music(app: tauri::AppHandle, window: tauri::Window) {
    let path = app.path().app_config_dir().unwrap().join("config.toml");
    let endpoint = format!("http://{}/updatemusic", daemon_(path));
    let _ = window.emit("startsync", "");
    let client = reqwest::Client::new();
    let _ = client.put(endpoint).send().await;
    let _ = window.emit("endsync", "");
}

#[tauri::command]
fn config(app: tauri::AppHandle) -> Config {
    let path = app.path().app_config_dir().unwrap().join("config.toml");
    Config::get(&path)
}

#[tauri::command]
fn default_config() -> Config {
    Config::default()
}

#[tauri::command]
fn set_locale(app: tauri::AppHandle, locale: String) -> Config {
    let path = app.path().app_config_dir().unwrap().join("config.toml");
    let mut config = Config::get(&path);
    lorconf::update_conf!(config, global, lang, Some(locale));
    Config::dump(&path, config.clone());

    config
}

#[tauri::command]
fn set_theme(app: tauri::AppHandle, theme: String) -> Config {
    let path = app.path().app_config_dir().unwrap().join("config.toml");
    let mut config = Config::get(&path);
    lorconf::update_conf!(config, global, theme, Some(theme));
    Config::dump(&path, config.clone());

    config
}

#[tauri::command]
fn set_blur(app: tauri::AppHandle, state: bool) -> Config {
    let path = app.path().app_config_dir().unwrap().join("config.toml");
    let mut config = Config::get(&path);
    lorconf::update_conf!(config, global, enable_blur, Some(state));
    Config::dump(&path, config.clone());

    config
}

#[tauri::command]
fn app_info(app: tauri::AppHandle) -> AppInfoExternal {
    let path = app.path().app_cache_dir().unwrap().join("__runned");
    AppInfoExternal {
        first_run: !path.exists(),
    }
}

#[tauri::command]
fn runned(app: tauri::AppHandle) {
    let path = app.path().app_cache_dir().unwrap().join("__runned");
    let _ = std::fs::File::create(path);
}

#[tauri::command]
fn start_daemon(window: tauri::Window) {
    tokio::task::spawn(async move {
        let _ = start(Some(window)).await;
    });
}

#[derive(Debug, serde::Serialize)]
struct AppInfoExternal {
    first_run: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    tauri::Builder::default()
        .plugin(tauri_plugin_decorum::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            platform,
            locale,
            set_locale,
            set_theme,
            set_blur,
            config,
            default_config,
            daemon_endpoint,
            sync_music,
            version,
            app_info,
            runned,
            start_daemon
        ])
        .setup(move |app| {
            let is_first_run = !app
                .path()
                .app_cache_dir()
                .unwrap()
                .join("__runned")
                .exists();
            tokio::task::spawn(async move {
                if !is_first_run {
                    let _ = start(None).await;
                }
            });

            // let main_window = app.get_webview_window("main").unwrap();
            // Get this from cache
            // let _ = main_window.set_size(tauri::Size::Logical(tauri::LogicalSize {
            //     width: 800.0,
            //     height: 600.0,
            // }));

            #[cfg(target_os = "windows")]
            {
                use tauri_plugin_decorum::WebviewWindowExt;

                let main_window = app.get_webview_window("main").unwrap();
                main_window.create_overlay_titlebar().unwrap();
            }

            #[cfg(target_os = "macos")]
            {
                use tauri_plugin_decorum::WebviewWindowExt;
                let main_window = app.get_webview_window("main").unwrap();
                main_window.set_traffic_lights_inset(16.0, 20.0).unwrap();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
