use std::{
    collections::HashMap,
    path::PathBuf,
    time::{Duration, SystemTime},
};

use glob::glob;
use lofty::{
    file::{AudioFile, TaggedFileExt},
    probe::Probe,
    tag::{Accessor, ItemKey},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Id {
    Digest(md5::Digest),
    Number(usize),
}

#[derive(Debug)]
pub struct Artist {
    pub id: Id,
    pub name: String,
}

#[derive(Debug)]
pub struct Song {
    pub id: Id,
    pub title: String,
    pub file_path: PathBuf,
    pub artists: Vec<Id>,
    pub track: u32,
    pub album: Option<Id>,
    pub duration: Duration,
    pub bitrate: u32,
    pub encoder: String,

    pub created_at: u64,
}

impl Song {
    fn new(id: Id, file_path: PathBuf) -> Self {
        Self {
            id,
            title: String::new(),
            file_path,
            artists: vec![],
            track: 0,
            album: None,
            duration: Duration::default(),
            bitrate: 0,
            encoder: String::new(),
            created_at: 0,
        }
    }
}

#[derive(Debug)]
pub struct Album {
    pub id: usize,
    pub genres: Vec<String>,
    pub artist: Option<Id>,
    pub year: Option<u32>,
    pub songs: Vec<Id>,
    pub disc_total: u32,
    pub songs_count: u32,
}

#[derive(Default)]
struct IdStore {
    current: usize,
}

impl IdStore {
    pub fn next(&mut self) -> Id {
        self.current += 1;
        return Id::Number(self.current);
    }

    pub fn digest<T: AsRef<[u8]>>(&self, data: T) -> Id {
        return Id::Digest(md5::compute(data));
    }
}

#[derive(Default)]
pub struct MusicCollection {
    pub artists: HashMap<Id, Artist>,
    pub albums: HashMap<Id, Album>,
    pub songs: HashMap<Id, Song>,

    artist_id_store: IdStore,
    album_id_store: IdStore,
    song_id_store: IdStore,
}

impl MusicCollection {
    pub fn index_file(&mut self, file_path: PathBuf) {
        let id = self
            .song_id_store
            .digest(format!("{}", file_path.to_str().unwrap()).as_bytes());

        if let Ok(tagged_file) = Probe::open(&file_path).unwrap().read() {
            let properties = tagged_file.properties();
            let bitrate = properties.audio_bitrate().unwrap_or(0);
            let duration = properties.duration();
            // let mime = tagged_file.file_type();

            let default_tag = lofty::tag::Tag::new(lofty::tag::TagType::Id3v2);
            let tag = match tagged_file.primary_tag() {
                Some(primary_tag) => primary_tag,
                // If the "primary" tag doesn't exist, we just grab the
                // first tag we can find. Realistically, a tag reader would likely
                // iterate through the tags to find a suitable one.
                None => tagged_file.first_tag().unwrap_or(&default_tag),
            };

            let mut audio = Song::new(id, file_path.clone());
            audio.bitrate = bitrate;
            audio.duration = duration;

            if let Ok(meta) = file_path.metadata() {
                if let Ok(tm) = meta.created() {
                    audio.created_at = tm
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or(Duration::default())
                        .as_secs();
                }
            };

            if let Some(encoder) = tag.get_string(&ItemKey::EncoderSettings) {
                audio.encoder = encoder.to_string();
            }

            if let Some(title) = tag.title() {
                audio.title = title.to_string();
            }
        }
    }

    pub fn index(&mut self, dir_path: PathBuf) {
        if let Ok(paths) = glob(&format!("{}/**/*", dir_path.display())) {
            for inode in paths.flatten() {
                if inode.is_file() {
                    let guess =
                        mime_guess::from_path(&inode).first_or("text/plain".parse().unwrap());
                    if guess.type_() == mime_guess::mime::AUDIO {
                        self.index_file(inode);
                    }
                }
            }
        }
    }
}
