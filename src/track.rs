use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    time::{Duration, SystemTime},
};

use glob::glob;
use lofty::{
    file::{AudioFile, TaggedFileExt},
    probe::Probe,
    tag::{Accessor, ItemKey},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Id {
    Digest(md5::Digest),
    Number(usize),
    Unresolved,
}

#[derive(Debug, Clone)]
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
    pub disc: u32,
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
            disc: 1,
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
    pub id: Id,
    pub genres: HashSet<String>,
    pub artist: Option<Id>,
    pub year: Option<u32>,
    pub songs: Vec<Id>,
    pub disc_total: u32,
    pub songs_count: u32,
}
impl Album {
    fn new(id: Id) -> Self {
        Self {
            id,
            genres: HashSet::new(),
            artist: None,
            year: None,
            songs: vec![],
            disc_total: 0,
            songs_count: 0,
        }
    }
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

            if let Some(artists) = tag.get_string(&ItemKey::TrackArtist) {
                audio.artists = artists
                    .split(';')
                    .filter(|x| !x.is_empty())
                    .map(|x| self.add_artist(x.trim().to_string()))
                    .collect();
            };

            audio.disc = tag.disk().unwrap_or(1);
            if let Some(no) = tag.track() {
                audio.track = no;
            }

            if let Some(album) = tag.album() {
                let mut bytes = album.as_bytes().to_vec();
                let album_artist = {
                    if let Some(album_artist) = tag.get_string(&ItemKey::OriginalArtist) {
                        album_artist
                    } else {
                        if let Some(id) = audio.artists.first() {
                            self.get_artist(id).unwrap().name.as_str()
                        } else {
                            "@UNKOWN@"
                        }
                    }
                }
                .as_bytes();
                bytes.extend(album_artist);

                let id = self.album_id_store.digest(bytes);
                if let Some(album) = self.albums.get_mut(&id) {
                    audio.album = Some(id);
                    if let Some(genres) = tag.genre() {
                        if genres.contains(';') {
                            let genres: Vec<String> =
                                genres.split(';').map(|x| x.trim().to_string()).collect();
                            album.genres.extend(genres);
                        } else {
                            let genres: Vec<String> =
                                genres.split(' ').map(|x| x.trim().to_string()).collect();
                            album.genres.extend(genres);
                        }
                    }

                    album.songs.push(audio.id);
                } else {
                    // let Album { id, genres, artist, year, songs, disc_total, songs_count }
                    let mut album = Album::new(id);

                    if let Some(genres) = tag.genre() {
                        if genres.contains(';') {
                            let genres: Vec<String> =
                                genres.split(';').map(|x| x.trim().to_string()).collect();
                            album.genres.extend(genres);
                        } else {
                            let genres: Vec<String> =
                                genres.split(' ').map(|x| x.trim().to_string()).collect();
                            album.genres.extend(genres);
                        }
                    }

                    album.artist = {
                        if let Some(album_artist) = tag.get_string(&ItemKey::OriginalArtist) {
                            self.get_artist_id_by_name(album_artist)
                        } else {
                            if let Some(id) = audio.artists.first() {
                                Some(*id)
                            } else {
                                self.get_artist_id_by_name("@UNKOWN@")
                            }
                        }
                    };

                    if let Some(year) = tag.year() {
                        album.year = Some(year);
                    } else if let Some(year) = tag.get_string(&ItemKey::Unknown("TDOR".into())) {
                        album.year = Some(year.parse().unwrap_or(0));
                    }

                    album.songs.push(audio.id);
                    if let Some(tt) = tag.track_total() {
                        album.songs_count = tt;
                    }

                    album.disc_total = tag.disk_total().unwrap_or(1);
                }
            }
        }
    }

    pub fn add_artist(&mut self, name: String) -> Id {
        for (id, artist) in &self.artists {
            if artist.name == name {
                return *id;
            }
        }

        let id = self.artist_id_store.next();
        let artist = Artist { id, name };

        self.artists.insert(id, artist);

        return id;
    }

    pub fn get_artist_id_by_name(&self, name: &str) -> Option<Id> {
        for (id, artist) in &self.artists {
            if artist.name == name {
                return Some(*id);
            }
        }

        None
    }

    pub fn get_artist(&self, id: &Id) -> Option<&Artist> {
        self.artists.get(id)
    }

    pub fn get_album(&self, id: &Id) -> Option<&Album> {
        self.albums.get(id)
    }

    pub fn index(&mut self, dir_path: PathBuf) {
        self.add_artist("@UNKOWN@".to_string());
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
