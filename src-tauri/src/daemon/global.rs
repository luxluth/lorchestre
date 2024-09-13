use crate::daemon::m3u8;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use bitcode::{Decode, Encode};
use color_thief::ColorFormat;
use lofty::picture::{MimeType, PictureType};
use lofty::prelude::*;
use lofty::probe::Probe;
use m3u8::PlaylistData;
use mime_guess::{self, mime};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::SystemTime;
use tantivy::collector::{Count, TopDocs};
use tantivy::query::FuzzyTermQuery;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexWriter, ReloadPolicy};
use tauri::Emitter;
use tracing::error;

#[derive(serde::Serialize, Debug)]
pub struct Cover {
    data: Vec<u8>,
    ext: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, Encode, Decode)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

const COLOR_THRESHOLD: f64 = 180.0;

impl Color {
    pub fn is_light_color(&self) -> bool {
        let luminance =
            0.2126 * (self.r as f64) + 0.7152 * (self.g as f64) + 0.0722 * (self.b as f64);

        luminance > COLOR_THRESHOLD
    }
}

#[derive(
    serde::Serialize, serde::Deserialize, Default, Debug, Clone, PartialEq, Eq, Encode, Decode,
)]
pub struct Album {
    pub name: String,
    pub artist: String,
    pub tracks: Vec<String>,
    pub year: Option<u32>,
    pub id: String,
    pub disc_total: u32,
    pub tracks_count: u32,
    pub genres: Vec<String>,
    pub encoder: String,
}

impl Album {
    pub fn remove_track(&mut self, path: String) {
        self.tracks.retain(|x| *x != path);
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Encode, Decode)]
pub struct Track {
    pub title: String,
    pub artists: Vec<String>,
    pub track: u32,
    pub album: String,
    pub album_artist: Option<String>,
    pub album_id: String,
    pub cover_ext: String,
    pub mime: String,
    pub disc: u32,
    pub disc_total: u32,
    pub album_year: Option<u32>,
    pub color: Option<Color>,
    pub is_light: Option<bool>,
    pub file_path: String,
    pub path_base64: String,
    pub duration: u64,
    pub bitrate: u32,
    pub encoder: String,
    pub genres: Vec<String>,
    pub tracks_count: u32,

    pub created_at: u64,
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.file_path == other.file_path
    }
}

impl Eq for Track {}

