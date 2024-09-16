pub struct ContinentSettings {
    scale: u32,
}

impl ContinentSettings {
    pub fn init(scale: u32) -> Self {
        ContinentSettings { scale }
    }
}

use image::Rgb;

pub struct Continent {
    center: (i64, i64),
    x_radius: u64,
    y_radius: u64,
}

impl Continent {
    pub fn init(center: (i64, i64), x_radius: u64, y_radius: u64) -> Continent {
        Continent {
            center,
            x_radius,
            y_radius,
        }
    }

    pub fn sample(&self, x: i64, y: i64) -> Rgb<u8> {
        // let temp_x: f64 = (x - self.center.0).pow(2) as f64 / self.x_radius.pow(2) as f64;
        // let temp_y: f64 = (y - self.center.1).pow(2) as f64 / self.y_radius.pow(2) as f64;

        let dist = self.dist_to_center(x, y);

        if dist <= 1.0 {
            return Rgb([0, 255, 0]);
        } else {
            return Rgb([0, 0, 255]);
        }
    }

    /*
     * 0.0 is at continent center
     * 1.0 is furthest distance from continent center within continent
     * greater than 1.0 is outside the continent
     */
    fn dist_to_center(&self, x: i64, y: i64) -> f64 {
        // distance will be determined by whether the point is within the elipse specified by the radius and center

        return ((x - self.center.0).pow(2) as f64 / self.x_radius.pow(2) as f64)
            + ((y - self.center.1).pow(2) as f64 / self.y_radius.pow(2) as f64);
    }
}

// anything x or y = 0-43 and 85-128 is blue
// for x = 44, anything other than y=64 is blue
