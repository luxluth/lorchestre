// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use daemon::{config::Dir, global::Media};
use lorconf::Config;
use tracing::error;
mod daemon;
use crate::daemon::entry::start;
use std::{env::consts::OS, fs::File, io::Write};
use tauri::{Emitter, Manager};
#[cfg(target_os = "macos")]
use tauri_plugin_decorum::WebviewWindowExt;
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};
use tracing_subscriber::FmtSubscriber;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tauri::command]
fn platform() -> String {
    OS.to_string()
}

#[cfg(target_os = "linux")]
#[tauri::command]
fn desktop() -> String {
    std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or("UNKNOWN".to_string())
        .to_lowercase()
}

#[cfg(target_os = "linux")]
#[tauri::command]
fn gnome_window_controls() -> Vec<String> {
    let cmd = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.wm.preferences", "button-layout"])
        .output()
        .unwrap();

    String::from_utf8(cmd.stdout)
        .unwrap()
        .replace('\'', "")
        .replace("appmenu:", "")
        .split(',')
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect()
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
fn save_lyrics(input: String, path: String) {
    let lrc_path = std::path::PathBuf::from(path).with_extension("lrc");
    let mut lrc_file = File::create(lrc_path).unwrap();
    let _ = lrc_file.write_all(input.as_bytes());
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
fn start_daemon(window: tauri::Window, app: tauri::AppHandle) {
    tokio::task::spawn(async move {
        let dirs = Dir {
            config: app.path().app_config_dir().unwrap(),
            cache: app.path().app_cache_dir().unwrap(),
        };
        let _ = start(Some(window), dirs).await;
    });
}

#[tauri::command]
fn cache_media(app: tauri::AppHandle, window: tauri::Window, media: Media) {
    media.cache(app.path().app_cache_dir().unwrap(), Some(window));
}

#[derive(Debug, serde::Serialize)]
struct AppInfoExternal {
    first_run: bool,
}

#[tauri::command]
fn close(app: tauri::AppHandle) {
    let _ = app.save_window_state(StateFlags::all());
    std::process::exit(0x00);
}

#[derive(Clone, serde::Serialize)]
struct SingleInstanceArgs {
    args: Vec<String>,
    cwd: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_decorum::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            error!(
                "An instance of l'orchestre is already running :: {}, {argv:?}, {cwd}",
                app.package_info().name
            );
            app.emit("single-instance", SingleInstanceArgs { args: argv, cwd })
                .unwrap();
        }))
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
            start_daemon,
            save_lyrics,
            close,
            cache_media,
            #[cfg(target_os = "linux")]
            desktop,
            #[cfg(target_os = "linux")]
            gnome_window_controls,
        ])
        .setup(move |app| {
            let is_first_run = !app
                .path()
                .app_cache_dir()
                .unwrap()
                .join("__runned")
                .exists();

            let dirs = Dir {
                config: app.path().app_config_dir().unwrap(),
                cache: app.path().app_cache_dir().unwrap(),
            };

            tokio::task::spawn(async move {
                if !is_first_run {
                    let _ = start(None, dirs).await;
                }
            });

            let window = app.get_webview_window("main").unwrap();
            let _ = window.restore_state(StateFlags::all());

            #[cfg(target_os = "macos")]
            {
                window.set_traffic_lights_inset(12.0, 16.0).unwrap();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