impl Track {
    pub fn from_file(covers_dir: &PathBuf, inode: PathBuf) -> Self {
        // TODO: avoid unwraping when openning audio files
        let tagged_file = Probe::open(&inode).unwrap().read().unwrap();
        let properties = tagged_file.properties();
        let bitrate = properties.audio_bitrate().unwrap_or(0);
        let duration = properties.duration();
        let mime = tagged_file.file_type();

        let default_tag = lofty::tag::Tag::new(lofty::tag::TagType::Id3v2);

        let tag = match tagged_file.primary_tag() {
            Some(primary_tag) => primary_tag,
            // If the "primary" tag doesn't exist, we just grab the
            // first tag we can find. Realistically, a tag reader would likely
            // iterate through the tags to find a suitable one.
            None => tagged_file.first_tag().unwrap_or(&default_tag),
        };

        let path = inode.to_str().unwrap().to_string();
        let mut audio: Track = Track {
            path_base64: URL_SAFE.encode(path.as_bytes()),
            file_path: path,
            ..Default::default()
        };

        audio.mime = match mime {
            lofty::file::FileType::Aac => "audio/aac",
            lofty::file::FileType::Aiff => "audio/aiff",
            lofty::file::FileType::Ape => "audio/ape",
            lofty::file::FileType::Flac => "audio/flac",
            lofty::file::FileType::Mpeg => "audio/mpeg",
            lofty::file::FileType::Mp4 => "audio/mp4",
            lofty::file::FileType::Mpc => "audio/mpc",
            lofty::file::FileType::Opus => "audio/webm",
            lofty::file::FileType::Vorbis => "audio/webm",
            lofty::file::FileType::Speex => "audio/speex",
            lofty::file::FileType::Wav => "audio/wav",
            lofty::file::FileType::WavPack => "audio/wav",
            _ => "application/octet-stream",
        }
        .to_string();

        if let Ok(meta) = inode.metadata() {
            if let Ok(tm) = meta.created() {
                let epoch = SystemTime::UNIX_EPOCH;
                audio.created_at = tm.duration_since(epoch).unwrap().as_secs();
            }
        };

        if let Some(year) = tag.year() {
            audio.album_year = Some(year);
        } else if let Some(year) = tag.get_string(&ItemKey::Unknown("TDOR".into())) {
            audio.album_year = Some(year.parse().unwrap_or(0));
        }

        if let Some(encoder) = tag.get_string(&ItemKey::EncoderSettings) {
            audio.encoder = encoder.to_string();
        }

        if let Some(genres) = tag.genre() {
            if genres.contains(';') {
                audio.genres = genres.split(';').map(|x| x.trim().to_string()).collect();
            } else {
                audio.genres = genres.split(' ').map(|x| x.trim().to_string()).collect();
            }
        }

        if let Some(title) = tag.title() {
            audio.title = title.to_string();
        }
        if let Some(artists) = tag.get_string(&ItemKey::TrackArtist) {
            audio.artists = artists
                .split(';')
                .filter(|x| !x.is_empty())
                .map(|x| x.trim().to_string())
                .collect();
        };

        if let Some(album) = tag.album() {
            audio.album = album.to_string();
        }

        if let Some(album_artist) = tag.get_string(&ItemKey::OriginalArtist) {
            audio.album_artist = Some(album_artist.to_string());
        }

        if let Some(no) = tag.track() {
            audio.track = no;
        }

        if let Some(tt) = tag.track_total() {
            audio.tracks_count = tt;
        }

        let mut bytes = audio.album.as_bytes().to_vec();
        bytes.extend(
            audio
                .artists
                .first()
                .unwrap_or(&"@UNKNOWN@".to_string())
                .as_bytes(),
        );

        let digest = md5::compute(bytes);

        audio.album_id = format!("{digest:x}");

        audio.disc = tag.disk().unwrap_or(1);
        audio.disc_total = tag.disk_total().unwrap_or(1);

        let cover = tag.get_picture_type(PictureType::CoverFront);
        if let Some(cover) = cover {
            let mime = cover.mime_type().unwrap();
            let cover = Cover {
                data: cover.data().to_vec(),
                ext: match mime {
                    MimeType::Png => ".png".to_string(),
                    MimeType::Jpeg => ".jpeg".to_string(),
                    MimeType::Tiff => ".tiff".to_string(),
                    MimeType::Bmp => ".bmp".to_string(),
                    MimeType::Gif => ".gif".to_string(),
                    MimeType::Unknown(o) => format!(".{o}"),
                    _ => ".png".to_string(),
                },
            };

            let pathstr = covers_dir.join(format!("{digest:x}{}", cover.ext));
            let cover_path = std::path::Path::new(&pathstr);

            if !cover_path.exists() {
                check_dir(covers_dir);
                let mut f = fs::File::create(cover_path).unwrap();
                f.write_all(&cover.data).unwrap();
            }

            let img = image::open(cover_path).unwrap();
            let pixels = utils::get_image_buffer(img);

            let color = color_thief::get_palette(&pixels, ColorFormat::Rgb, 1, 2).unwrap();

            let color = Color {
                r: color[0].r,
                g: color[0].g,
                b: color[0].b,
            };

            audio.is_light = Some(color.is_light_color());
            audio.color = Some(color);
            audio.cover_ext = cover.ext;
        }

        audio.duration = duration.as_secs();
        audio.bitrate = bitrate;

        audio
    }

    pub fn parse_lyrics(input: &str) -> alrc::AdvancedLrc {
        alrc::AdvancedLrc::parse(input).unwrap()
    }

    pub fn get_lyrics(&self) -> Vec<alrc::Line> {
        let lrc_path = PathBuf::from(&self.file_path).with_extension("lrc");
        if lrc_path.exists() {
            let mut f = fs::File::open(&lrc_path).unwrap();
            let mut buf = Vec::new();
            f.read_to_end(&mut buf).unwrap();
            let buf = String::from_utf8(buf);
            match buf {
                Ok(buf) => Track::parse_lyrics(&buf).lines,
                Err(e) => {
                    error!("{}", e);
                    vec![]
                }
            }
        } else {
            vec![]
        }
    }
}

