mod continent;
mod map;
mod voronoi;

use std::{collections::HashMap, str::FromStr};

use image::{DynamicImage, GenericImage, GenericImageView, Rgb, RgbImage, Rgba, RgbaImage};
use libnoise::{Generator, Source, Visualizer};
use map::Map;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use voronoi::Voronoi;

fn main() {
    let vor = Voronoi::init(3, 16, 2);
    let template = image::open("map.png").unwrap();

    let mut image = RgbaImage::new(2048, 2048);
    let mut colors: HashMap<(i64, i64), Rgba<u8>> = HashMap::new();

    let fort = Source::worley(0)
        .scale([0.005, 0.005])
        .fbm(1, 1.0, 1.0, 1.0)
        .mul(1.0);
    let nite = Source::perlin(1)
        .scale([0.05, 0.05])
        .fbm(8, 1.0, 2.0, 0.25)
        .mul(16.0);

    Visualizer::<2>::new([2048, 2048], &fort)
        .write_to_file("fort.png")
        .unwrap();

    // let mut get_base_color = |x: f64, y: f64| {
    //     let ox = fort.sample([x + 0.01, y + 0.01]);
    //     let oy = nite.sample([x + 0.01, y + 0.01]);

    //     let color = template.get_pixel(
    //         ((x + ox) as u32 / 16).clamp(0, 127),
    //         ((y + oy) as u32 / 16).clamp(0, 127),
    //     );
    //     return color;

    //     // let site = vor.nearest_site((x + ox) as i64, (y + oy) as i64);
    //     // if colors.contains_key(&site) {
    //     //     let color = *colors.get(&site).unwrap();
    //     //     return color;
    //     // } else {
    //     //     let site_box = ((site.0 / 16).clamp(0, 127), (site.1 / 16).clamp(0, 127));
    //     //     let color = template.get_pixel(site_box.0 as u32, site_box.1 as u32);

    //     //     colors.insert(site, color);
    //     //     return color;
    //     // }
    // };

    // let mut cache: HashMap<(i64, i64), Rgba<u8>> = HashMap::new();

    // let mut get_color = |x: f64, y: f64| {
    //     let mut counts: HashMap<Rgba<u8>, u8> = HashMap::new();

    //     for xo in -4..=4 {
    //         for yo in -4..=4 {
    //             let color;
    //             if cache.contains_key(&(x as i64 + xo as i64, y as i64 + yo as i64)) {
    //                 color = *cache
    //                     .get(&(x as i64 + xo as i64, y as i64 + yo as i64))
    //                     .unwrap();
    //             } else {
    //                 color = get_base_color(x + xo as f64, y + yo as f64);
    //                 cache.insert((x as i64 + xo as i64, y as i64 + yo as i64), color);
    //             }

    //             let count = *counts.get(&color).unwrap_or(&0);

    //             counts.insert(color, count + 1);
    //         }
    //     }

    //     let mut highest = 0;
    //     let mut ret = Rgba([0, 0, 0, 0]);

    //     for color in counts {
    //         if highest < color.1 {
    //             highest = color.1;
    //             ret = color.0;
    //         } else if highest == color.1 {
    //             let base = get_base_color(x, y);
    //             if base == color.0 {
    //                 ret = color.0;
    //             }
    //         }
    //     }

    //     // println!("{x}, {y}, {}, {}, {}", ret[0], ret[1], ret[2]);

    //     ret
    // };

    // for x in 0..2048 {
    //     for y in 0..2048 {
    //         let color = get_color(x as f64, y as f64);

    //         image.put_pixel(x, y, color);

    //         // let ox = fort.sample([x as f64 + 0.01, y as f64 + 0.01]);
    //         // let oy = nite.sample([x as f64 + 0.01, y as f64 + 0.01]);

    //         // let site = vor.nearest_site((x as f64 + ox) as i64, (y as f64 + oy) as i64);
    //         // if colors.contains_key(&site) {
    //         //     let color = colors.get(&site).unwrap();
    //         //     image.put_pixel(x as u32, y as u32, *color);
    //         // } else {
    //         //     let site_box = (site.0 / 16, site.1 / 16);
    //         //     let color;

    //         //     if site_box.0 > 127 || site_box.1 > 127 || site_box.0 < 0 || site_box.1 < 0 {
    //         //         color = Rgba([0, 255, 0, 255]);
    //         //     } else {
    //         //         color = template.get_pixel(site_box.0 as u32, site_box.1 as u32);
    //         //     }

    //         //     colors.insert(site, color);
    //         //     image.put_pixel(x as u32, y as u32, color);
    //         // }
    //     }
    // }

    // let _ = image.save("vorsq.png");
}
