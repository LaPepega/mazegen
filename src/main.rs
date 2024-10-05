use maze::{enums::Direction, Maze, MazeCell};

mod maze;
fn main() {
    // All walls 5x5
    let l = vec![vec![MazeCell(0b1111_0000); 5]; 5];
    // Upper-left corner to lower-right corner
    let mut m = Maze::new(l, (0, 0), (4, 4));

    m.generate((0, 0), Direction::Down);
    for row in m.layout.iter() {
        let u: Vec<u8> = row.iter().map(|c| c.0).collect();
        println!("{:?}", u)
    }
}
