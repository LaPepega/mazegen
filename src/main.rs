#![recursion_limit = "1000"]
use maze::Maze;
use std::env;

mod maze;
fn main() {
    let arg: Vec<String> = env::args().collect();
    let w: usize = arg
        .get(1)
        .expect("No width specified")
        .parse()
        .expect("Invalid width");
    let h: usize = arg
        .get(2)
        .expect("No height specified")
        .parse()
        .expect("Invalid height");
    let m = Maze::generate(w, h, (0, 0), (w - 1, h - 1));

    m.print();
}
