use image::{GenericImage, Rgb, RgbImage};

pub struct Map {
    map_size: u32,
}

impl Map {
    pub fn init(map_size: u32) -> Self {
        Map { map_size }
    }

    fn sample(&self, x: u32, y: u32) -> Rgb<u8> {
        Rgb([255, 255, 255])
    }

    pub fn gen_image(&self, path: String) {
        let mut image = RgbImage::new(self.map_size, self.map_size);

        for x in 0..self.map_size {
            for y in 0..self.map_size {
                let color = self.sample(x, y);
                image.put_pixel(x, y, color);
            }
        }

        let _ = image.save(path);
    }
}
