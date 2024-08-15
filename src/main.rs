mod voronoi;

use std::collections::HashMap;

use image::{Rgb, RgbImage};
use libnoise::*;
use rand::{rngs::StdRng, Rng, SeedableRng};
use voronoi::Voronoi;

fn yeah(seed: u64) -> [u8; 32] {
    let bytes = seed.to_be_bytes();
    let mut ret: [u8; 32] = [0; 32];
    ret[..8].copy_from_slice(&bytes);
    ret
}

fn main() {
    let seed = 3;
    let size = 2048;
    let temp: [u8; 32] = [0; 32];

    // Noise and randomness
    let sim_x = Source::simplex(seed & 171).fbm(8, 0.05, 2.0, 0.5).mul(4.5);
    let sim_y = Source::simplex(seed & 73).fbm(8, 0.05, 2.0, 0.5).mul(4.5);
    // let sim_color = Source::simplex(seed & 17).fbm(8, 0.05, 2.0, 0.5).mul(4.5);
    let vor = Voronoi::init(seed, 64);
    let mut rng = StdRng::from_seed(temp);

    // Image generation stuff
    let mut image = RgbImage::new(size, size);
    let mut colors: HashMap<(i64, i64), Rgb<u8>> = HashMap::new();

    // Filling the Image
    for x in 0..size {
        for y in 0..size {
            let jitter_x: i64 = sim_x.sample([x as f64]).round() as i64;
            let jitter_y: i64 = sim_y.sample([y as f64]).round() as i64;

            let site = vor.nearest_site(x as i64 + jitter_x, y as i64 + jitter_y);
            if colors.contains_key(&site) {
                image.put_pixel(x, y, *colors.get(&site).unwrap());
            } else {
                let r: u8 = rng.gen();
                let g: u8 = rng.gen();
                let b: u8 = rng.gen();

                let color = Rgb([r, g, b]);
                colors.insert(site, color);
                image.put_pixel(x, y, color);
            }
        }
    }

    let _ = image.save("fuzzy.png");
    // vor.gen_image(String::from("image.png"), 1024);
}
