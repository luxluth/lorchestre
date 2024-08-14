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
    pub tracks: Vec<PathBuf>,
    pub path: String,
    pub id: String,
}

pub type PlaylistMetadata = HashMap<String, String>;

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct PlaylistData {
    pub metadata: PlaylistMetadata,
    pub tracks: Vec<PathBuf>,
    pub path: String,
    pub id: String,
}

pub enum PlaylistAction {
    RemoveTrack(PathBuf),
    AddTrack(PathBuf),
    UpdateOrder(Vec<PathBuf>),
    RemoveMeta(String),
    AddMeta(String, String),
}

impl PlaylistData {
    pub fn update(&mut self, action: PlaylistAction) -> io::Result<()> {
        match action {
            PlaylistAction::RemoveTrack(track) => {
                self.tracks.retain(|p| *p != track);
            }
            PlaylistAction::AddTrack(track) => {
                self.tracks.push(track);
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

    pub fn delete(&self) -> io::Result<()> {
        let path = Path::new(&self.path);
        if path.exists() {
            std::fs::remove_file(path)?;
        }

        Ok(())
    }
}

impl PlaylistData {
    pub fn parse(path: PathBuf) -> Self {
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
                } else {
                    if let Ok(path) = PathBuf::from_str(line) {
                        tracks.push(path)
                    };
                }
            }
        }

        let data = format!("{}", path.display());
        let id = md5::compute(data);

        Self {
            metadata,
            path: format!("{}", path.display()),
            tracks,
            id: format!("{id:x}"),
        }
    }

    pub fn get_name(&self) -> String {
        if let Some(name) = self.metadata.get("Name") {
            name.clone()
        } else {
            "@UNKNOWN@".to_string()
        }
    }

    pub fn from_m3u8_playlist(p: M3u8Playlist) -> Self {
        let mut metadata = PlaylistMetadata::new();
        metadata.insert("Name".to_string(), p.name.clone());

        Self {
            metadata,
            tracks: p.tracks,
            path: p.path,
            id: p.id,
        }
    }

    pub fn save(&self, path: PathBuf) -> io::Result<()> {
        let mut output = Vec::new();
        for (meta_k, meta_v) in &self.metadata {
            writeln!(&mut output, "{}: {}", meta_k, meta_v)?;
        }

        writeln!(&mut output)?;

        for file in &self.tracks {
            writeln!(&mut output, "{}", file.display())?;
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

        let mut playlist = M3u8Playlist {
            name: name.to_string(),
            path: format!("{}", path.display()),
            tracks: text
                .lines()
                .filter(|x| !x.is_empty() && !x.starts_with('#'))
                .map(|x| Path::new(x).to_path_buf())
                .filter(|p| p.exists())
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

        let mut f = File::create(path.with_extension("m3u8.bak")).unwrap();
        let _ = f.write(&text.as_bytes());

        let playlist = PlaylistData::from_m3u8_playlist(playlist);
        if playlist.save(path.with_extension("playlist")).is_ok() {
            let _ = std::fs::remove_file(&path);
        }

        playlist
    }
}
