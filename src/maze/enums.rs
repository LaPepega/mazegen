pub enum CellFlag {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    Visited = 4,
    Start = 5,
    Finish = 6,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn random() -> Self {
        Direction::try_from(rand::random::<u8>() % 3).expect("rand dir fucked up")
    }
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::Up),
            1 => Ok(Direction::Down),
            2 => Ok(Direction::Left),
            3 => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl Into<CellFlag> for Direction {
    fn into(self) -> CellFlag {
        match self {
            Direction::Up => CellFlag::Up,
            Direction::Down => CellFlag::Down,
            Direction::Left => CellFlag::Left,
            Direction::Right => CellFlag::Right,
        }
    }
}
