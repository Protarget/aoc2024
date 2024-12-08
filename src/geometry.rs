use std::ops::{Add, Sub};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Point(pub i64, pub i64);

impl Direction {
    pub fn offset(self) -> Point {
        match self {
            Direction::North => Point(0, -1),
            Direction::East => Point(1, 0),
            Direction::South => Point(0, 1),
            Direction::West => Point(-1, 0),
        }
    }

    pub fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        }
    }

    pub fn to_bit_mask(self) -> u8 {
        match self {
            Direction::North => 1,
            Direction::East => 2,
            Direction::South => 4,
            Direction::West => 8
        }
    }
}


impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self {
        Point(self.0 + other.0, self.1  + other.1)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl Point {
    pub fn in_bounds(self, size: Point) -> bool {
        self.0 >= 0 && self.0 < size.0 && self.1 >= 0 && self.1 < size.1
    }

    pub fn step(self, direction: Direction) -> Point {
        self + direction.offset()
    }

    pub fn area(self) -> i64 {
        self.0 * self.1
    }
}