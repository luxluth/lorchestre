use crate::Track;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

pub struct M3U8;

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct Playlist {
    pub name: String,
    pub tracks: Vec<Track>,
    pub path: String,
    pub id: String,
}

impl M3U8 {
    pub fn parse(covers_dir: String, path: PathBuf) -> Playlist {
        let p = path.clone();
        let name = p.file_stem().unwrap().to_str().unwrap_or("@UNKNOWN@");
        let mut text = String::new();
        let mut f = File::open(path.clone()).unwrap();
        let _ = f.read_to_string(&mut text);

        let mut playlist = Playlist {
            name: name.to_string(),
            path: format!("{}", path.display()),
            tracks: text
                .lines()
                .filter(|x| !x.is_empty() && !x.starts_with('#'))
                .map(|x| Path::new(x).to_path_buf())
                .filter(|p| p.exists())
                .map(|inode| Track::from_file(covers_dir.clone(), inode))
                .collect(),
            id: String::new(),
        };

        let data = format!(
            "{}{}#{}",
            playlist.name,
            playlist.path,
            playlist.tracks.len()
        );

        let id = md5::compute(data);
        playlist.id = format!("{id:x}");

        playlist
    }
}
