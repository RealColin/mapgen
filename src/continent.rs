pub struct ContinentSettings {
    scale: u32,
}

impl ContinentSettings {
    pub fn init(scale: u32) -> Self {
        ContinentSettings { scale }
    }
}

pub struct Continent {
    center: (i64, i64),
}

impl Continent {
    pub fn init() -> Self {
        Continent { center: (0, 0) }
    }
}
