// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env::consts::OS,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use mu::{Album, Media, Track};
use tauri::Manager;

enum CacheCompareDiff {
    ToAdd { files: Vec<PathBuf> },
    ToRemove { files: Vec<PathBuf> },
    NoDiff,
}

fn compare_caches(prev: Vec<PathBuf>, curr: Vec<PathBuf>) -> Vec<CacheCompareDiff> {
    let mut files_to_add = vec![];
    let mut files_to_remove = vec![];

    for file in prev.clone().into_iter() {
        if !curr.contains(&file) {
            files_to_remove.push(file.clone());
        }
    }

    for file in curr {
        if !prev.contains(&file) {
            files_to_add.push(file.clone());
        }
    }

    if files_to_add.is_empty() && files_to_remove.is_empty() {
        return vec![CacheCompareDiff::NoDiff];
    }

    vec![
        CacheCompareDiff::ToAdd {
            files: files_to_add,
        },
        CacheCompareDiff::ToRemove {
            files: files_to_remove,
        },
    ]
}

fn get_chache(app_cache_dir: PathBuf) -> Media {
    if !app_cache_dir.exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .create(format!("{}", app_cache_dir.display()))
            .unwrap();
    }

    let p_string = format!("{}/.cache.json", app_cache_dir.display());
    let covers_dir = format!("{}/covers", app_cache_dir.display());
    let ac_string = format!("{}/.cache.audios", app_cache_dir.display());
    let ac_path = Path::new(&ac_string);

    let prev_audio_files = mu::utils::read_cahe_audio_files(ac_path);
    let curr_audio_files = mu::utils::get_audio_files();
    mu::utils::cache_audio_files(ac_path);

    let diff = compare_caches(prev_audio_files, curr_audio_files.clone());

    let mut cache = Media::default();
    let cache_file = Path::new(&p_string);
    let mut needs_update = true;

    if cache_file.exists() {
        let mut f = fs::File::open(cache_file).unwrap();
        let mut buf = String::new();
        let _ = f.read_to_string(&mut buf);
        if let Ok(mut cache_data) = serde_json::from_str::<Media>(&buf) {
            'f: for d in diff {
                match d {
                    CacheCompareDiff::ToAdd { files } => {
                        for file in files {
                            eprintln!("+ {}", file.display());
                            cache_data.add_song(Track::from_file(covers_dir.clone(), file));
                        }
                    }
                    CacheCompareDiff::ToRemove { files } => {
                        for file in files {
                            eprintln!("- {}", file.display());
                            cache_data.remove_song(file);
                        }
                    }
                    CacheCompareDiff::NoDiff => {
                        needs_update = false;
                        eprintln!("(~) cache");
                        break 'f;
                    }
                }
            }

            cache = cache_data;
        }
    } else {
        for file in curr_audio_files {
            eprintln!("+ {}", file.display());
            cache.add_song(Track::from_file(covers_dir.clone(), file));
        }
        needs_update = true;
    }

    if needs_update {
        eprintln!("(!) cache");
        let jason = serde_json::to_string(&cache).unwrap();
        let mut f = fs::File::create(format!("{}/.cache.json", app_cache_dir.display())).unwrap();
        let _ = f.write_all(jason.as_bytes());
    }

    cache
}

#[tauri::command]
fn update_cache(app: tauri::AppHandle) -> Media {
    let app_cache_dir = app.path().app_cache_dir().unwrap();
    return get_chache(app_cache_dir);
}

#[tauri::command]
fn index(app: tauri::AppHandle) -> Media {
    let app_cache_dir = app.path().app_cache_dir().unwrap();
    let cache = get_chache(app_cache_dir);
    return cache;
}

#[tauri::command]
fn get_album(app: tauri::AppHandle, id: String) -> Option<Album> {
    let app_cache_dir = app.path().app_cache_dir().unwrap();
    let media = get_chache(app_cache_dir);
    media.get_album(id)
}

#[tauri::command]
fn platform() -> String {
    OS.to_string()
}

#[tauri::command]
fn locale(app: tauri::AppHandle) -> String {
    let app_cache_dir = app.path().app_cache_dir().unwrap();
    let cache_dir = format!("{}/.locale", app_cache_dir.display());
    mu::utils::get_locale(Path::new(&cache_dir))
}

#[tauri::command]
fn set_locale(app: tauri::AppHandle, locale: String) {
    let app_cache_dir = app.path().app_cache_dir().unwrap();
    let cache_dir = format!("{}/.locale", app_cache_dir.display());
    mu::utils::set_locale(Path::new(&cache_dir), locale)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            index,
            update_cache,
            get_album,
            platform,
            locale,
            set_locale,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
