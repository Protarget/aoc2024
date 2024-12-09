use std::collections::HashSet;

use crate::geometry::{Direction, Point};

struct VisitedSet {
    width: usize,
    content: Box<[u8]>
}

struct PatrolMap {
    size: Point,
    start_position: Point,
    obstacles: VisitedSet
}

struct PatrolMapIterator<'a> {
    patrol_map: &'a PatrolMap,
    position: Point,
    direction: Direction,
    overlay: Point
}

impl <'a> Iterator  for PatrolMapIterator<'a> {
    type Item = (Point, Direction, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.in_bounds(self.patrol_map.size) {
            let old_position = self.position;

            let mut changed_direction = false;

            let mut new_position = self.position + self.direction;

            while new_position.in_bounds(self.patrol_map.size) && (self.patrol_map.obstacles.is_visited(new_position, Direction::North) || new_position == self.overlay) {
                self.direction = self.direction.turn_right();
                new_position = self.position + self.direction;
                changed_direction = true;
            }

            self.position = new_position;

            Some((old_position, self.direction, changed_direction))
        }
        else {
            None
        }
    }
}


impl From<&PatrolMap> for VisitedSet {
    fn from(value: &PatrolMap) -> Self {
        VisitedSet::new(value.size)
    }
}

impl VisitedSet {
    fn new(size: Point) -> VisitedSet {
        VisitedSet {
            width: size.0 as usize,
            content: vec![0; size.area() as usize].into_boxed_slice()
        }
    }

    fn new_from_obstacles(size: Point, obstacle_set: &HashSet<Point>) -> VisitedSet {
        let mut result = VisitedSet::new(size);

        for &obstacle in obstacle_set {
            result.visit(obstacle, Direction::North);
        }

        result
    }

    fn visit(&mut self, position: Point, direction: Direction) {
        let index = position.1 as usize * self.width + position.0 as usize;
        self.content[index] |= direction.to_bit_mask();
    }

    fn is_visited(&self, position: Point, direction: Direction) -> bool {
        let index = position.1 as usize * self.width + position.0 as usize;
        self.content[index] & direction.to_bit_mask() > 0
    }
}

impl From<&str> for PatrolMap {
    fn from(input: &str) -> Self {
        let mut obstacle_set = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut start_position = Point(0, 0);
    
        for c in input.chars() {
            match c {
                '#' => { obstacle_set.insert(Point(x, y)); }
                '^' => { start_position = Point(x, y); }
                '\n' => {
                    width = x;
                    x = -1;
                    y += 1;
                }
                _ => {}
            };
    
            x += 1;
        }

        let size = Point(width, y + 1);
        
        PatrolMap {
            size,
            start_position,
            obstacles: VisitedSet::new_from_obstacles(size, &obstacle_set)
        }
    }
}

impl PatrolMap {
    fn path<'a>(&'a self, start_position: Point, start_direction: Direction) -> PatrolMapIterator<'a> {
        PatrolMapIterator {
            patrol_map: self,
            position: start_position,
            direction: start_direction,
            overlay: Point(-100, -100)
        }
    }

    fn path_with_overlay<'a>(&'a self, start_position: Point, start_direction: Direction, overlay: Point) -> PatrolMapIterator<'a> {
        PatrolMapIterator {
            patrol_map: self,
            position: start_position,
            direction: start_direction,
            overlay
        }
    }
}


pub fn run(input_path: &str, part: i32) {
    if part <= 1 {
        part1(input_path);
    }
    else {
        part2(input_path);
    }
}

fn part1(input_path: &str) {
    let input_string = std::fs::read_to_string(input_path).unwrap();
    let patrol_map: PatrolMap = input_string.as_str().into();
    let point_set: HashSet<Point> = patrol_map.path(patrol_map.start_position, Direction::North).map(|x| x.0).collect();

    println!("{}", point_set.len());
}

fn part2(input_path: &str) {
    let input_string = std::fs::read_to_string(input_path).unwrap();
    let patrol_map: PatrolMap = input_string.as_str().into();
    let point_set: HashSet<Point> = patrol_map.path(patrol_map.start_position, Direction::North).map(|x| x.0).collect();

    let mut non_terminating_count = 0;

    for path_point in point_set {
        if path_point != patrol_map.start_position {
            let mut visited_set= VisitedSet::from(&patrol_map);
            for (point, direction, changed_direction) in patrol_map.path_with_overlay(patrol_map.start_position, Direction::North, path_point) {
                if changed_direction {
                    if visited_set.is_visited(point, direction) {
                        non_terminating_count += 1;
                        break
                    }

                    visited_set.visit(point, direction);
                }
            }
        }
    }

    println!("{}", non_terminating_count);
}