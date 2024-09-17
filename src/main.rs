mod continent;
mod map;
mod voronoi;

use std::{collections::HashMap, str::FromStr};

use image::{GenericImage, GenericImageView, Rgb, RgbImage, Rgba, RgbaImage};
use libnoise::{Generator, Source, Visualizer};
use map::Map;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use voronoi::Voronoi;

fn main() {
    let template = image::open("map.png").unwrap();

    let scale_factor = 16.0;

    let (width, height) = template.dimensions();
    let scaled_width = (width as f64 * scale_factor) as u32;
    let scaled_height = (height as f64 * scale_factor) as u32;

    let mut image = RgbaImage::new(scaled_width, scaled_height);

    for x in 0..scaled_width {
        for y in 0..scaled_height {
            let gx = x / 16;
            let gy = y / 16;

            let color = template.get_pixel(gx, gy);
            image.put_pixel(x, y, color);
        }
    }

    let _ = image.save("output.png");

    // let vor = Voronoi::init(0, 16, 2);
    // let template = image::open("map.png").unwrap();

    // let mut image = RgbaImage::new(2048, 2048);
    // let mut colors: HashMap<(i64, i64), Rgba<u8>> = HashMap::new();

    // let fort = Source::perlin(0)
    //     .scale([0.05, 0.05])
    //     .fbm(8, 2.0, 2.0, 0.125)
    //     .mul(8.0);
    // let nite = Source::perlin(1)
    //     .scale([0.05, 0.05])
    //     .fbm(8, 2.0, 2.0, 0.125)
    //     .mul(8.0);

    // // Visualizer::<2>::new([2048, 2048], &fort)
    // //     .write_to_file("fort.png")
    // //     .unwrap();

    // let clamp = |val: f64| val.max(-4.0).min(4.0);

    // for x in 0..2048 {
    //     for y in 0..2048 {
    //         let ox = clamp(fort.sample([x as f64 + 0.01, y as f64 + 0.01]));
    //         let oy = clamp(nite.sample([x as f64 + 0.01, y as f64 + 0.01]));

    //         let site = vor.nearest_site((x as f64 + ox) as i64, (y as f64 + oy) as i64);
    //         if colors.contains_key(&site) {
    //             let color = colors.get(&site).unwrap();
    //             image.put_pixel(x as u32, y as u32, *color);
    //         } else {
    //             let site_box = (site.0 / 16, site.1 / 16);
    //             let color;

    //             if site_box.0 > 127 || site_box.1 > 127 || site_box.0 < 0 || site_box.1 < 0 {
    //                 color = Rgba([0, 255, 0, 255]);
    //             } else {
    //                 color = template.get_pixel(site_box.0 as u32, site_box.1 as u32);
    //             }

    //             colors.insert(site, color);
    //             image.put_pixel(x as u32, y as u32, color);
    //         }
    //     }
    // }

    // let _ = image.save("vorsq.png");
}
