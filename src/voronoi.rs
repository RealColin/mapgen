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

        // shuffle sites with a random function seeded with seed

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
                let temp = self.site_at(gx, gy);
                let dist = ((x - temp.0).pow(2) as f64 + (y - temp.1).pow(2) as f64).sqrt();

                if dist < min_dist {
                    min_dist = dist;
                    site = temp;
                }
            }
        }

        site
    }

    pub fn nearest_two_sites(&self, x: i64, y: i64) -> ((i64, i64), (i64, i64)) {
        // TODO implement this function
        ((0, 0), (0, 0))
    }

    pub fn gen_image(&self, path: String, size: u32) {
        // TODO implement this function
    }

    /* Get the global coords of the site belonging to the square */
    fn site_at(&self, gx: i64, gy: i64) -> (i64, i64) {
        let index = ((53 + Voronoi::hash(gx)) * 53 + Voronoi::hash(gy)) as usize % self.sites.len();

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
