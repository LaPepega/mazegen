use maze::{enums::Direction, Maze, MazeCell};

mod maze;
fn main() {
    // All walls 5x5
    let m = Maze::generate(20, 20, (0, 0), (4, 4));
    m.print();
}
