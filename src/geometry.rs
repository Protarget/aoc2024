use std::{collections::HashSet, fmt::Display, ops::{Add, Div, Mul, Rem, Sub}};

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

#[derive(Debug)]
pub struct DirectionMap<T> {
    north: T,
    east: T,
    south: T,
    west: T
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

impl Mul<Point> for Point {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Div<Point> for Point {
    type Output = Point;

    fn div(self, rhs: Point) -> Self::Output {
        Point(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<i64> for Point {
    type Output = Point;

    fn div(self, rhs: i64) -> Self::Output {
        Point(self.0 / rhs, self.1 / rhs)
    }
}

impl Rem<Point> for Point {
    type Output = Point;

    fn rem(self, rhs: Point) -> Self::Output {
        Point(self.0 % rhs.0, self.1 % rhs.1)
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

    pub fn wrap(self, boundary: Point) -> Point {
        let mut wrapped = self % boundary;

        if wrapped.0 < 0 {
            wrapped.0 = boundary.0 + wrapped.0;
        }

        if wrapped.1 < 0 {
            wrapped.1 = boundary.1 + wrapped.1;
        }

        wrapped
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

        let height = y + 1;

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

    pub fn iter(&self) -> impl Iterator<Item = (Point, &T)> + '_ {
        self.content.iter().enumerate().map(|(i, v)| (Point(i as i64 % self.size.0, i as i64 / self.size.0), v))
    }

    pub fn flood_fill(&self, position: Point, f: impl Fn(&T) -> bool) -> Vec<(Point, DirectionMap<bool>)> {
        let mut result: Vec<(Point, DirectionMap<bool>)> = vec![];
        let mut visited: HashSet<Point> = HashSet::new();
        let mut visit_stack = vec![position];

        // Special case for initial not matching flood fill condition
        let initial_value = self.get(position);

        if initial_value.is_none() || !f(initial_value.unwrap()) {
            return result;
        }

        while !visit_stack.is_empty() {
            let visit_position = visit_stack.pop().unwrap();

            if visit_position.in_bounds(self.size) && !visited.contains(&visit_position) {
                let north_neighbor = self.get_with_index(visit_position + Direction::North);
                let east_neighbor = self.get_with_index(visit_position + Direction::East);
                let south_neighbor = self.get_with_index(visit_position + Direction::South);
                let west_neighbor = self.get_with_index(visit_position + Direction::West);

                let neighbors = [
                    (Direction::North, north_neighbor),
                    (Direction::East, east_neighbor),
                    (Direction::South, south_neighbor),
                    (Direction::West, west_neighbor)
                ];

                let valid_neighbors: Vec<(Direction, (Point, &T))> = neighbors
                    .iter()
                    .filter(|(_, x)| x.is_some())
                    .map(|(d, x)| (*d, x.unwrap()))
                    .filter(|(_, (_, x))| f(x))
                    .collect();

                let mut direction_map = DirectionMap::new(false);

                for neighbor in valid_neighbors {
                    direction_map.set(neighbor.0, true);
                    visit_stack.push(neighbor.1.0);
                }

                result.push((visit_position, direction_map));
                visited.insert(visit_position);
            }
        }

        result
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

impl <T: Clone> Grid<T> {
    pub fn new(size: Point, value: T) -> Grid<T> {
        let content = vec![value; (size.0 * size.1) as usize];
        Grid {
            size,
            content: content.into_boxed_slice()
        }
    }
}

impl <T> DirectionMap<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        [&self.north, &self.east, &self.south, &self.west].into_iter()
    }

    pub fn get(&self, direction: Direction) -> &T {
        match direction {
            Direction::North => &self.north,
            Direction::East => &self.east,
            Direction::South => &self.south,
            Direction::West => &self.west
        }
    }

    pub fn set(&mut self, direction: Direction, value: T) {
        match direction {
            Direction::North => { self.north = value },
            Direction::East => { self.east = value },
            Direction::South => { self.south = value },
            Direction::West => { self.west = value },
        }
    }
}

impl <T: Clone> DirectionMap<T> {
    pub fn new(value: T) -> DirectionMap<T> {
        DirectionMap {
            north: value.clone(),
            east: value.clone(),
            south: value.clone(),
            west: value.clone()
        }
    }
}