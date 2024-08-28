use std::collections::HashMap;

use image::{self, Rgb, RgbImage};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

pub struct Voronoi {
    square_size: u32,
    sites: Vec<(u32, u32)>,
}

impl Voronoi {
    pub fn init(seed: u64, square_size: u32) -> Self {
        let mut sites: Vec<(u32, u32)> = Vec::new();
        for x in 0..square_size {
            for y in 0..square_size {
                sites.push((x, y));
            }
        }

        // Shuffle the sites so that each seed can have a different output image
        let mut _seed: [u8; 32] = [0; 32];
        let seed_bytes = seed.to_be_bytes();
        for index in 0..8 {
            _seed[index] = seed_bytes[index];
        }

        let mut rng = StdRng::from_seed(_seed);
        sites.shuffle(&mut rng);

        Voronoi { square_size, sites }
    }

    /* Get the site nearest to the global position (x, y). This is the cell that the position belongs to. */
    pub fn nearest_site(&self, x: i64, y: i64) -> (i64, i64) {
        /* Grid coordinates that the (x, y) pos belongs to */
        let gx = x / (self.square_size as i64);
        let gy = y / (self.square_size as i64);

        let mut min_dist = f64::INFINITY;
        let mut site: (i64, i64) = (0, 0);

        for cx in -2..=2 {
            for cy in -2..=2 {
                let temp = self.site_at(gx + cx, gy + cy);
                let dist = self.dist((x, y), temp, 3);

                if dist < min_dist {
                    min_dist = dist;
                    site = temp;
                }
            }
        }

        site
    }

    pub fn gen_image(&self, path: String, size: u32) {
        let mut image = RgbImage::new(size, size);

        let mut colors: HashMap<(i64, i64), Rgb<u8>> = HashMap::new();

        let seed: [u8; 32] = [0; 32];
        let mut rng = StdRng::from_seed(seed);

        for x in 0..size {
            for y in 0..size {
                let site = self.nearest_site(x as i64, y as i64);
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

        let _ = image.save(path);
    }

    fn dist(&self, point: (i64, i64), other: (i64, i64), exp: u32) -> f64 {
        ((point.0 - other.0).abs().pow(exp) as f64 + (point.1 - other.1).abs().pow(exp) as f64)
            .powf(1.0 / exp as f64)
    }

    /* Get the global coords of the site belonging to the square */
    fn site_at(&self, gx: i64, gy: i64) -> (i64, i64) {
        // let index = ((53 + Voronoi::hash(gx)) * 53 + Voronoi::hash(gy)) as usize % self.sites.len();
        let index = (((53 + Voronoi::hash(gx))
            .wrapping_mul(53)
            .wrapping_add(Voronoi::hash(gy))) as usize)
            % self.sites.len();

        // the coords in the sites vec are all relative to the square rather than being global coords
        let local = *self.sites.get(index as usize).unwrap();

        // in order to get the global position, we must offset by the distance (in global coords) that the square is from the origin
        let site = (
            local.0 as i64 + (gx * self.square_size as i64) as i64,
            local.1 as i64 + (gy * self.square_size as i64) as i64,
        );

        site
    }

    fn hash(mut num: i64) -> i64 {
        num = (!num).wrapping_add(num << 21);
        num = num ^ (num >> 24);
        num = num.wrapping_add((num << 3) + (num << 8));
        num = num ^ (num >> 14);
        num = num.wrapping_add((num << 2) + (num << 4));
        num = num ^ (num >> 28);
        num = num.wrapping_add(num << 31);
        num
    }
}
