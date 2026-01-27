use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    fs,
    io::{Read, Write},
    path::PathBuf,
    time::{Duration, SystemTime},
};

use bincode::{Decode, Encode};
use glob::glob;
use lofty::{
    file::{AudioFile, TaggedFileExt},
    picture::{MimeType, PictureType},
    probe::Probe,
    tag::{Accessor, ItemKey},
};

use crate::{Lorch, di::Di};

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
    fn new(id: Id, file_path: PathBuf) -> Self {
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

#[derive(Debug, Decode, Encode, Clone)]
pub struct Album {
    pub id: Id,
    pub name: String,
    pub genres: HashSet<String>,
    pub artist: Option<Id>,
    pub year: Option<u32>,
    pub songs: Vec<Id>,
    pub disc_total: u32,
    pub songs_count: u32,
    pub cover: Option<Id>,
}
impl Album {
    fn new(id: Id) -> Self {
        Self {
            id,
            name: String::new(),
            genres: HashSet::new(),
            artist: None,
            year: None,
            songs: vec![],
            disc_total: 0,
            songs_count: 0,
            cover: None,
        }
    }
}

#[derive(Default, Debug, Decode, Encode)]
pub struct Cover {
    id: Id,
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
        Lorch::covers_dir().join(&format!("{:x}{}", self.id.digest().unwrap(), self.ext))
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
    fn new() -> Self {
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

pub struct MusicCollectionIndexer {
    pub collection: MusicCollection,

    artist_id_store: IdStore,
    album_id_store: IdStore,
    song_id_store: IdStore,
}

impl MusicCollectionIndexer {
    pub fn new() -> Self {
        Self {
            collection: MusicCollection::new(),
            artist_id_store: IdStore::default(),
            album_id_store: IdStore::default(),
            song_id_store: IdStore::default(),
        }
    }

    pub fn load_from_cache(&mut self) {
        let cache_path = Lorch::cache_path();
        let mut f = std::fs::File::open(cache_path).unwrap();
        let config = bincode::config::standard();
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).unwrap();
        let (collection, _): (MusicCollection, usize) =
            bincode::decode_from_slice(&buf, config).unwrap();

        self.collection = collection;
    }

    pub fn save(&mut self) {
        let cache_path = Lorch::cache_path();
        let mut f = std::fs::File::create(cache_path).unwrap();
        let config = bincode::config::standard();
        let data = bincode::encode_to_vec(&self.collection, config).unwrap();
        f.write_all(&data).unwrap();
    }
}

impl MusicCollectionIndexer {
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
                self.collection.index.insert(title, IdKey::SongTitle(id));
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

            if let Some(lyrics) = tag.get_string(&ItemKey::Lyrics) {
                audio.embeded_lyrics = Some(lyrics.to_string());
            }

            if let Some(album) = tag.album() {
                let mut bytes = album.as_bytes().to_vec();
                let album_artist = {
                    if let Some(album_artist) = tag.get_string(&ItemKey::AlbumArtist) {
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

                let album_id = self.album_id_store.digest(bytes);
                if let Some(album) = self.collection.albums.get_mut(&album_id) {
                    audio.album = Some(album_id);
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
                    let mut album = Album::new(album_id);
                    audio.album = Some(album_id);

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

                    if let Some(album_name) = tag.album() {
                        album.name = album_name.to_string();
                        self.collection
                            .index
                            .insert(&album_name, IdKey::AlbumName(album_id));
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
                    } else if let Some(year) = tag.get_string(&ItemKey::Unknown("date".into())) {
                        album.year = Some(year.parse().unwrap_or(0));
                    }

                    album.songs.push(audio.id);
                    if let Some(tt) = tag.track_total() {
                        album.songs_count = tt;
                    }

                    album.disc_total = tag.disk_total().unwrap_or(1);
                    let cover_id = album.id;

                    let possible_covers = [
                        PictureType::CoverFront,
                        PictureType::Media,
                        PictureType::Other,
                        PictureType::CoverBack,
                    ];

                    'cover_loop: for picture_type in possible_covers {
                        if let Some(cover) = tag.get_picture_type(picture_type) {
                            let mime = cover.mime_type().unwrap();
                            let data = cover.data().to_vec();

                            let cover = Cover::new(cover_id, mime);
                            let cover_path = cover.get_path();

                            let mut f = fs::File::create(cover_path).unwrap();
                            f.write_all(&data).unwrap();

                            album.cover = Some(cover.id);
                            self.collection.covers.insert(cover_id, cover);

                            break 'cover_loop;
                        }
                    }

                    self.collection.albums.insert(album_id, album);
                }
            }

            self.collection.songs.insert(id, audio);
        }
    }

    pub fn add_artist(&mut self, name: String) -> Id {
        for (id, artist) in &self.collection.artists {
            if artist.name == name {
                return *id;
            }
        }

        let id = self.artist_id_store.next();
        self.collection.index.insert(&name, IdKey::ArtistName(id));
        let artist = Artist { id, name };

        self.collection.artists.insert(id, artist);

        return id;
    }

    pub fn get_artist_id_by_name(&self, name: &str) -> Option<Id> {
        for (id, artist) in &self.collection.artists {
            if artist.name == name {
                return Some(*id);
            }
        }

        None
    }

    pub fn get_artist(&self, id: &Id) -> Option<&Artist> {
        self.collection.artists.get(id)
    }

    pub fn get_album(&self, id: &Id) -> Option<&Album> {
        self.collection.albums.get(id)
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

        self.collection.index.finalize();
    }
}
