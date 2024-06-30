// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lorconf::Config;
mod args;
mod daemon;
use crate::daemon::entry::start;
use clap::Parser;
use std::{env::consts::OS, io::Write};
use tauri::Manager;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(target_os = "linux")]
const LINUX_SYSTEMD: &str = include_str!("./systemd/lorchestre.service");
#[cfg(target_os = "macos")]
const MACOS_LAUNCHD: &str = include_str!("./launchd/dev.luxluth.lorchestre.plist");

#[cfg(target_os = "linux")]
fn init_service(bin_path: String) -> std::io::Result<()> {
    let service_dir = dirs::config_dir().unwrap().join("systemd/user");

    if !service_dir.exists() {
        std::fs::DirBuilder::new()
            .recursive(true)
            .create(&service_dir)
            .unwrap();
    }

    let service_path = service_dir.join("lorchestre.service");
    let mut file = std::fs::File::create(&service_path)?;
    let service_content = LINUX_SYSTEMD.replace("{{BIN_PATH}}", &bin_path);
    file.write_all(service_content.as_bytes())?;
    info!(
        "Systemd service file created at: {}",
        service_path.display()
    );

    // Reload systemd and start the service
    std::process::Command::new("systemctl")
        .arg("--user")
        .arg("daemon-reload")
        .output()?;
    std::process::Command::new("systemctl")
        .arg("--user")
        .arg("enable")
        .arg("lorchestre.service")
        .output()?;
    std::process::Command::new("systemctl")
        .arg("--user")
        .arg("start")
        .arg("lorchestre.service")
        .output()?;
    info!("Systemd service enabled and started.");
    Ok(())
}

#[cfg(target_os = "macos")]
fn init_service(bin_path: String) -> std::io::Result<()> {
    let plist_dir = dirs::home_dir().unwrap().join("/Library/LaunchAgents/");

    if !plist_dir.exists() {
        std::fs::DirBuilder::new()
            .recursive(true)
            .create(&plist_dir)
            .unwrap();
    }

    let plist_path = plist_dir.join("dev.luxluth.lorchestre.plist");
    let mut file = std::fs::File::create(&plist_path)?;
    let plist_content = MACOS_LAUNCHD.replace("{{BIN_PATH}}", &bin_path);
    file.write_all(plist_content.as_bytes())?;
    info!("Launchd service file created at: {}", plist_path.display());

    // Load the launchd service
    std::process::Command::new("launchctl")
        .arg("load")
        .arg(&plist_path)
        .output()?;

    // start the launchd service
    std::process::Command::new("launchctl")
        .arg("start")
        .arg("dev.luxluth.lorchestre")
        .output()?;
    info!("Launchd service loaded.");
    Ok(())
}

#[cfg(target_os = "windows")]
fn init_service(bin_path: String) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Need investigation

    std::process::Command::new(&bin_path)
        .arg("daemon")
        .spawn()?;
    Ok(())
}

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
fn start_service(
    window: tauri::Window,
    app_info: tauri::State<AppInfo>,
) -> std::result::Result<(), ()> {
    let clone_path = app_info.bin_path.clone();
    std::thread::spawn(move || {
        let bin_path = clone_path;
        info!("BIN_PATH: {bin_path}");
        if init_service(bin_path.clone()).is_ok() {
            let _ = window.emit("daemon-ok", ());
        } else {
            let _ = window.emit("daemon-error", ());
        }
    });

    Ok(())
}

#[derive(Debug, serde::Serialize)]
struct AppInfoExternal {
    first_run: bool,
}

struct AppInfo {
    bin_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    let program = std::env::args().next().expect("You're os is weird");
    let args_vec: Vec<String> = std::env::args().collect();
    let args = args::LorArgs::parse();
    info!("args: {:?}", args_vec);
    if args.entity.is_some() {
        start().await?;
        Ok(())
    } else {
        tauri::Builder::default()
            .plugin(tauri_plugin_shell::init())
            .manage(AppInfo {
                bin_path: program.clone(),
            })
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
                start_service
            ])
            .setup(move |_| {
                #[cfg(target_os = "windows")]
                {
                    init_service(program.clone());
                }

                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
        Ok(())
    }
}