impl Default for Track {
    fn default() -> Self {
        Self {
            title: "@UNKNOWN@".to_string(),
            artists: vec![],
            track: 0,
            album: "@UNKNOWN@".to_string(),
            album_artist: None,
            album_id: String::new(),
            album_year: None,
            cover_ext: ".png".to_string(),
            mime: "audio/mp3".to_string(),
            color: None,
            disc: 1,
            disc_total: 1,
            is_light: None,
            file_path: String::new(),
            path_base64: String::new(),
            bitrate: 0,
            duration: 0,
            created_at: 0,
            encoder: "Unknown".into(),
            genres: vec![],
            tracks_count: 0,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct Songs {
    pub audios: Vec<Track>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Encode, Decode)]
pub struct TrackCollection(String, Track);

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone, Encode, Decode)]
pub struct Media {
    pub tracks: Vec<TrackCollection>,
    pub albums: Vec<Album>,
    pub playlists: Vec<PlaylistData>,
}

impl Media {
    pub fn search(&self, cache_dir: PathBuf, query: &str) -> SearchResults {
        let songs_index_path = cache_dir.join(".search.songs");
        let albums_index_path = cache_dir.join(".search.albums");
        let songs_index = Index::open_in_dir(songs_index_path).unwrap();
        let albums_index = Index::open_in_dir(albums_index_path).unwrap();

        let mut schema_builder = Schema::builder();
        let title = schema_builder.add_text_field("title", TEXT | STORED);
        let artists = schema_builder.add_text_field("artists", TEXT | STORED);
        let album = schema_builder.add_text_field("album", TEXT | STORED);
        schema_builder.add_text_field("path", TEXT | STORED);
        let songs_schema = schema_builder.build();

        schema_builder = tantivy::schema::Schema::builder();
        let album_name = schema_builder.add_text_field("name", TEXT | STORED);
        let album_artist = schema_builder.add_text_field("artist", TEXT | STORED);
        schema_builder.add_text_field("id", TEXT | STORED);
        let albums_schema = schema_builder.build();

        let songs_reader = songs_index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .unwrap();

        let albums_reader = albums_index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .unwrap();

        let mut tracks = vec![];
        let mut albums = vec![];

        let song_searcher = songs_reader.searcher();
        let album_searcher = albums_reader.searcher();

        let terms = [
            Term::from_field_text(title, query),
            Term::from_field_text(artists, query),
            Term::from_field_text(album, query),
        ];

        for term in terms {
            let q = FuzzyTermQuery::new(term, 2, true);
            let (top_docs, _) = song_searcher
                .search(&q, &(TopDocs::with_limit(10), Count))
                .unwrap();

            for (_, doc_address) in top_docs {
                let doc: TantivyDocument = song_searcher.doc(doc_address).unwrap();
                let doc = doc.to_named_doc(&songs_schema);

                if let Some(paths) = doc.0.get("path") {
                    if let OwnedValue::Str(p) = paths[0].clone() {
                        if let Some(track) = self.get_song(&p) {
                            tracks.push(track);
                        };
                    }
                }
            }
        }

        let terms = [
            Term::from_field_text(album_name, query),
            Term::from_field_text(album_artist, query),
        ];

        for term in terms {
            let q = FuzzyTermQuery::new(term, 2, true);
            let (top_docs, _) = album_searcher
                .search(&q, &(TopDocs::with_limit(5), Count))
                .unwrap();

            for (_, doc_address) in top_docs {
                let doc: TantivyDocument = album_searcher.doc(doc_address).unwrap();
                let doc = doc.to_named_doc(&albums_schema);
                if let Some(ids) = doc.0.get("id") {
                    if let OwnedValue::Str(id) = ids[0].clone() {
                        if let Some(album) = self.get_album(&id) {
                            albums.push(album);
                        };
                    }
                }
            }
        }

        albums.dedup();
        tracks.dedup();

        SearchResults { albums, tracks }
    }

    pub fn cache(&self, cache_dir: PathBuf, win: Option<tauri::Window>) {
        let p_string = cache_dir.join(".cache");
        let ac_string = cache_dir.join(".cache.list");
        let ac_path = std::path::Path::new(&ac_string);
        utils::cache_audio_files(ac_path);

        let bin = bitcode::encode(&self.clone());
        let mut f = fs::File::create(&p_string).unwrap();
        let _ = f.write_all(&bin);

        self.create_search_index(cache_dir);

        if let Some(win) = win {
            let _ = win.emit("synched", ());
        }
    }

