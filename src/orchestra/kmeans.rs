use std::path::Path;

use bincode::{Decode, Encode};
use image::imageops::FilterType;
use kmeans_colors::get_kmeans_hamerly;
use mtk::{Color, rgb};
use palette::{FromColor, IntoColor, Lab, Srgb};

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct Swatch {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub percentage: f32,
}

impl Swatch {
    pub fn to_color(&self) -> Color {
        rgb!(self.r, self.g, self.b)
    }
}

pub fn extract_album_palette<P: AsRef<Path>>(
    path: P,
    k: usize,
) -> Result<Vec<Swatch>, Box<dyn std::error::Error>> {
    let img = image::open(path)?;

    let thumb = img.resize(256, 256, FilterType::Triangle).to_rgb8();

    let lab_pixels: Vec<Lab> = thumb
        .pixels()
        .map(|p| {
            let srgb = Srgb::new(
                p[0] as f32 / 255.0,
                p[1] as f32 / 255.0,
                p[2] as f32 / 255.0,
            );
            srgb.into_color()
        })
        .collect();

    let max_iterations = 20;
    let converge = 1.0;
    let verbose = false;
    let seed = 42;

    let result = get_kmeans_hamerly(k, max_iterations, converge, verbose, &lab_pixels, seed);

    let mut counts = vec![0usize; k];
    for &idx in &result.indices {
        counts[idx as usize] += 1;
    }

    let total_pixels = lab_pixels.len() as f32;

    let mut palette: Vec<Swatch> = result
        .centroids
        .into_iter()
        .enumerate()
        .map(|(i, lab)| {
            let srgb: Srgb = Srgb::from_color(lab);
            Swatch {
                r: (srgb.red.clamp(0.0, 1.0) * 255.0).round() as u8,
                g: (srgb.green.clamp(0.0, 1.0) * 255.0).round() as u8,
                b: (srgb.blue.clamp(0.0, 1.0) * 255.0).round() as u8,
                percentage: (counts[i] as f32 / total_pixels) * 100.0,
            }
        })
        .collect();

    palette.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap());
    Ok(palette)
}
