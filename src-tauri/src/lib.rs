use color_thief::ColorFormat;
use glob::glob;
use lofty::picture::{MimeType, PictureType};
use lrc::Lyrics;
use mime_guess::{self, mime};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use lofty::prelude::*;
use lofty::probe::Probe;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct LyricLine {
    pub start_time: i64,
    pub text: String,
}

#[derive(serde::Serialize, Debug)]
pub struct Cover {
    data: Vec<u8>,
    ext: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn is_light_color(&self) -> bool {
        let luminance =
            0.2126 * (self.r as f64) + 0.7152 * (self.g as f64) + 0.0722 * (self.b as f64);
        let threshold = 180.0;

        luminance > threshold
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
pub struct Album {
    pub name: String,
    pub artist: String,
    pub tracks: Vec<Audio>,
    pub year: Option<u32>,
    pub id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Audio {
    pub title: String,
    pub artists: Vec<String>,
    pub track: u32,
    pub album: String,
    pub album_artist: Option<String>,
    pub album_id: String,
    pub album_year: Option<u32>,
    pub lyrics: Vec<LyricLine>,
    pub cover: Option<String>,
    pub color: Option<Color>,
    pub is_light: Option<bool>,
    pub file_path: String,
    pub duration: u64,
    pub id: String,
}

impl Default for Audio {
    fn default() -> Self {
        Self {
            title: "@UNKNOWN@".to_string(),
            artists: vec![],
            track: 0,
            album: "@UNKNOWN@".to_string(),
            album_artist: None,
            album_id: String::new(),
            album_year: None,
            lyrics: vec![],
            cover: None,
            color: None,
            is_light: None,
            file_path: String::new(),
            duration: 0,
            id: String::new(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct Songs {
    pub audios: Vec<Audio>,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
pub struct Media {
    pub albums: Vec<Album>,
}

impl Media {
    pub fn new(songs: Songs) -> Self {
        Self {
            albums: songs.get_albums(),
        }
    }

    pub fn get_album(&self, id: String) -> Option<Album> {
        for album in self.albums.iter() {
            if album.id == id {
                return Some(Album {
                    name: album.name.clone(),
                    artist: album.artist.clone(),
                    tracks: album.tracks.clone(),
                    year: album.year.clone(),
                    id: album.id.clone(),
                });
            }
        }

        None
    }
}

impl Songs {
    pub fn get_albums(self) -> Vec<Album> {
        let mut albums = vec![];
        let mut album_map: HashMap<String, Vec<Audio>> = HashMap::new();
        for audio in self.audios {
            album_map
                .entry(audio.album_id.clone())
                .or_insert_with(Vec::new) // Insert a new Vec<Audio> if the key does not exist
                .push(audio);
        }

        for (k, v) in album_map {
            albums.push(Album {
                name: v[0].album.clone(),
                artist: v[0].album_artist.clone().unwrap_or(String::from(
                    v[0].artists
                        .clone()
                        .get(0)
                        .unwrap_or(&"@UNKNOWN@".to_string()),
                )),
                year: v[0].album_year,
                tracks: v,
                id: k,
            });
        }

        albums
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
pub struct MediaCache {
    pub media: Media,
    pub md5: String,
}

fn check_cache_covers(dir: String) {
    let dir = Path::new(&dir);
    if !dir.exists() {
        fs::DirBuilder::new().recursive(true).create(dir).unwrap();
    }
}

pub mod utils {
    use crate::glob;

    pub fn get_image_buffer(img: image::DynamicImage) -> Vec<u8> {
        match img {
            image::DynamicImage::ImageRgb8(buffer) => buffer.to_vec(),
            _ => unreachable!(),
        }
    }

    pub fn remove_lyrics_tags(buf: String) -> String {
        let strings: Vec<String> = buf
            .lines()
            .filter(|x| !x.is_empty())
            .filter(|x| {
                let mut chars = x.chars();
                let openparen = chars.next().unwrap();
                let nextdata = chars.next().unwrap();
                if openparen == '[' && nextdata.is_ascii_digit() {
                    return true;
                } else {
                    return false;
                }
            })
            .map(|x| x.to_string())
            .collect();

        strings.join("\n")
    }

    pub fn music_dir_md5() -> String {
        let mut files = vec![];
        if let Some(music_dir) = dirs::audio_dir() {
            if let Ok(paths) = glob(&format!("{}/**/*", music_dir.display())) {
                for p in paths {
                    if p.is_ok() {
                        let path_buf = p.unwrap();
                        files.extend(path_buf.to_str().unwrap().as_bytes());
                    }
                }

                let digest = md5::compute(files);

                return format!("{digest:x}");
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }
}

impl Songs {
    pub fn new(cache_dir: PathBuf) -> Self {
        check_cache_covers(format!("{}/covers", cache_dir.display()));
        let covers_dir = format!("{}/covers", cache_dir.display());

        if let Some(music_dir) = dirs::audio_dir() {
            let mut audios = vec![];
            glob(&format!("{}/**/*", music_dir.display()))
                .expect("Failed to read glob pattern")
                .for_each(|entry| {
                    if let Ok(inode) = entry {
                        if inode.is_file() {
                            let guess = mime_guess::from_path(&inode)
                                .first_or("text/plain".parse().unwrap());

                            if guess.type_() == mime::AUDIO {
                                let tagged_file = Probe::open(&inode).unwrap().read().unwrap();
                                let properties = tagged_file.properties();
                                let duration = properties.duration();

                                let default_tag = lofty::tag::Tag::new(lofty::tag::TagType::Id3v2);

                                let tag = match tagged_file.primary_tag() {
                                    Some(primary_tag) => primary_tag,
                                    // If the "primary" tag doesn't exist, we just grab the
                                    // first tag we can find. Realistically, a tag reader would likely
                                    // iterate through the tags to find a suitable one.
                                    None => tagged_file.first_tag().unwrap_or(&default_tag),
                                };

                                let mut audio: Audio = Audio {
                                    file_path: inode.to_str().unwrap().to_string(),
                                    ..Default::default()
                                };

                                if let Some(year) = tag.year() {
                                    audio.album_year = Some(year);
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

                                if let Some(album_artist) = tag.get_string(&ItemKey::OriginalArtist)
                                {
                                    audio.album_artist = Some(album_artist.to_string());
                                }

                                if let Some(no) = tag.track() {
                                    audio.track = no;
                                }

                                let mut bytes = audio.album.as_bytes().to_vec();
                                bytes.extend(
                                    audio
                                        .artists
                                        .get(0)
                                        .unwrap_or(&"@UNKNOWN@".to_string())
                                        .as_bytes(),
                                );

                                let digest = md5::compute(bytes);

                                audio.album_id = format!("{digest:x}");

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

                                    let pathstr = format!("{covers_dir}/{digest:x}{}", cover.ext);
                                    let cover_path = std::path::Path::new(&pathstr);

                                    if !cover_path.exists() {
                                        let mut f = fs::File::create(cover_path).unwrap();
                                        f.write_all(&cover.data).unwrap();
                                    }

                                    let img = image::open(cover_path).unwrap();
                                    let pixels = utils::get_image_buffer(img);

                                    let color =
                                        color_thief::get_palette(&pixels, ColorFormat::Rgb, 1, 2)
                                            .unwrap();

                                    let color = Color {
                                        r: color[0].r,
                                        g: color[0].g,
                                        b: color[0].b,
                                    };

                                    audio.is_light = Some(color.is_light_color());
                                    audio.color = Some(color);

                                    let path = cover_path.to_str().unwrap().to_string();

                                    audio.cover = Some(path);
                                }

                                audio.duration = duration.as_secs();

                                let lrc_path = inode.with_extension("lrc");
                                if lrc_path.exists() {
                                    let mut f = fs::File::open(&lrc_path).unwrap();
                                    let mut buf = Vec::new();
                                    f.read_to_end(&mut buf).unwrap();
                                    let buf = String::from_utf8(buf);
                                    match buf {
                                        Ok(buf) => {
                                            let buf = utils::remove_lyrics_tags(buf);
                                            let lyrics = Lyrics::from_str(buf).unwrap();
                                            let lines: Vec<LyricLine> = lyrics
                                                .get_timed_lines()
                                                .iter()
                                                .map(|(time, content)| LyricLine {
                                                    start_time: time.get_timestamp(),
                                                    text: content.to_string(),
                                                })
                                                .collect();

                                            audio.lyrics = lines;
                                        }
                                        Err(e) => {
                                            eprintln!("{e}");
                                        }
                                    }
                                }

                                let data = format!(
                                    "{}{}{}{}###{}{}",
                                    audio.title,
                                    audio.album,
                                    audio.duration,
                                    audio.artists.join(";"),
                                    audio.album_id,
                                    audio.track,
                                );

                                let id = md5::compute(data);
                                audio.id = format!("{id:x}");

                                audios.push(audio);
                            }
                        }
                    }
                });

            Self { audios }
        } else {
            Self::default()
        }
    }
}