    pub fn create_search_index(&self, cache_dir: PathBuf) {
        let songs_index_path = cache_dir.join(".search.songs");
        let albums_index_path = cache_dir.join(".search.albums");

        if songs_index_path.exists() {
            let _ = std::fs::remove_dir_all(&songs_index_path);
        }

        if std::fs::DirBuilder::new()
            .recursive(true)
            .create(&songs_index_path)
            .is_err()
        {
            return;
        };

        if albums_index_path.exists() {
            let _ = std::fs::remove_dir_all(&albums_index_path);
        }
        if std::fs::DirBuilder::new()
            .recursive(true)
            .create(&albums_index_path)
            .is_err()
        {
            return;
        };

        let mut schema_builder = tantivy::schema::Schema::builder();
        let title = schema_builder.add_text_field("title", TEXT | STORED);
        let artists = schema_builder.add_text_field("artists", TEXT | STORED);
        let album = schema_builder.add_text_field("album", TEXT | STORED);
        let path = schema_builder.add_text_field("path", TEXT | STORED);
        let songs_schema = schema_builder.build();

        schema_builder = tantivy::schema::Schema::builder();
        let album_name = schema_builder.add_text_field("name", TEXT | STORED);
        let album_artist = schema_builder.add_text_field("artist", TEXT | STORED);
        let album_id = schema_builder.add_text_field("id", TEXT | STORED);
        let album_schema = schema_builder.build();

        let songs_index = Index::create_in_dir(&songs_index_path, songs_schema).unwrap();
        let mut index_writer: IndexWriter = songs_index.writer(50_000_000).unwrap();
        for TrackCollection(_, track) in &self.tracks {
            let _ = index_writer.add_document(doc!(
                title => track.title.clone(),
                artists => track.artists.join(";"),
                album => track.album.clone(),
                path => track.file_path.clone(),
            ));
        }

        let p = index_writer.prepare_commit().unwrap();
        let _ = p.commit();

        let albums_index = Index::create_in_dir(&albums_index_path, album_schema).unwrap();
        index_writer = albums_index.writer(50_000_000).unwrap();
        for album in &self.albums {
            let _ = index_writer.add_document(doc!(
                album_name => album.name.clone(),
                album_artist => album.artist.clone(),
                album_id => album.id.clone(),
            ));
        }

        let p = index_writer.prepare_commit().unwrap();
        let _ = p.commit();
    }

    pub fn swap_with(&mut self, media: Media) {
        self.albums = media.albums;
        self.tracks = media.tracks;
        self.playlists = media.playlists;
    }

    pub fn add_song(&mut self, song: Track) {
        let mut inserted = false;
        for album in &mut self.albums {
            if song.album_id == album.id {
                album.tracks.push(song.file_path.clone());
                self.tracks
                    .ninsert(song.file_path.to_string(), song.clone());
                inserted = true;
                break;
            }
        }
        if !inserted {
            self.tracks.ninsert(song.file_path.clone(), song.clone());
            let albums = Songs { audios: vec![song] }.get_albums();
            self.albums.extend(albums);
        }
    }

    pub fn add_media(&mut self, path: PathBuf, covers_dir: &PathBuf) {
        let ext = path.extension().unwrap().to_str().unwrap();
        if ext == "m3u8" {
            self.add_playlist(m3u8::M3U8::parse(path));
        } else if ext == "playlist" {
            self.add_playlist(PlaylistData::parse(format!("{}", path.display())));
        } else {
            self.add_song(Track::from_file(covers_dir, path));
        }
    }

    pub fn remove_media(&mut self, path: PathBuf) {
        let ext = path.extension().unwrap().to_str().unwrap();
        if ext == "playlist" {
            self.remove_playlist(format!("{}", path.display()));
        } else {
            self.remove_song(format!("{}", path.display()));
        }
    }

    pub fn add_playlist(&mut self, playlist: PlaylistData) {
        let mut playlist_exist = false;
        for list in &self.playlists {
            if list.path == playlist.path {
                playlist_exist = true;
                break;
            }
        }

        if !playlist_exist {
            self.playlists.push(playlist);
        }
    }

    pub fn substitute_playlist(&mut self, playlist: PlaylistData) {
        for list in &mut self.playlists {
            if list.path == playlist.path {
                list.path = playlist.path;
                list.metadata = playlist.metadata;
                list.tracks = playlist.tracks;

                break;
            }
        }
    }

    #[inline]
    pub fn remove_playlist(&mut self, path: String) {
        self.playlists.retain(|x| x.path != path);
    }

    pub fn remove_song(&mut self, path: String) {
        for album in &mut self.albums {
            album.remove_track(path.clone());
        }

        self.tracks.remove_entry(&path);
        self.albums.retain(|x| !x.tracks.is_empty());
    }

    pub fn get_album(&self, id: &str) -> Option<Album> {
        let id = id.to_string();
        for album in self.albums.iter() {
            if album.id == id {
                return Some(album.clone());
            }
        }

        None
    }

