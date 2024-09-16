<<<<<<< HEAD
use std::time::Instant;

use image::{GenericImage, Rgb, RgbImage};
=======
use image::{Rgb, RgbImage};

use crate::continent::*;
>>>>>>> defab78 (keep pls)

pub struct Map {
    seed: i64,
    map_size: u32,
    continent: Continent,
}

impl Map {
    pub fn init(seed: i64, map_size: u32) -> Self {
<<<<<<< HEAD
        Map { seed, map_size }

        // determine continent grid size from input
        // generate continent(s)
=======
        let continent = Continent::init(((map_size / 2) as i64, (map_size / 2) as i64), 200, 200);
        Map {
            seed,
            map_size,
            continent,
        }
>>>>>>> defab78 (keep pls)
    }

    fn sample(&self, x: u32, y: u32) -> Rgb<u8> {
        self.continent.sample(x as i64, y as i64)
    }

    pub fn gen_image(&self, path: String) {
        let mut image = RgbImage::new(self.map_size, self.map_size);

        println!("Generating map image at path: {:?}", path);
        let start = Instant::now();

        for x in 0..self.map_size {
            for y in 0..self.map_size {
                let color = self.sample(x, y);
                image.put_pixel(x, y, color);
            }
        }

        let elapsed = start.elapsed();
        println!("Map image generated in: {:?}", elapsed);

        let _ = image.save(path);
    }
}
