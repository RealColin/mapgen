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
    let input = image::open("input.png").unwrap();

    let scale_factor = 4;
    let width = input.dimensions().0 * scale_factor;
    let height = input.dimensions().1 * scale_factor;

    let mut output = RgbaImage::new(width, height);

    let mut base_colors: HashMap<(u32, u32), Rgba<u8>> = HashMap::new();

    let mut get_base_color = |x: u32, y: u32| {
        if x >= width || y >= width {
            return Rgba([0, 0, 0, 255]);
        }

        if base_colors.contains_key(&(x, y)) {
            return *base_colors.get(&(x, y)).unwrap();
        } else {
            let color = input.get_pixel(x / scale_factor, y / scale_factor);
            base_colors.insert((x, y), color);
            return color;
        }
    };

    let mut get_color = |x: u32, y: u32| {
        let px = x % scale_factor;
        let py = y % scale_factor;

        let base = get_base_color(x, y);

        if px <= 1 && py <= 1 {
            let left = get_base_color((x - px) - 1, y);
            let up = get_base_color(x, (y - py) - 1);

            if left != base && up != base {
                if px == 1 && py == 1 {
                    return base;
                }

                if px == 0 {
                    return left;
                } else {
                    return up;
                }
            }
        } else if px >= 2 && py >= 2 {
            let right = get_base_color((x + (3 - px)) + 1, y);
            let down = get_base_color(x, (y + (3 - py)) + 1);

            if right != base && down != base {
                if px == 2 && py == 2 {
                    return base;
                }

                if px == 3 {
                    return right;
                } else {
                    return down;
                }
            }
        }

        return get_base_color(x, y);
    };

    for x in 0..width {
        for y in 0..height {
            let color = get_color(x, y);

            output.put_pixel(x, y, color);
        }
    }

    let _ = output.save("output.png");
}
