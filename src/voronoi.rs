pub struct Voronoi {
    seed: u64,
    square_size: u32,
}

impl Voronoi {
    pub fn init(seed: u64, square_size: u32) -> Self {
        Voronoi { seed, square_size }
    }

    /* Get the cell that the global pos (x, y) belongs to */
    pub fn get_cell_at(&self, x: i64, y: i64) -> (i64, i64) {
        // TODO implement this function
        (0, 0)
    }

    /* Get the site belonging to the square */
    fn get_site_at(&self, gx: i64, gy: i64) {
        // TODO implement this function
    }
}
