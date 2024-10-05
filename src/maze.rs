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

    // FIXME: no idea if this works
    fn get_bit(&self, idx: u8) -> bool {
        (self.0 & (1 << idx)) == 1
    }

    pub fn get_flag(&self, flag: CellFlag) -> bool {
        self.get_bit(flag as u8)
    }

    // Returns new [Cell] with changed flag
    pub fn flag(&self, f: CellFlag, b: bool) -> Self {
        self.bit(f as usize, b)
    }

    // Changes flag to b
    // TODO: Maybe return self too?
    pub fn set_flag(&mut self, f: CellFlag, b: bool) {
        self.set_bit(f as usize, b);
    }

    // Constructs a [Cell] with specified parameters
    pub fn with_flags(
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
    // Creates a new maze
    pub fn new(layout: Vec<Vec<MazeCell>>, start: Point, end: Point) -> Self {
        Self {
            layout,
            start,
            finish: end,
        }
    }

    fn neighbor_pos(pos: Point, dir: Direction) -> Option<Point> {
        // TODO: overflows
        match dir {
            Direction::Down => Some((pos.0, pos.1 + 1)),
            Direction::Up => match pos.1.checked_sub(1) {
                Some(p) => Some((pos.0, p)),
                None => None,
            },
            Direction::Right => match pos.0.checked_sub(1) {
                Some(p) => Some((p, pos.1)),
                None => None,
            },
            Direction::Left => Some((pos.0 + 1, pos.1)),
        }
    }

    // (direction to the cell, cell's position, cell object reference)
    // FIXME: there can be no neighbors at pos
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
    // FIXME: there can be no neighbors at pos
    fn all_neighbors(&self, pos: Point) -> Vec<Option<(Direction, Point, &MazeCell)>> {
        vec![
            Maze::neighbor(self, pos, Direction::Up),
            Maze::neighbor(self, pos, Direction::Down),
            Maze::neighbor(self, pos, Direction::Left),
            Maze::neighbor(self, pos, Direction::Right),
        ]
    }

    // Returns the position generator moved to
    pub fn generate(&mut self, pos: Point, dir: Direction) -> Point {
        // Check if current position is out of bounds
        let current_cell: &mut MazeCell = self
            .layout
            .get_mut(pos.1)
            .expect("YPos is fucked up")
            .get_mut(pos.0)
            .expect("XPos is fucked up");

        // Mark visited
        current_cell.set_flag(CellFlag::Visited, true);

        // Remove c->n wall
        current_cell.set_flag(dir.into(), false);

        let (_, next_position, next_cell) = match self.neighbor_mut(pos, dir) {
            Some(n) => n,
            None => return pos,
        };

        // Remove n->c wall
        next_cell.set_flag(dir.opposite().into(), false);
        // Choose next victim
        let possible: Vec<(Direction, Point, &MazeCell)> = self
            .all_neighbors(pos)
            .iter()
            .cloned()
            .filter_map(|nbr| nbr)
            .collect();

        if !possible.is_empty() {
            self.generate(
                next_position,
                possible
                    .choose(&mut rand::thread_rng())
                    .expect("not_visited isn't empty so smthn deeply fucked up if u see this")
                    .0,
            );
        }
        next_position
    }
}
