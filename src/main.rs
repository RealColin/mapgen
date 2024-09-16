mod continent;
mod map;
mod voronoi;

use std::collections::HashMap;

use image::{GenericImage, GenericImageView, Rgb, RgbImage, Rgba, RgbaImage};
use libnoise::{Generator, Source, Visualizer};
use map::Map;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

fn main() {
    // let seed = 3;
    // let size = 2048;

    let fort = Source::simplex(0)
        .scale([0.05, 0.05])
        .fbm(4, 2.0, 2.0, 0.125)
        .mul(8.0);
    let nite = Source::simplex(1)
        .scale([0.05, 0.05])
        .fbm(4, 2.0, 2.0, 0.125)
        .mul(8.0);

    let mut image = RgbaImage::new(2048, 2048);
    let mut colors: HashMap<(u32, u32), Rgba<u8>> = HashMap::new();

    let template = image::open("map.png").unwrap();
    let dim = template.dimensions();

    for x in 0..dim.0 {
        for y in 0..dim.1 {
            let color = template.get_pixel(x, y);
            colors.insert((x, y), color);
        }
    }

    let seed: [u8; 32] = [0; 32];
    let mut rng = StdRng::from_seed(seed);

    for x in 0..2048 {
        for y in 0..2048 {
            // let gx = x / 16;
            // let gy = y / 16;
            let sx = fort.sample([x as f64, y as f64]);
            let sy = nite.sample([x as f64, y as f64]);
            // println!("{sx}, {sy}");

            let gx = ((x as f64 + sx) / 16.0) as u32;
            let gy = ((y as f64 + sy) / 16.0) as u32;

            if colors.contains_key(&(gx, gy)) {
                image.put_pixel(x as u32, y as u32, *colors.get(&(gx, gy)).unwrap());
            } else {
                let color = Rgba([255, 255, 255, 255]);
                colors.insert((gx, gy), color);
                image.put_pixel(x as u32, y as u32, color);
            }
        }
    }

    // let _ = fort.sample([0.2, 1.3]);

    // Visualizer::<2>::new([512, 512], &fort)
    // .write_to_file("fort.png")
    // .unwrap();

    let _ = image.save("squares.png");
}
