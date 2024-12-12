use std::collections::HashSet;

use crate::geometry::{Direction, DirectionMap, Grid, Point};

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
    let grid = Grid::from(input_string.as_str());
    let sum = solve(grid, false);
    println!("{}", sum);
}

fn part2(input_path: &str) {
    let input_string = std::fs::read_to_string(input_path).unwrap();
    let grid = Grid::from(input_string.as_str());
    let sum = solve(grid, true);
    println!("{}", sum);
}

fn solve(grid: Grid<char>, calculate_sides: bool) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut regions: Vec<Vec<(Point, DirectionMap<bool>)>> = vec![];
    for (p, v) in grid.iter() {
        if !visited.contains(&p) {
            let region = grid.flood_fill(p, |x| x == v);

            for (vp, n) in region.iter() {
                visited.insert(*vp);
            }

            if !region.is_empty() {
                regions.push(region);
            }
        }
    }

    if calculate_sides {
        calculate_with_sides(&regions)
    }
    else {
        calculate_with_perimeter(&regions)
    }
}

fn calculate_with_perimeter(regions: &Vec<Vec<(Point, DirectionMap<bool>)>>) -> usize {
    let mut sum = 0;
    for region in regions {
        let mut area = 0;
        let mut perimeter = 0;
        for (_, n) in region.iter() {
            area += 1;
            perimeter += 4 - n.iter().filter(|&&x| x).count()
        }
        sum += area * perimeter;
    }
    sum
}

fn calculate_with_sides(regions: &Vec<Vec<(Point, DirectionMap<bool>)>>) -> usize {
    let mut sum = 0;
    for region in regions {
        let area = region.len();
        let mut sides = 0;
        for direction in [Direction::North, Direction::East, Direction::South, Direction::West] {
            let mut visited: HashSet<Point> = HashSet::new();
            let boundary: HashSet<Point> = region.iter().filter(|x| !x.1.get(direction)).map(|x| x.0).collect();

            for &p in boundary.iter() {
                if !visited.contains(&p) {
                    sides += 1;
                    visited.insert(p);

                    let edge_direction = direction.turn_right();

                    let mut next = p + edge_direction;

                    while !visited.contains(&next) && boundary.contains(&next) {
                        visited.insert(next);
                        next = next + edge_direction;
                    }

                    next = p - edge_direction;

                    while !visited.contains(&next) && boundary.contains(&next) {
                        visited.insert(next);
                        next = next - edge_direction;
                    }
                }
            }
        }

        sum += area * sides;
    }
    sum
}