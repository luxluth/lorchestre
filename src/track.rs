use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    path::PathBuf,
    time::Duration,
};

use bincode::{Decode, Encode};
use lofty::picture::MimeType;

use crate::{Utils, di::Di};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Default, Decode, Encode)]
pub struct Digest(pub [u8; 16]);

impl Debug for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", md5::Digest(self.0))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default, Decode, Encode)]
pub enum Id {
    Digest(Digest),
    Number(usize),
    #[default]
    Unresolved,
}

impl Id {
    pub fn digest(&self) -> Option<md5::Digest> {
        match self {
            Id::Digest(Digest(data)) => Some(md5::Digest(data.clone())),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Decode, Encode)]
pub struct Artist {
    pub id: Id,
    pub name: String,
}

#[derive(Debug, Decode, Encode)]
pub struct Song {
    pub id: Id,
    pub title: String,
    pub file_path: PathBuf,
    pub artists: Vec<Id>,
    pub track: u32,
    pub disc: u32,
    pub embeded_lyrics: Option<String>,
    pub album: Option<Id>,
    pub duration: Duration,
    pub bitrate: u32,
    pub encoder: String,

    pub created_at: u64,
}

impl Song {
    pub fn new(id: Id, file_path: PathBuf) -> Self {
        Self {
            id,
            title: String::new(),
            file_path,
            artists: vec![],
            track: 0,
            disc: 1,
            embeded_lyrics: None,
            album: None,
            duration: Duration::default(),
            bitrate: 0,
            encoder: String::new(),
            created_at: 0,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default, Encode, Decode)]
pub struct Timestamp {
    pub year: u16,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
}

impl Timestamp {
    pub fn from_lofty(time: lofty::tag::items::Timestamp) -> Self {
        return Self {
            year: time.year,
            month: time.month,
            day: time.day,
            hour: time.hour,
            minute: time.minute,
            second: time.second,
        };
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.year
            .cmp(&other.year)
            .then(self.month.cmp(&other.month))
            .then(self.day.cmp(&other.day))
            .then(self.hour.cmp(&other.hour))
            .then(self.minute.cmp(&other.minute))
            .then(self.second.cmp(&other.second))
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}", self.year)?;

        if let Some(month) = self.month {
            write!(f, "-{:02}", month)?;

            if let Some(day) = self.day {
                write!(f, "-{:02}", day)?;

                if let Some(hour) = self.hour {
                    write!(f, "T{:02}", hour)?;

                    if let Some(minute) = self.minute {
                        write!(f, ":{:02}", minute)?;

                        if let Some(second) = self.second {
                            write!(f, ":{:02}", second)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Decode, Encode, Clone)]
pub struct Album {
    pub id: Id,
    pub name: String,
    pub genres: HashSet<String>,
    pub artist: Option<Id>,
    pub date: Option<Timestamp>,
    pub songs: Vec<Id>,
    pub disc_total: u32,
    pub songs_count: u32,
    pub cover: Option<Id>,
}

impl Album {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            name: String::new(),
            genres: HashSet::new(),
            artist: None,
            date: None,
            songs: vec![],
            disc_total: 0,
            songs_count: 0,
            cover: None,
        }
    }
}

#[derive(Default, Debug, Decode, Encode)]
pub struct Cover {
    pub id: Id,
    pub ext: String,
}

impl Cover {
    pub fn new(id: Id, mime: &MimeType) -> Cover {
        Cover {
            id,
            ext: match mime {
                MimeType::Png => ".png".to_string(),
                MimeType::Jpeg => ".jpeg".to_string(),
                MimeType::Tiff => ".tiff".to_string(),
                MimeType::Bmp => ".bmp".to_string(),
                MimeType::Gif => ".gif".to_string(),
                MimeType::Unknown(o) => format!(".{o}"),
                _ => ".png".to_string(),
            },
        }
    }
    pub fn get_path(&self) -> PathBuf {
        Utils::covers_dir().join(&format!("{:x}{}", self.id.digest().unwrap(), self.ext))
    }
}

#[derive(Default)]
pub struct IdStore {
    current: usize,
}

impl std::fmt::Debug for IdStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IdStore({})", self.current)
    }
}

impl IdStore {
    pub fn next(&mut self) -> Id {
        self.current += 1;
        return Id::Number(self.current);
    }

    pub fn digest<T: AsRef<[u8]>>(&self, data: T) -> Id {
        return Id::Digest(Digest(md5::compute(data).0));
    }
}

#[derive(Default, Decode, Encode, Debug)]
pub struct MusicCollection {
    pub artists: HashMap<Id, Artist>,
    pub albums: HashMap<Id, Album>,
    pub songs: HashMap<Id, Song>,
    pub covers: HashMap<Id, Cover>,
    pub index: Di<IdKey>,
}
impl MusicCollection {
    pub fn new() -> Self {
        MusicCollection {
            index: Di::new(0.9),
            ..Default::default()
        }
    }
}

#[derive(Default, Decode, Encode, Debug, Clone, Copy)]
pub enum IdKey {
    SongTitle(Id),
    ArtistName(Id),
    AlbumName(Id),

    #[default]
    Unknown,
}
