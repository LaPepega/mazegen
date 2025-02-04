use enums::{CellFlag, Direction};
use rand::seq::SliceRandom;
pub mod enums;
// A cell of a maze where wall configuration is
// represented as a u8 bitfield with a layout of
// 0b00000000
// ->udlrvsf
// where:
// u - upwards wall
// d - downwards wall
// l - left wall
// r - right wall
// v - visited
// s - start
// f - finish
// Yes, I am really fucking smart <3
// There's 1 extra bit use it as u please
#[derive(Debug, Clone)]
pub struct MazeCell(pub u8);

type Point = (usize, usize);

impl MazeCell {
    // Returns new [Cell] with bit at idx changed to b
    fn bit(&self, idx: usize, b: bool) -> Self {
        let mask = 0b1000_0000 >> idx;
        if b {
            Self(self.0 | mask)
        } else {
            Self(self.0 & !mask)
        }
    }

    // Changes bit at idx to b
    fn set_bit(&mut self, idx: usize, b: bool) {
        let mask = 0b1000_0000 >> idx;
        if b {
            self.0 = self.0 | mask;
        } else {
            self.0 = self.0 & !mask;
        }
    }

    pub fn as_str(&self) -> String {
        let mut s: String = "".to_string();
        if self.get_flag(CellFlag::Up) {
            s.push('u');
        } else {
            s.push(' ');
        }
        if self.get_flag(CellFlag::Down) {
            s.push('d');
        } else {
            s.push(' ');
        }
        if self.get_flag(CellFlag::Left) {
            s.push('l');
        } else {
            s.push(' ');
        }
        if self.get_flag(CellFlag::Right) {
            s.push('r');
        } else {
            s.push(' ');
        }
        s
    }

    fn get_bit(&self, idx: u8) -> bool {
        (self.0 & (0b1000_0000 >> idx)).count_ones() == 1
    }

    pub fn get_flag(&self, flag: CellFlag) -> bool {
        self.get_bit(flag as u8)
    }

    // Returns new [Cell] with changed flag
    pub fn flag(&self, f: CellFlag, b: bool) -> Self {
        self.bit(f as usize, b)
    }

    // Changes flag to b

    pub fn set_flag(&mut self, f: CellFlag, b: bool) {
        self.set_bit(f as usize, b);
    }

    // Constructs a [Cell] with specified parameters
    pub fn with_bits(
        up: bool,
        down: bool,
        left: bool,
        right: bool,
        visited: bool,
        start: bool,
        finish: bool,
    ) -> Self {
        // I honestly don't know if this is the best way, but at least it's readable
        MazeCell(0b0000_0000)
            .bit(0, up)
            .bit(1, down)
            .bit(2, left)
            .bit(3, right)
            .bit(4, visited)
            .bit(5, start)
            .bit(6, finish)
    }
}

pub struct Maze {
    pub layout: Vec<Vec<MazeCell>>,
    pub start: (usize, usize),
    pub finish: (usize, usize),
}

impl Maze {
    pub fn print(&self) {
        for row in self.layout.iter() {
            let u: Vec<String> = row.iter().map(|c| c.as_str()).collect();
            println!("{:?}", u)
        }
    }
    // Creates a new maze
    pub fn new(layout: Vec<Vec<MazeCell>>, start: Point, end: Point) -> Self {
        Self {
            layout,
            start,
            finish: end,
        }
    }

    fn neighbor_pos(pos: Point, dir: Direction) -> Option<Point> {
        match dir {
            Direction::Down => Some((pos.0, pos.1 + 1)), //+y
            Direction::Up => Some((pos.0, pos.1.checked_sub(1)?)), //-y
            Direction::Left => Some((pos.0.checked_sub(1)?, pos.1)), //-x
            Direction::Right => Some((pos.0 + 1, pos.1)), //+x
        }
    }

    // (direction to the cell, cell's position, cell object reference)

    fn neighbor(&self, pos: Point, dir: Direction) -> Option<(Direction, Point, &MazeCell)> {
        let np = Maze::neighbor_pos(pos, dir);
        Some((dir, np?, self.layout.get(np?.1)?.get(np?.0)?))
    }

    fn neighbor_mut(
        &mut self,
        pos: Point,
        dir: Direction,
    ) -> Option<(Direction, Point, &mut MazeCell)> {
        let np = Maze::neighbor_pos(pos, dir);
        Some((dir, np?, self.layout.get_mut(np?.1)?.get_mut(np?.0)?))
    }

    // I *think* this is ok

    fn all_neighbors(&self, pos: Point) -> Vec<Option<(Direction, Point, &MazeCell)>> {
        vec![
            Maze::neighbor(self, pos, Direction::Up),
            Maze::neighbor(self, pos, Direction::Down),
            Maze::neighbor(self, pos, Direction::Left),
            Maze::neighbor(self, pos, Direction::Right),
        ]
    }

    pub fn get_cell(&self, pos: Point) -> Option<&MazeCell> {
        Some(self.layout.get(pos.1)?.get(pos.0)?)
    }

    pub fn get_cell_mut(&mut self, pos: Point) -> Option<&mut MazeCell> {
        Some(self.layout.get_mut(pos.1)?.get_mut(pos.0)?)
    }

    fn valid_dirs(&self, pos: Point) -> Vec<Direction> {
        self.all_neighbors(pos)
            .iter()
            .cloned()
            .filter_map(|nbr| nbr) // Only leave existing neighbors and unwrap 'em
            .filter(|p| !p.2.get_flag(CellFlag::Visited)) // Which haven't been visited
            .map(|p| p.0) // Only need direction
            .collect()
    }

    pub fn generate(width: usize, height: usize, start: Point, end: Point) -> Self {
        let layout = vec![vec![MazeCell(0b1111_0000); width]; height];
        let mut maze = Maze::new(layout, start, end);
        let dirs = maze.valid_dirs(start);

        maze.generate_step(
            start,
            dirs.choose(&mut rand::thread_rng())
                .expect("No dirs on start")
                .clone(),
        );
        maze
    }

    // IT FUCKING WOOOOOOOOOOOOORKS LES'T GOOOOOOOOOO
    pub fn generate_step(&mut self, pos: Point, dir: Direction) {
        // Check if current position is out of bounds
        let current_cell: &mut MazeCell = self.get_cell_mut(pos).expect("Bad pos");

        current_cell.set_flag(CellFlag::Visited, true);

        // Remove c->n wall
        current_cell.set_flag(dir.into(), false);

        let (_, next_position, next_cell) = self.neighbor_mut(pos, dir).expect("Invalid next pos");

        // Remove n->c wall
        next_cell.set_flag(dir.opposite().into(), false);

        next_cell.set_flag(CellFlag::Visited, true);

        // Choose next victim
        let mut possible_dirs: Vec<Direction> = self.valid_dirs(next_position);

        while !possible_dirs.is_empty() {
            self.generate_step(
                next_position,
                possible_dirs
                    .choose(&mut rand::thread_rng())
                    .expect("No dirs")
                    .clone(),
            );
            // Backtracking

            possible_dirs = self.valid_dirs(next_position);
        }
    }
}
