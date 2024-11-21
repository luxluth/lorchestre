use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use bitcode::{Decode, Encode};
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

pub type PlaylistMetadata = HashMap<String, String>;

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone, Encode, Decode)]
pub struct PlaylistData {
    pub metadata: PlaylistMetadata,
    pub tracks: Vec<String>,
    pub path: String,
    pub path_base64: String,
}

impl PlaylistData {
    pub fn update(&mut self, meta: PlaylistMetadata, tracks: Vec<String>) -> io::Result<()> {
        for (k, v) in meta {
            self.metadata.insert(k, v);
        }

        self.tracks = tracks;
        self.save(PathBuf::from(&self.path))
    }

    pub fn create<P>(
        audio_dir: P,
        metadata: PlaylistMetadata,
        tracks: Vec<String>,
    ) -> io::Result<String>
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

        let base64_path = list.path_base64.clone();

        list.save(list_path)?;

        Ok(base64_path)
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
                } else {
                    tracks.push(PathBuf::from_str(line).unwrap());
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