    pub fn get_playlist<T>(&self, path_base64: T) -> Option<PlaylistData>
    where
        T: AsRef<[u8]>,
    {
        let path = String::from_utf8_lossy(&URL_SAFE.decode(path_base64).unwrap()).to_string();

        for playlist in self.playlists.iter() {
            if playlist.path == path {
                return Some(playlist.clone());
            }
        }

        None
    }

    pub fn get_song(&self, path: &str) -> Option<Track> {
        self.tracks.get(path).cloned()
    }
}

pub trait MapLikeAction {
    fn ninsert(&mut self, path: String, song: Track);
    fn get(&self, path: &str) -> Option<&Track>;
    fn remove_entry(&mut self, path: &str);
}

impl MapLikeAction for Vec<TrackCollection> {
    fn ninsert(&mut self, path: String, song: Track) {
        let mut exist = false;
        for TrackCollection(p, _) in self.iter() {
            if *p == path {
                exist = true;
                break;
            }
        }

        if !exist {
            self.push(TrackCollection(path, song));
        } else {
            for TrackCollection(p, s) in self.iter_mut() {
                if *p == path {
                    *s = song;
                    break;
                }
            }
        }
    }
    fn get(&self, path: &str) -> Option<&Track> {
        let path = path.to_string();
        for TrackCollection(p, s) in self.iter() {
            if *p == path {
                return Some(s);
            }
        }
        None
    }
    fn remove_entry(&mut self, path: &str) {
        let path = path.to_string();
        self.retain(|TrackCollection(p, _)| *p != path);
    }
}

#[derive(serde::Serialize, Debug)]
pub struct SearchResults {
    pub albums: Vec<Album>,
    pub tracks: Vec<Track>,
}

impl Songs {
    pub fn get_albums(self) -> Vec<Album> {
        let mut albums = vec![];
        let mut album_map: HashMap<String, Vec<Track>> = HashMap::new();
        for audio in self.audios {
            album_map
                .entry(audio.album_id.clone())
                .or_default()
                .push(audio);
        }

        for (k, v) in album_map {
            albums.push(Album {
                name: v[0].album.clone(),
                artist: v[0].album_artist.clone().unwrap_or(String::from(
                    v[0].artists
                        .clone()
                        .first()
                        .unwrap_or(&"@UNKNOWN@".to_string()),
                )),
                disc_total: v[0].disc_total,
                year: v[0].album_year,
                encoder: v[0].encoder.clone(),
                tracks_count: v[0].tracks_count,
                genres: v[0].genres.clone(),
                tracks: v.into_iter().map(|x| x.file_path).collect(),
                id: k,
            });
        }

        albums
    }
}

pub fn check_dir(dir: &PathBuf) {
    if !dir.exists() {
        fs::DirBuilder::new().recursive(true).create(dir).unwrap();
    }
}

pub mod utils {
    use std::{
        io::{Read, Write},
        path::PathBuf,
        str::FromStr,
    };

    use glob::glob;

    pub fn get_image_buffer(img: image::DynamicImage) -> Vec<u8> {
        img.to_rgb8().to_vec()
    }

    pub fn get_audio_files() -> Vec<PathBuf> {
        let mut files = vec![];
        if let Some(audio_dir) = dirs::audio_dir() {
            if let Ok(paths) = glob(&format!("{}/**/*", audio_dir.display())) {
                for inode in paths.flatten() {
                    if inode.is_file() {
                        let guess =
                            mime_guess::from_path(&inode).first_or("text/plain".parse().unwrap());
                        if guess.type_() == super::mime::AUDIO
                            || (inode.extension().is_some()
                                && inode.extension().unwrap() == "playlist")
                        {
                            files.push(inode);
                        }
                    }
                }
            }
        }
        files
    }

    pub fn cache_audio_files(cache_path: &std::path::Path) {
        let files: Vec<String> = get_audio_files()
            .iter()
            .map(|x| format!("{}", x.display()))
            .collect();
        let data = files.join("\n");
        let mut f = std::fs::File::create(cache_path).unwrap();
        let _ = f.write_all(data.as_bytes());
    }

    pub fn read_cache_audio_files(cache_path: &std::path::Path) -> Vec<PathBuf> {
        let mut buf = String::new();
        if cache_path.exists() {
            let mut f = std::fs::File::open(cache_path).unwrap();
            let _ = f.read_to_string(&mut buf);
        }

        buf.lines().map(|x| PathBuf::from_str(x).unwrap()).collect()
    }
}
