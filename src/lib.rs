use std::{fs::DirBuilder, path::PathBuf};

pub mod di;
pub mod track;
pub mod ui;

pub struct Utils;

impl Utils {
    pub fn cache_dir() -> PathBuf {
        let cache_dir = dirs::cache_dir().unwrap().join("lorchestre");
        if !cache_dir.exists() {
            let _ = DirBuilder::new()
                .recursive(true)
                .create(cache_dir.as_path());
        }

        cache_dir
    }

    pub fn cache_path() -> PathBuf {
        Utils::cache_dir().join("_index")
    }

    pub fn covers_dir() -> PathBuf {
        let store = Utils::cache_dir().join("__COVERS_STORE");
        if !store.exists() {
            let _ = DirBuilder::new().recursive(true).create(store.as_path());
        }

        store
    }
}
