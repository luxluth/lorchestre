use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::daemon::global::utils::cache_audio_files;
use crate::daemon::global::utils::get_audio_files;
use crate::daemon::global::utils::read_cache_audio_files;
use crate::daemon::global::Media;
use tauri::Emitter;
use tracing::{info, warn};

pub enum CacheCompareDiff {
    ToAdd { files: Vec<PathBuf> },
    ToRemove { files: Vec<PathBuf> },
    NoDiff,
}

pub async fn cache_resolve(cache_dir: &Path, win: Option<tauri::Window>) -> Media {
    info!("Starting cache process...");
    let p_string = cache_dir.join(".cache");
    let covers_dir = cache_dir.join("covers");
    let ac_string = cache_dir.join(".cache.list");
    let ac_path = Path::new(&ac_string);

    let prev_audio_files = read_cache_audio_files(ac_path);
    let curr_audio_files = get_audio_files();
    cache_audio_files(ac_path);

    let (diff, _, _) = compare_caches(prev_audio_files, curr_audio_files.clone());

    let mut cache = Media::default();
    let cache_file = Path::new(&p_string);
    let mut needs_update = true;

    if cache_file.exists() {
        let mut f = fs::File::open(cache_file).unwrap();
        let mut buf = Vec::new();
        let _ = f.read_to_end(&mut buf);
        if let Ok(mut cache_data) = bitcode::decode::<Media>(&buf) {
            'f: for d in diff {
                match d {
                    CacheCompareDiff::ToAdd { files } => {
                        for file in files {
                            let msg = format!("+ {}", file.display());
                            info!(msg);
                            if let Some(win) = win.clone() {
                                let _ = win.emit("sync", msg);
                            }
                            cache_data.add_media(file, &covers_dir);
                        }
                    }
                    CacheCompareDiff::ToRemove { files } => {
                        for file in files {
                            let msg = format!("- {}", file.display());
                            info!(msg);
                            if let Some(win) = win.clone() {
                                let _ = win.emit("sync", msg);
                            }
                            cache_data.remove_media(file);
                        }
                    }
                    CacheCompareDiff::NoDiff => {
                        needs_update = false;
                        info!("~ No cache change");
                        break 'f;
                    }
                }
            }

            cache = cache_data;
        } else {
            warn!("[WARN] Unmatched Media cache verison");
            for file in curr_audio_files {
                let msg = format!("+ {}", file.display());
                info!(msg);
                if let Some(win) = win.clone() {
                    let _ = win.emit("sync", msg);
                }

                cache.add_media(file, &covers_dir);
            }
            needs_update = true;
        }
    } else {
        for file in curr_audio_files {
            let msg = format!("+ {}", file.display());
            info!(msg);
            if let Some(win) = win.clone() {
                let _ = win.emit("sync", msg);
            }

            info!("+ {}", file.display());
            cache.add_media(file, &covers_dir);
        }
        needs_update = true;
    }

    if needs_update {
        info!("* cache updated");
        let bin = bitcode::encode(&cache);
        let mut f = fs::File::create(&p_string).unwrap();
        let _ = f.write_all(&bin);
    }

    info!("cache process ended");

    cache.create_search_index(cache_dir.to_path_buf());

    cache
}

pub fn compare_caches(
    prev: Vec<PathBuf>,
    curr: Vec<PathBuf>,
) -> (Vec<CacheCompareDiff>, usize, usize) {
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
            CacheCompareDiff::ToRemove {
                files: files_to_remove,
            },
            CacheCompareDiff::ToAdd {
                files: files_to_add,
            },
        ],
        to_add_len,
        to_remove_len,
    )
}
