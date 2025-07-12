#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux as frontend;

use crate::track::MusicCollection;

pub trait Frontend {
    fn init(collection: MusicCollection) -> Self;
    fn start(self) -> std::process::ExitCode;
}
