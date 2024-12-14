use std::{collections::{HashMap, HashSet}, hash::{DefaultHasher, Hash, Hasher}};

use crate::geometry::Point;

#[derive(Debug, Hash)]
struct Robot {
    position: Point,
    velocity: Point
}

#[derive(Hash)]
struct PatrolMap {
    size: Point,
    robots: Vec<Robot>
}

impl Robot {
    fn step(&mut self, amount: i64, size: Point) {
        self.position = (self.position + self.velocity * amount).wrap(size);
    }
}

impl PatrolMap {
    fn from_string_with_size(value: &str, size: Point) -> PatrolMap {

        let mut robots: Vec<Robot> = vec![];

        for line in value.lines() {
            let segments: Vec<&str> = line.split(" ").collect();
            let position_segments: Vec<&str>= segments[0].split("=").skip(1).last().unwrap().split(",").collect();
            let px: i64 = position_segments[0].parse().unwrap();
            let py: i64 = position_segments[1].parse().unwrap();
            let velocity_segments: Vec<&str>= segments[1].split("=").skip(1).last().unwrap().split(",").collect();
            let vx: i64 = velocity_segments[0].parse().unwrap();
            let vy: i64 = velocity_segments[1].parse().unwrap();

            robots.push(Robot {
                position: Point(px, py),
                velocity: Point(vx, vy)
            })
        }

        PatrolMap {
            size,
            robots
        }
    }


    fn step(&mut self, amount: i64) {
        for robot in self.robots.iter_mut() {
            robot.step(amount, self.size);
        }
    }

    fn count_quadrants(&self) -> i64 {
        let mut results = [0, 0, 0, 0];
        let midpoint_x = self.size.0 / 2;
        let midpoint_y = self.size.1 / 2;
        for robot in self.robots.iter() {
            if robot.position.0 < midpoint_x && robot.position.1 < midpoint_y {
                results[0] += 1;
            }
            else if robot.position.0 > midpoint_x && robot.position.1 < midpoint_y {
                results[1] += 1;
            }
            else if robot.position.0 < midpoint_x && robot.position.1 > midpoint_y {
                results[2] += 1;
            }
            else if robot.position.0 > midpoint_x && robot.position.1 > midpoint_y {
                results[3] += 1;
            }
        }

        results[0] * results[1] * results[2] * results[3]
    }

    fn estimate_entropy(&self) -> i64 {
        let mut x_buckets: HashMap<i64, Vec<i64>> = HashMap::new();
        let mut y_buckets: HashMap<i64, Vec<i64>> = HashMap::new();

        for robot in self.robots.iter() {
            x_buckets.entry(robot.position.0).or_insert_with(|| vec![]).push(robot.position.1);
            y_buckets.entry(robot.position.1).or_insert_with(|| vec![]).push(robot.position.0);
        }

        let mut sum = 0;

        for (_, bucket) in x_buckets.iter_mut() {
            sum += PatrolMap::estimate_line_entropy(bucket);
        }

        for (_, bucket) in y_buckets.iter_mut() {
            sum += PatrolMap::estimate_line_entropy(bucket);
        }

        sum
    }

    fn estimate_line_entropy(bucket: &mut Vec<i64>) -> i64 {
        bucket.sort();

        let mut bucket_iter = bucket.iter();
        let mut previous = *bucket_iter.next().unwrap();
        let mut runs = 0;
        for &value in bucket_iter {
            if value != previous + 1 {
                runs += 1;
            }

            previous = value;
        }
        runs
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
    let mut patrol_map = PatrolMap::from_string_with_size(&input_string, Point(101, 103));
    patrol_map.step(100);
    println!("{}", patrol_map.count_quadrants());
}

fn part2(input_path: &str) {
    let input_string = std::fs::read_to_string(input_path).unwrap();
    let mut patrol_map = PatrolMap::from_string_with_size(&input_string, Point(101, 103));
    let mut visited_states: HashSet<u64> = HashSet::new();
    let mut running = true;
    let mut minimum_entropy = 0xFFFFFFFFFFFFFF;
    let mut minimum_entropy_index = -1;


    let mut index = 0;
    while running {
        index += 1;
        patrol_map.step(1);
        let mut hasher = DefaultHasher::new();
        patrol_map.hash(&mut hasher);
        let state_hash = hasher.finish();

        if visited_states.contains(&state_hash) {
            running = false;
        }
        else {
            visited_states.insert(state_hash);
        }

        let estimated_entropy = patrol_map.estimate_entropy();

        if estimated_entropy < minimum_entropy {
            minimum_entropy = estimated_entropy;
            minimum_entropy_index = index;
        }
    }
    // 1389 too low

    println!("{}", minimum_entropy_index);
}