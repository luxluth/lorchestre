use std::{fs, path::PathBuf};
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const _APP_ID: &str = "lorchestre";

#[derive(Debug, Clone)]
pub struct Dir {
    pub config: PathBuf,
    pub cache: PathBuf,
    // pub audio: PathBuf,
}

impl Dir {
    pub fn check(&self) {
        if !self.config.exists() {
            fs::DirBuilder::new()
                .recursive(true)
                .create(&self.config)
                .unwrap();
        }

        if !self.cache.exists() {
            fs::DirBuilder::new()
                .recursive(true)
                .create(&self.cache)
                .unwrap();
        }
    }
}
