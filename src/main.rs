// palette_scale, an image upscaler which upscales an image normally and the corrects the colors to align with the original image's color palette.
// Copyright (C) 2021  Victor "VoxWave" Bankowski

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use clap::App;
use clap::Arg;
use image::imageops::resize;
use image::imageops::FilterType::Lanczos3;
use image::io::Reader as ImageReader;
use image::Rgba;
use image::RgbaImage;
use rand::prelude::SliceRandom;
use std::collections::HashSet;
use std::path::Path;

fn main() {
    let matches = App::new("Palette based image upscaler")
        .version("0.1")
        .author("Victor \"VoxWave\" Bankowski <victor_bankowski@hotmail.com>")
        .about("Meant to upscale pixel art")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .required(true),
        )
        .arg(
            Arg::with_name("scale")
                .short("s")
                .long("scale")
                .value_name("SCALE")
                .default_value("2"),
        )
        .get_matches();
    let filepath = matches.value_of("file").unwrap();
    let path = Path::new(filepath);
    let scale = matches.value_of("scale").unwrap().parse::<u32>().unwrap();
    let image = ImageReader::open(path).unwrap().decode().unwrap();
    let new_image = palette_scale(&image.to_rgba8(), scale);
    new_image
        .save(format!(
            "{}_resized.png",
            path.file_stem().unwrap().to_string_lossy()
        ))
        .unwrap();
}

fn palette_scale(image: &RgbaImage, scale: u32) -> RgbaImage {
    //TODO: Replace the rng with a manually seeded one to increase precictability.
    let mut rng = rand::thread_rng();
    // Save the colors of the original image in a palette.
    let mut palette = HashSet::new();
    for color in image.pixels() {
        palette.insert(*color);
    }
    let mut resized = resize(
        image,
        image.width() * scale,
        image.height() * scale,
        Lanczos3,
    );
    for x in 0..resized.width() {
        for y in 0..resized.height() {
            let pixel_color = resized.get_pixel(x, y).clone();
            if palette.contains(&pixel_color) {
                continue;
            }
            // Get the distances of the colors of the palette to the color of this pixel
            let mut color_distances = palette
                .iter()
                .map(|color| (color, dist_squared(color, &pixel_color)))
                .collect::<Vec<_>>();
            // Sort the colors by the distance
            color_distances.sort_by_key(|(_, dist)| *dist);
            // Get the color or colors that are closest to the pixel
            let first_val = color_distances[0].1;
            let mut index = match color_distances
                .iter()
                .enumerate()
                .find(|(_, (_, dist))| *dist != first_val)
            {
                Some((index, _)) => index,
                None => 1,
            };
            // Assign the new color randomly for this pixel from the closest colors
            resized.put_pixel(x, y, *color_distances[0..index].choose(&mut rng).unwrap().0);
        }
    }
    resized
}

fn dist_squared(a: &Rgba<u8>, b: &Rgba<u8>) -> i32 {
    (a.0[0] as i32 - b.0[0] as i32).pow(2)
        + (a.0[1] as i32 - b.0[1] as i32).pow(2)
        + (a.0[2] as i32 - b.0[2] as i32).pow(2)
        + (a.0[3] as i32 - b.0[3] as i32).pow(2)
}
