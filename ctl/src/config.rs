use std::{fs, path::PathBuf};
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_ID: &str = "lorchestre";

#[derive(Debug, Clone)]
pub struct Dir {
    pub config: PathBuf,
    pub app: PathBuf,
    pub cache: PathBuf,
    // pub audio: PathBuf,
}

pub fn get_dirs() -> Dir {
    let dir = Dir {
        config: dirs::config_dir().unwrap().join(APP_ID),
        app: dirs::data_local_dir().unwrap().join(APP_ID),
        cache: dirs::cache_dir().unwrap().join(APP_ID),
        // audio: dirs::audio_dir().unwrap(),
    };

    if !dir.config.exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .create(&dir.config)
            .unwrap();
    }

    if !dir.app.exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .create(&dir.app)
            .unwrap();
    }

    if !dir.cache.exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .create(&dir.cache)
            .unwrap();
    }

    return dir;
}
