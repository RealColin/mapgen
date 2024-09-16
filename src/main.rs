mod continent;
mod map;
mod voronoi;

use std::collections::HashMap;

use image::{GenericImage, Rgb, RgbImage};
use libnoise::{Generator, Source, Visualizer};
use map::Map;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

fn main() {
    // let seed = 3;
    // let size = 2048;

<<<<<<< HEAD
    // Noise and randomness
    // let sim_x = Source::simplex(seed & 171)
    //     .scale([0.5])
    //     .fbm(8, 0.05, 2.0, 0.5)
    //     .mul(4.5);
    // let sim_y = Source::simplex(seed & 73)
    //     .scale([0.5])
    //     .fbm(8, 0.05, 2.0, 0.5)
    //     .mul(4.5);
    // let sim_color = Source::worley(seed & 17)
    //     .scale([0.002, 0.002])
    //     .fbm(4, 1.0, 2.0, 0.5)
    // //    .mul(4.5)
    // ;
    let vor = Voronoi::init(seed, 64);

    // // Image generation stuff
    // let mut image = RgbImage::new(size, size);
    // let mut colors: HashMap<(i64, i64), Rgb<u8>> = HashMap::new();

    // let start = Instant::now();

    // // Filling the Image
    // for x in 0..size {
    //     for y in 0..size {
    //         let jitter_x: i64 = sim_x.sample([x as f64]).round() as i64;
    //         let jitter_y: i64 = sim_y.sample([y as f64]).round() as i64;

    //         let site = vor.nearest_site(x as i64 + jitter_x, y as i64 + jitter_y);
    //         if colors.contains_key(&site) {
    //             image.put_pixel(x, y, *colors.get(&site).unwrap());
    //         } else {
    //             let fort = sim_color.sample([site.0 as f64, site.1 as f64]);

    //             let color;

    //             if fort < 0.0 {
    //                 color = Rgb([0, 255, 0]);
    //             } else {
    //                 color = Rgb([0, 0, 255]);
    //             }

    //             colors.insert(site, color);
    //             image.put_pixel(x, y, color);
    //         }
    //     }
    // }

    // let duration = start.elapsed();
    // println!("Elapsed time for algorithm: {:?}", duration);

    // let _ = image.save("fuzzy.png");

    // Visualizer::<2>::new([2048, 2048], &sim_color)
    //     .write_to_file("noise.png")
    //     .unwrap();

    vor.gen_image(String::from("image.png"), 1024);
=======
    // let map = Map::init(seed, size);
    // map.gen_image(String::from("map.png"));

    // let sim_x = Source::simplex(seed & 171)
    //     .scale([0.5])
    //     .fbm(8, 0.05, 2.0, 0.5)
    //     .mul(4.5);

    let fort = Source::simplex(0).scale([0.05, 0.05]).mul(4.0);
    let nite = Source::simplex(1).scale([0.05, 0.05]).mul(4.0);

    let mut image = RgbImage::new(512, 512);
    let mut colors: HashMap<(i64, i64), Rgb<u8>> = HashMap::new();

    let blue = Rgb([0, 0, 255]);
    colors.insert((0, 0), blue);

    let seed: [u8; 32] = [0; 32];
    let mut rng = StdRng::from_seed(seed);

    for x in 0..512 {
        for y in 0..512 {
            // let gx = x / 16;
            // let gy = y / 16;
            let sx = fort.sample([x as f64, y as f64]);
            let sy = nite.sample([x as f64, y as f64]);
            // println!("{sx}, {sy}");

            let gx: i64 = ((x as f64 + sx) / 16.0) as i64;
            let gy: i64 = ((y as f64 + sy) / 16.0) as i64;

            if colors.contains_key(&(gx, gy)) {
                image.put_pixel(x as u32, y as u32, *colors.get(&(gx, gy)).unwrap());
            } else {
                let color = Rgb([0, 255, 0]);
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
>>>>>>> 147ec49 (yeah)
}
// HOW TO USE VISUALIZER ON NOISE
// Visualizer::<2>::new([2048, 2048], &sim_color)
//     .write_to_file("noise.png")
//     .unwrap();
