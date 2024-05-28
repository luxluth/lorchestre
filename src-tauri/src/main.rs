// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env::consts::OS,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use mu::{Album, Media, MediaCache, Songs};
use tauri::Manager;

fn get_chache(app_cache_dir: PathBuf) -> MediaCache {
    if !app_cache_dir.exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .create(format!("{}", app_cache_dir.display()))
            .unwrap();
    }

    let p_string = format!("{}/.cache.json", app_cache_dir.display());
    let mut needs_update = false;

    let mut cache: MediaCache = MediaCache::default();
    let cache_file = Path::new(&p_string);
    let current_md5 = mu::utils::music_dir_md5();
    if cache_file.exists() {
        let mut f = fs::File::open(cache_file).unwrap();
        let mut buf = String::new();
        let _ = f.read_to_string(&mut buf);
        if let Ok(cache_data) = serde_json::from_str::<MediaCache>(&buf) {
            if cache_data.md5 != current_md5 {
                needs_update = true
            } else {
                eprintln!("[INFO] __update_cache: cache hit");
                cache = cache_data;
            }
        } else {
            needs_update = true;
        }
    } else {
        needs_update = true;
    }

    if needs_update {
        eprintln!("[INFO] __update_cache: cache miss");
        let songs = Songs::new(app_cache_dir.clone());
        cache = MediaCache {
            media: Media::new(songs),
            md5: current_md5,
        };
        let jason = serde_json::to_string(&cache).unwrap();
        let mut f = fs::File::create(format!("{}/.cache.json", app_cache_dir.display())).unwrap();
        let _ = f.write_all(jason.as_bytes());
    }

    cache
}

#[tauri::command]
fn update_cache(app: tauri::AppHandle) -> MediaCache {
    let app_cache_dir = app.path().app_cache_dir().unwrap();
    return get_chache(app_cache_dir);
}

#[tauri::command]
fn index(app: tauri::AppHandle) -> Media {
    let app_cache_dir = app.path().app_cache_dir().unwrap();
    let cache = get_chache(app_cache_dir);
    return cache.media;
}

#[tauri::command]
fn get_album(app: tauri::AppHandle, id: String) -> Option<Album> {
    let app_cache_dir = app.path().app_cache_dir().unwrap();
    let cache = get_chache(app_cache_dir);
    cache.media.get_album(id)
}

#[tauri::command]
fn platform() -> String {
    OS.to_string()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            index,
            update_cache,
            get_album,
            platform
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
