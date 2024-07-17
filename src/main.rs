mod voronoi;

use voronoi::Voronoi;

fn main() {
    let vor = Voronoi::init(0, 16);
    vor.gen_image(String::from("sigma.png"), 256);

    println!("Hello, world!");
}
