mod continent;
mod map;
mod voronoi;

use std::collections::HashMap;

use image::{GenericImageView, Rgba, RgbaImage};

fn main() {
    let input = image::open("input.png").unwrap();

    let scale_factor = 16;
    let width = input.dimensions().0 * scale_factor;
    let height = input.dimensions().1 * scale_factor;

    let mut output = RgbaImage::new(width, height);

    let mut base_colors: HashMap<(u32, u32), Rgba<u8>> = HashMap::new();

    let mut get_base_color = |mut x: u32, mut y: u32| {
        x = x.clamp(0, width - 1);
        y = y.clamp(0, height - 1);

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
        let half = scale_factor / 2;

        let base = get_base_color(x, y);
        let left = get_base_color((x - px) - 1, y);
        let right = get_base_color((x + ((scale_factor - 1) - px)) + 1, y);
        let up = get_base_color(x, (y - py) - 1);
        let down = get_base_color(x, (y + ((scale_factor - 1) - py)) + 1);

        if px < half && left != base {
            if py < half && up != base {
                if px + py < half {
                    if left == up {
                        return left;
                    } else {
                        let corner = get_base_color((x - px) - 1, (y - py) - 1);

                        if corner == left || corner == up {
                            return corner;
                        }
                    }
                }
            } else if py > half && down != base {
                if px + py > half + (px * 2) {
                    if left == down {
                        return left;
                    } else {
                        let corner =
                            get_base_color((x - px) - 1, (y + ((scale_factor - 1) - py)) + 1);

                        if corner == left || corner == down {
                            return corner;
                        }
                    }
                }
            }
        }

        if px >= half && right != base {
            if py < half && up != base {
                if px + py >= half + (py * 2) {
                    if right == up {
                        return right;
                    } else {
                        let corner =
                            get_base_color((x + ((scale_factor - 1) - px)) + 1, (y - py) - 1);

                        if corner == right || corner == up {
                            return corner;
                        }
                    }
                }
            } else if py > half && down != base {
                if px + py >= half * 3 {
                    if right == down {
                        return right;
                    } else {
                        let corner = get_base_color(
                            (x + ((scale_factor - 1) - px)) + 1,
                            (y + ((scale_factor - 1) - py)) + 1,
                        );

                        if corner == right || corner == down {
                            return corner;
                        }
                    }
                }
            }
        }
        return base;
    };

    for x in 0..width {
        for y in 0..height {
            let color = get_color(x, y);

            output.put_pixel(x, y, color);
        }
    }

    let _ = output.save("output.png");
}
