mod voronoi;

use voronoi::Voronoi;

fn main() {
    let vor = Voronoi::init(3, 64);
    vor.gen_image(String::from("fortnite.png"), 1024);

    println!("Hello, world!");
}
