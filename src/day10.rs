use std::collections::HashSet;

use crate::geometry::{Direction, Grid, Point};

pub fn run(input_path: &str, part: i32) {
    if part <= 1 {
        part1(input_path);
    }
    else {
        part2(input_path);
    }
}

fn part1(input_path: &str) {
    println!("{}", solve(input_path, false));
}

fn part2(input_path: &str) {
    println!("{}", solve(input_path, true));
}

fn solve(input_path: &str, distinct: bool) -> i64 {
    let input_string = std::fs::read_to_string(input_path).unwrap();
    let char_grid = Grid::from(input_string.as_str());
    let number_grid: Grid<i64> = char_grid.map(|x| if *x == '.' { -1 } else { x.to_digit(10).unwrap() as i64 });

    number_grid
        .find_all(|&x| x == 0)
        .map(|x| count_paths(&number_grid, x.0))
        .fold(0, |a, b| a + if distinct { b.1 } else { b.0 })
}

fn count_paths(grid: &Grid<i64>, start_position: Point) -> (i64, i64) {
    let mut check_stack = vec![start_position];
    let mut ending_locations: HashSet<Point> = HashSet::new();
    let mut distinct_trails = 0;

    while !check_stack.is_empty() {
        let current_position = check_stack.pop().unwrap();
        let &current_value = grid.get(current_position).unwrap();

        if current_value == 9 {
            ending_locations.insert(current_position);
            distinct_trails += 1;
        }
        else {
            let neighbors = [
                grid.get_with_index(current_position + Direction::North),
                grid.get_with_index(current_position + Direction::East),
                grid.get_with_index(current_position + Direction::South),
                grid.get_with_index(current_position + Direction::West)
            ];

            let valid_neighbors = neighbors
                .iter()
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .filter(|(_, &x)| x == current_value + 1);

            for neighbor in valid_neighbors {
                check_stack.push(neighbor.0);
            }
        }
    }

    (ending_locations.len() as i64, distinct_trails)
}