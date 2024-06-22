use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use mud::Media;
use tracing::{info, warn};

pub enum CacheCompareDiff {
    ToAdd { files: Vec<PathBuf> },
    ToRemove { files: Vec<PathBuf> },
    NoDiff,
}

pub async fn cache_resolve(cache_dir: &PathBuf) -> Media {
    info!("Starting cache process...");
    let p_string = cache_dir.join(".cache.json");
    let covers_dir = cache_dir.join("covers");
    let ac_string = cache_dir.join(".cache.list");
    let ac_path = Path::new(&ac_string);

    let prev_audio_files = mud::utils::read_cahe_audio_files(ac_path);
    let curr_audio_files = mud::utils::get_audio_files();
    mud::utils::cache_audio_files(ac_path);

    let (diff, _, _) = compare_caches(prev_audio_files, curr_audio_files.clone());

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
                            info!("+ {}", file.display().to_string());
                            cache_data.add_media(file, &covers_dir);
                        }
                    }
                    CacheCompareDiff::ToRemove { files } => {
                        for file in files {
                            info!("- {}", file.display().to_string());
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
                info!("+ {}", file.display().to_string());
                cache.add_media(file, &covers_dir);
            }
            needs_update = true;
        }
    } else {
        for file in curr_audio_files {
            info!("+ {}", file.display().to_string());
            cache.add_media(file, &covers_dir);
        }
        needs_update = true;
    }

    if needs_update {
        info!("* cache updated");
        let jason = serde_json::to_string(&cache).unwrap();
        let mut f = fs::File::create(&p_string).unwrap();
        let _ = f.write_all(jason.as_bytes());
    }

    info!("cache process ended");

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
