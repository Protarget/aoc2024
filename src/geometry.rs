use std::{fmt::Display, ops::{Add, Mul, Sub}};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Point(pub i64, pub i64);

pub struct Grid<T> {
    pub size: Point,
    content: Box<[T]>
}

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

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self {
        Point(self.0 + other.0, self.1  + other.1)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, other: i64) -> Self {
        Point(self.0 * other, self.1 * other)
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, other: Direction) -> Self {
        self + other.offset()
    }
}

impl Sub<Direction> for Point {
    type Output = Point;

    fn sub(self, other: Direction) -> Self {
        self - other.offset()
    }
}


impl Point {
    pub fn in_bounds(self, size: Point) -> bool {
        self.0 >= 0 && self.0 < size.0 && self.1 >= 0 && self.1 < size.1
    }

    pub fn area(self) -> i64 {
        self.0 * self.1
    }

    pub fn taxicab_distance(self, other: Point) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }

    pub fn dot_product(self, other: Point) -> i64 {
        (self.0 * other.0) + (self.1 * other.1)
    }
}

impl <'a, T: Into<&'a str>> From<T> for Grid<char> {
    fn from(value: T) -> Self {
        let mut content = vec![];
        let mut width = 0;
        let mut height: i64 = 0;
        let string_content: &str = value.into();
        let mut x = 0;
        let mut y = 0;

        for c in string_content.chars() {
            match c {
                '\n' => {
                    width = width.max(x);
                    x = 0;
                    y += 1;
                }
                v => {
                    content.push(v);
                    x += 1;
                }
            }
        }

        height = y + 1;

        let slice_content = content.into_boxed_slice();

        Grid {
            size: Point(width, height),
            content: slice_content
        }
    }
}

impl <T> Grid<T> {
    pub fn get(&self, position: Point) -> Option<&T> {
        let index = self.calculate_index(position);
        index.map(|x| &self.content[x])
    }

    pub fn get_with_index(&self, position: Point) -> Option<(Point, &T)> {
        let index = self.calculate_index(position);
        index.map(|x| (position, &self.content[x]))
    }

    pub fn map<T2>(&self, f: fn(&T) -> T2) -> Grid<T2> {
        let new_content: Box<[T2]> = self.content.iter().map(f).collect();
        Grid {
            size: self.size,
            content: new_content
        }
    }

    pub fn find_all(&self, f: fn(&T) -> bool) -> impl Iterator<Item = (Point, &T)> + '_ {
        self.content.iter().enumerate().filter(move |(_, v)| f(v)).map(|(i, v)| (Point(i as i64 % self.size.0, i as i64 / self.size.0), v))
    }

    pub fn calculate_index(&self, position: Point) -> Option<usize> {
        if position.in_bounds(self.size) {
            Some((position.1 * self.size.0 + position.0) as usize)
        }
        else {
            None
        }
    }
}