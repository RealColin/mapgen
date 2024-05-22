pub struct Voronoi {
    seed: u64,
    square_size: u32,
}

impl Voronoi {
    pub fn init(seed: u64, square_size: u32) -> Self {
        Voronoi { seed, square_size }
    }
}
