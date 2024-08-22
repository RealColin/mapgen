mod voronoi;

use std::{collections::HashMap, time::Instant};

use image::{Rgb, RgbImage};
use libnoise::*;
use voronoi::Voronoi;

fn main() {
    let seed = 3;
    let size = 2048;

    // Noise and randomness
    let sim_x = Source::simplex(seed & 171)
        .scale([0.5])
        .fbm(8, 0.05, 2.0, 0.5)
        .mul(4.5);
    let sim_y = Source::simplex(seed & 73)
        .scale([0.5])
        .fbm(8, 0.05, 2.0, 0.5)
        .mul(4.5);
    let sim_color = Source::worley(seed & 17)
        .scale([0.002, 0.002])
        .fbm(4, 1.0, 2.0, 0.5)
    //    .mul(4.5)
    ;
    let vor = Voronoi::init(seed, 64);

    // Image generation stuff
    let mut image = RgbImage::new(size, size);
    let mut colors: HashMap<(i64, i64), Rgb<u8>> = HashMap::new();

    let start = Instant::now();

    // Filling the Image
    for x in 0..size {
        for y in 0..size {
            let jitter_x: i64 = sim_x.sample([x as f64]).round() as i64;
            let jitter_y: i64 = sim_y.sample([y as f64]).round() as i64;

            let site = vor.nearest_site(x as i64 + jitter_x, y as i64 + jitter_y);
            if colors.contains_key(&site) {
                image.put_pixel(x, y, *colors.get(&site).unwrap());
            } else {
                let fort = sim_color.sample([site.0 as f64, site.1 as f64]);

                let color;

                if fort < 0.0 {
                    color = Rgb([0, 255, 0]);
                } else {
                    color = Rgb([0, 0, 255]);
                }

                colors.insert(site, color);
                image.put_pixel(x, y, color);
            }
        }
    }

    let duration = start.elapsed();
    println!("Elapsed time for algorithm: {:?}", duration);

    let _ = image.save("fuzzy.png");

    Visualizer::<2>::new([2048, 2048], &sim_color)
        .write_to_file("noise.png")
        .unwrap();

    // vor.gen_image(String::from("image.png"), 1024);
}
