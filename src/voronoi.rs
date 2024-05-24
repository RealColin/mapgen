pub struct Voronoi {
    seed: u64,
    square_size: u32,
}

impl Voronoi {
    pub fn init(seed: u64, square_size: u32) -> Self {
        Voronoi { seed, square_size }
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

    /* Get the site belonging to the square */
    fn site_at(&self, gx: i64, gy: i64) -> (i64, i64) {
        // TODO implement this function

        (0, 0)
    }
}
