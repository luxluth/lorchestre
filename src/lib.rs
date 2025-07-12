use std::{fs::DirBuilder, path::PathBuf};

// TODO: remove pubs
pub mod di;
pub mod platform;
pub mod track;

pub use platform::frontend::Lorchestre;

pub struct Lorch;

impl Lorch {
    pub fn cache_dir() -> PathBuf {
        let cache_dir = dirs::cache_dir().unwrap().join("lorchestre");
        if !cache_dir.exists() {
            let _ = DirBuilder::new()
                .recursive(true)
                .create(cache_dir.as_path());
        }

        cache_dir
    }

    pub fn covers_dir() -> PathBuf {
        let store = Lorch::cache_dir().join("__COVERS_STORE");
        if !store.exists() {
            let _ = DirBuilder::new().recursive(true).create(store.as_path());
        }

        store
    }
}
