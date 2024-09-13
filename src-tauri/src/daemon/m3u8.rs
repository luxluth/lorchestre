use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use bitcode::{Decode, Encode};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

pub struct M3U8;

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct M3u8Playlist {
    pub name: String,
    pub tracks: Vec<String>,
    pub path: String,
}

pub type PlaylistMetadata = HashMap<String, String>;

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone, Encode, Decode)]
pub struct PlaylistData {
    pub metadata: PlaylistMetadata,
    pub tracks: Vec<String>,
    pub path: String,
    pub path_base64: String,
}

pub enum PlaylistAction {
    RemoveTracks(Vec<String>),
    AddTracks(Vec<String>),
    UpdateOrder(Vec<String>),
    RemoveMeta(String),
    AddMeta(String, String),
}

impl PlaylistData {
    pub fn update(&mut self, action: PlaylistAction) -> io::Result<()> {
        match action {
            PlaylistAction::RemoveTracks(tracks) => {
                for track in tracks {
                    self.tracks.retain(|p| *p != track);
                }
            }
            PlaylistAction::AddTracks(track) => {
                self.tracks.extend(track);
            }
            PlaylistAction::UpdateOrder(tracks) => {
                self.tracks = tracks;
            }
            PlaylistAction::RemoveMeta(k) => {
                self.metadata.remove_entry(&k);
            }
            PlaylistAction::AddMeta(k, v) => {
                self.metadata.insert(k, v);
            }
        }

        self.save(PathBuf::from(&self.path))
    }

    pub fn create<P>(
        audio_dir: P,
        metadata: PlaylistMetadata,
        tracks: Vec<String>,
    ) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let playlists_dir = audio_dir.as_ref().join("Playlists");
        if !playlists_dir.exists() {
            std::fs::DirBuilder::new()
                .recursive(true)
                .create(&playlists_dir)
                .unwrap();
        }
        let list_path = playlists_dir.join(format!("{}.playlist", uuid::Uuid::new_v4()));
        let list = PlaylistData {
            metadata,
            tracks,
            path_base64: URL_SAFE.encode(format!("{}", list_path.display()).as_bytes()),
            path: format!("{}", list_path.display()),
        };

        list.save(list_path)?;

        Ok(())
    }

    pub fn delete(&self) -> io::Result<()> {
        let path = Path::new(&self.path);
        if path.exists() {
            std::fs::remove_file(path)?;
        }

        Ok(())
    }
}

impl PlaylistData {
    pub fn parse(path: String) -> Self {
        let mut f = std::fs::File::open(&path).unwrap();
        let mut input = String::new();
        let _ = f.read_to_string(&mut input);

        let mut metadata = PlaylistMetadata::new();
        let mut tracks: Vec<PathBuf> = vec![];
        for line in input.lines() {
            let line = line.trim();
            if !line.is_empty() {
                if let Some((left, right)) = line.split_once(':') {
                    metadata.insert(left.trim().to_string(), right.trim().to_string());
                } else if let Ok(path) = PathBuf::from_str(line) {
                    tracks.push(path)
                }
            }
        }

        let s_path = path.to_string();

        Self {
            metadata,
            path_base64: URL_SAFE.encode(s_path.as_bytes()),
            path,
            tracks: tracks.iter().map(|p| format!("{}", p.display())).collect(),
        }
    }

    pub fn from_m3u8_playlist(p: M3u8Playlist) -> Self {
        let mut metadata = PlaylistMetadata::new();
        metadata.insert("Name".to_string(), p.name.clone());

        Self {
            metadata,
            tracks: p.tracks,
            path_base64: URL_SAFE.encode(p.path.as_bytes()),
            path: p.path,
        }
    }

    pub fn save(&self, path: PathBuf) -> io::Result<()> {
        let mut output = Vec::new();
        for (meta_k, meta_v) in &self.metadata {
            writeln!(&mut output, "{}: {}", meta_k, meta_v)?;
        }

        writeln!(&mut output)?;

        for file in &self.tracks {
            writeln!(&mut output, "{}", file)?;
        }

        let mut f = std::fs::File::create(path)?;
        f.write_all(&output)?;

        Ok(())
    }
}

impl M3U8 {
    pub fn parse(path: PathBuf) -> PlaylistData {
        let p = path.clone();
        let name = p.file_stem().unwrap().to_str().unwrap_or("@UNKNOWN@");
        let mut text = String::new();
        let mut f = File::open(path.clone()).unwrap();
        let _ = f.read_to_string(&mut text);

        let playlist = M3u8Playlist {
            name: name.to_string(),
            path: format!("{}", path.display()),
            tracks: text
                .lines()
                .filter(|x| !x.is_empty() && !x.starts_with('#'))
                .map(|x| Path::new(x).to_path_buf())
                .filter(|p| p.exists())
                .map(|x| format!("{}", x.display()))
                .collect(),
        };

        let mut f = File::create(path.with_extension("m3u8.bak")).unwrap();
        let _ = f.write(text.as_bytes());

        let playlist = PlaylistData::from_m3u8_playlist(playlist);
        if playlist.save(path.with_extension("playlist")).is_ok() {
            let _ = std::fs::remove_file(&path);
        }

        playlist
    }
}
