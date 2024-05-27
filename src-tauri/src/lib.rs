use audiotags::Tag;
use color_thief::ColorFormat;
use glob::glob;
use lrc::Lyrics;
use mime_guess::{self, mime};
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

use lofty::prelude::*;
use lofty::probe::Probe;

#[derive(serde::Serialize, Debug)]
pub struct LyricLine {
    pub start_time: i64,
    pub text: String,
}

#[derive(serde::Serialize, Debug)]
pub struct Cover {
    data: Vec<u8>,
    ext: String,
}

#[derive(serde::Serialize, Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn is_light_color(&self) -> bool {
        let luminance =
            0.2126 * (self.r as f64) + 0.7152 * (self.g as f64) + 0.0722 * (self.b as f64);
        let threshold = 200.0;

        luminance > threshold
    }
}

#[derive(serde::Serialize, Default, Debug)]
pub struct Audio {
    pub title: Option<String>,
    pub artists: Vec<String>,
    pub album: Option<String>,
    pub lyrics: Vec<LyricLine>,
    pub cover: Option<String>,
    pub color: Option<Color>,
    pub is_light: Option<bool>,
    pub file_path: String,
    pub duration: u64,
}

// fn random() {
//     eprintln!("{}", tauri::path::PathResolver::app_cache_dir(x))
// }

#[derive(serde::Serialize, Default)]
pub struct Media {
    pub audios: Vec<Audio>,
}

fn check_cache_dir(dir: PathBuf) {
    if !dir.exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .create(format!("{}/covers", dir.display()))
            .unwrap();
    }
}

mod utils {
    pub fn get_image_buffer(img: image::DynamicImage) -> Vec<u8> {
        match img {
            image::DynamicImage::ImageRgb8(buffer) => buffer.to_vec(),
            _ => unreachable!(),
        }
    }
}

impl Media {
    pub fn new(cache_dir: PathBuf) -> Self {
        check_cache_dir(cache_dir.clone());

        let covers_dir = format!("{}/covers", cache_dir.display());

        if let Some(music_dir) = dirs::audio_dir() {
            let music_dir = music_dir.to_str().unwrap();
            let mut audios = vec![];
            glob(&format!("{music_dir}/**/*"))
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

                                let tag = Tag::new().read_from_path(&inode).unwrap();
                                let mut audio: Audio = Audio {
                                    file_path: inode.to_str().unwrap().to_string(),
                                    ..Default::default()
                                };

                                if let Some(title) = tag.title() {
                                    audio.title = Some(title.to_string());
                                }
                                if let Some(artists) = tag.artists() {
                                    audio.artists =
                                        artists.into_iter().map(|x| x.to_string()).collect();
                                }
                                if let Some(album) = tag.album_title() {
                                    audio.album = Some(album.to_string());
                                }

                                if let Some(cover) = tag.album_cover() {
                                    let cover = Cover {
                                        data: cover.data.to_vec(),
                                        ext: match cover.mime_type {
                                            audiotags::MimeType::Png => ".png".to_string(),
                                            audiotags::MimeType::Jpeg => ".jpeg".to_string(),
                                            audiotags::MimeType::Tiff => ".tiff".to_string(),
                                            audiotags::MimeType::Bmp => ".bmp".to_string(),
                                            audiotags::MimeType::Gif => ".gif".to_string(),
                                        },
                                    };

                                    let digest = md5::compute(
                                        audio.album.clone().unwrap_or(String::from("NoName")),
                                    );

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
