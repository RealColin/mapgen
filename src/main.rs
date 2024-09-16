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

    let fort = Source::simplex(0).scale([0.05, 0.05]).mul(4.0);
    let nite = Source::simplex(1).scale([0.05, 0.05]).mul(4.0);

    let mut image = RgbImage::new(512, 512);
    let mut colors: HashMap<(i64, i64), Rgb<u8>> = HashMap::new();

    let blue = Rgb([0, 0, 255]);
    colors.insert((0, 0), blue);
    colors.insert((0, 1), blue);
    colors.insert((0, 2), blue);
    colors.insert((0, 3), blue);
    colors.insert((1, 0), blue);
    colors.insert((1, 1), blue);
    colors.insert((1, 2), blue);
    colors.insert((1, 3), blue);
    colors.insert((1, 4), blue);
    colors.insert((2, 0), blue);
    colors.insert((2, 1), blue);
    colors.insert((2, 2), blue);
    colors.insert((2, 3), blue);
    colors.insert((2, 4), blue);
    colors.insert((3, 0), blue);
    colors.insert((3, 1), blue);
    colors.insert((3, 2), blue);
    colors.insert((3, 3), blue);
    colors.insert((3, 4), blue);
    colors.insert((3, 5), blue);

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
}
