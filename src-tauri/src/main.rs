// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env::consts::OS,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[cfg(unix)]
use std::os::unix::fs::{FileExt, MetadataExt};

#[cfg(target_os = "windows")]
use std::os::windows::fs::{FileExt, MetadataExt};

use mu::{check_dir, Album, Media, Track};
use tauri::{http, Manager};

enum CacheCompareDiff {
    ToAdd { files: Vec<PathBuf> },
    ToRemove { files: Vec<PathBuf> },
    NoDiff,
}

fn compare_caches(prev: Vec<PathBuf>, curr: Vec<PathBuf>) -> (Vec<CacheCompareDiff>, usize, usize) {
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
        return (vec![CacheCompareDiff::NoDiff], 0, 0);
    }

    let to_add_len = files_to_add.len();
    let to_remove_len = files_to_remove.len();

    (
        vec![
            CacheCompareDiff::ToAdd {
                files: files_to_add,
            },
            CacheCompareDiff::ToRemove {
                files: files_to_remove,
            },
        ],
        to_add_len,
        to_remove_len,
    )
}

#[derive(Debug, Clone, serde::Serialize)]
enum CachePayLoad {
    FileProcessed(String),
    TotalFiles { count: usize },
    Ended { media: Media },
}

fn get_chache(app_cache_dir: PathBuf, window: Option<tauri::Window>) -> Media {
    // if let Some(win) = window.clone() {
    //     let _ = win.emit("cache-update-start", ());
    // }

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

    let (diff, to_add_len, to_remove_len) =
        compare_caches(prev_audio_files, curr_audio_files.clone());

    if let Some(win) = window.clone() {
        let _ = win.emit(
            "cache-update-files",
            CachePayLoad::TotalFiles {
                count: to_add_len + to_remove_len,
            },
        );
    }

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
                            if let Some(win) = window.clone() {
                                let _ = win.emit(
                                    "cache-update-data",
                                    CachePayLoad::FileProcessed(format!("+ {}", file.display())),
                                );
                            }
                            cache_data.add_song(Track::from_file(covers_dir.clone(), file));
                        }
                    }
                    CacheCompareDiff::ToRemove { files } => {
                        for file in files {
                            if let Some(win) = window.clone() {
                                let _ = win.emit(
                                    "cache-update-data",
                                    CachePayLoad::FileProcessed(format!("- {}", file.display())),
                                );
                            }
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
        } else {
            eprintln!("[WARN] Unmatched Media cache verison");
            if let Some(win) = window.clone() {
                let _ = win.emit(
                    "cache-update-files",
                    CachePayLoad::TotalFiles {
                        count: curr_audio_files.len(),
                    },
                );
            }
            for file in curr_audio_files {
                if let Some(win) = window.clone() {
                    let _ = win.emit(
                        "cache-update-data",
                        CachePayLoad::FileProcessed(format!("+ {}", file.display())),
                    );
                }

                cache.add_song(Track::from_file(covers_dir.clone(), file));
            }
            needs_update = true;
        }
    } else {
        if let Some(win) = window.clone() {
            let _ = win.emit(
                "cache-update-files",
                CachePayLoad::TotalFiles {
                    count: curr_audio_files.len(),
                },
            );
        }
        for file in curr_audio_files {
            if let Some(win) = window.clone() {
                let _ = win.emit(
                    "cache-update-data",
                    CachePayLoad::FileProcessed(format!("+ {}", file.display())),
                );
            }

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

    if let Some(win) = window.clone() {
        let _ = win.emit(
            "cache-update-end",
            CachePayLoad::Ended {
                media: cache.clone(),
            },
        );
    }

    cache
}

#[tauri::command]
fn update_cache(app: tauri::AppHandle, window: tauri::Window) -> Media {
    let app_cache_dir = app.path().app_cache_dir().unwrap();
    return get_chache(app_cache_dir, Some(window));
}

#[tauri::command]
async fn index(app: tauri::AppHandle, window: tauri::Window) {
    let app_cache_dir = app.path().app_cache_dir().unwrap();
    check_dir(format!("{}", app_cache_dir.display()));
    get_chache(app_cache_dir, Some(window));
}

#[tauri::command]
fn get_album(app: tauri::AppHandle, id: String) -> Option<Album> {
    let app_cache_dir = app.path().app_cache_dir().unwrap();
    let media = get_chache(app_cache_dir, None);
    media.get_album(id)
}

#[tauri::command]
fn platform() -> String {
    OS.to_string()
}

#[tauri::command]
fn locale(app: tauri::AppHandle) -> String {
    let app_cache_dir = app.path().app_cache_dir().unwrap();
    check_dir(format!("{}", app_cache_dir.display()));
    let cache_dir = format!("{}/.locale", app_cache_dir.display());
    mu::utils::get_locale(Path::new(&cache_dir))
}

#[tauri::command]
fn set_locale(app: tauri::AppHandle, locale: String) {
    let app_cache_dir = app.path().app_cache_dir().unwrap();
    let cache_dir = format!("{}/.locale", app_cache_dir.display());
    mu::utils::set_locale(Path::new(&cache_dir), locale)
}

#[derive(Debug)]
enum ReqType {
    Cover(String),
    Audio(String),
}

impl ReqType {
    pub fn parse(data: Vec<&str>) -> Option<Self> {
        let mut iter = data.iter();
        if let Some(kind) = iter.next() {
            match *kind {
                "cover" => {
                    if let Some(cover_file) = iter.next() {
                        return Some(ReqType::Cover(cover_file.to_string()));
                    } else {
                        return None;
                    }
                }
                "audio" => {
                    if let Some(id) = iter.next() {
                        return Some(ReqType::Audio(id.to_string()));
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        } else {
            None
        }
    }
}

fn range(data: (&str, &str)) -> (u64, u64) {
    let (start, end) = data;

    let start = match start {
        "" => 0,
        _ => start.parse::<u64>().unwrap(),
    };

    let end = match end {
        "" => 0,
        _ => end.parse::<u64>().unwrap(),
    };

    (start, end)
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
        .register_asynchronous_uri_scheme_protocol("mu", move |app, request, responder| {
            // let audio_dir = app.path().audio_dir().unwrap();
            let cache_dir = app.path().app_cache_dir().unwrap();
            let req_string = request
                .uri()
                .clone()
                .into_parts()
                .path_and_query
                .unwrap()
                .to_string()[1..]
                .to_string();

            let req = url_escape::decode(&req_string).to_string();
            println!("{req}");

            if let Some(req) = ReqType::parse(req.split(':').collect()) {
                match req {
                    ReqType::Cover(cover_file) => {
                        let cover_path = format!("{}/covers/{}", cache_dir.display(), cover_file);
                        let mut buf = vec![];
                        if let Ok(mut file) = std::fs::File::open(&cover_path) {
                            let _ = file.read_to_end(&mut buf);
                            responder.respond(
                                http::Response::builder()
                                    .status(200)
                                    .header("Access-Control-Allow-Origin", "*")
                                    .body(buf)
                                    .unwrap(),
                            );
                        } else {
                            let buf = include_bytes!("./assets/default-cover.png");
                            responder.respond(
                                http::Response::builder()
                                    .status(200)
                                    .body(buf.to_vec())
                                    .unwrap(),
                            );
                        }
                    }
                    ReqType::Audio(id) => {
                        println!("{id}");
                        let mut resp =
                            http::Response::builder().header("Access-Control-Allow-Origin", "*");
                        let media = get_chache(cache_dir, None);
                        let mut buf: Vec<u8> = vec![];
                        if let Some(song) = media.get_song(id) {
                            let path = Path::new(&song.file_path);
                            if !path.exists() {
                                buf = "File doesn't exist".as_bytes().to_vec();
                                resp = resp.status(404);
                            } else {
                                #[cfg(unix)]
                                let size = fs::metadata(path).unwrap().size();

                                #[cfg(target_os = "windows")]
                                let size = fs::metadata(path).unwrap().file_size();

                                println!("{size}ko");

                                let audio = fs::File::open(path).unwrap();
                                if let Some(r) = request.headers().get("range") {
                                    let (start, mut end): (u64, u64) = r
                                        .to_str()
                                        .unwrap()
                                        .to_lowercase()
                                        .replace(' ', "")
                                        .strip_prefix("bytes=")
                                        .unwrap()
                                        .split_once('-')
                                        .map(range)
                                        .unwrap();
                                    if end > size {
                                        resp = resp.status(416);
                                        resp = resp
                                            .header("Content-Range", &format!("bytes */{size}"));
                                    } else {
                                        if end == 0 {
                                            end = start + 1024 * 32;
                                            if end > size {
                                                end = size
                                            }
                                        }

                                        let reading_size = (end - start) as usize;

                                        let mut buf = vec![0u8; reading_size];

                                        #[cfg(unix)]
                                        let _ = audio.read_at(&mut buf, start);

                                        #[cfg(target_os = "windows")]
                                        let _ = audio.seek_read(&mut buf, start);

                                        resp = resp.status(206);
                                        resp = resp.header(
                                            "Content-Range",
                                            &format!("bytes {start}-{end}/{size}"),
                                        );
                                        resp = resp.header("Accept-Ranges", "bytes");
                                        resp = resp.header("Content-Type", song.mime.as_str());
                                    }
                                } else {
                                    resp = resp.status(416);
                                    resp = resp.header("Content-Range", &format!("bytes */{size}"));
                                }
                            }
                        } else {
                            buf = "Song not found".as_bytes().to_vec();
                            resp = resp.status(404);
                        }

                        responder.respond(resp.body(buf).unwrap());
                    }
                }
            } else {
                responder.respond(
                    http::Response::builder()
                        .status(404)
                        .body(Vec::new())
                        .unwrap(),
                );
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
