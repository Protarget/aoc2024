use std::collections::{HashMap, HashSet};

use crate::geometry::Point;

struct CityMap {
    size: Point,
    antennae: HashMap<char, Vec<Point>>
}

impl From<&str> for CityMap {
    fn from(input: &str) -> Self {
        let mut antennae: HashMap<char, Vec<Point>> = HashMap::new();
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
    
        for c in input.chars() {
            match c {
                '.' => { }
                '\n' => {
                    width = x;
                    x = -1;
                    y += 1;
                }
               c => {
                    let location = Point(x, y);
                    antennae.entry(c).or_default().push(location);
               }
            };
    
            x += 1;
        }

        let size = Point(width, y + 1);
        
        CityMap {
            size,
            antennae
        }
    }
}

impl CityMap {
    fn get_antinodes(&self, bounded: bool) -> impl Iterator<Item = Point> + '_ {
        self.antennae.values()
            .filter(|av| av.len() > 1)
            .flat_map(move |a| {
                let mut result: Vec<Point> = vec![];

                if !bounded && a.len() > 1 {
                    for &antenna in a {
                        result.push(antenna);
                    }
                }

                for (&antenna1, &antenna2) in get_pairs(a) {
                    if antenna1 != antenna2 {
                        let a1delta = antenna2 - antenna1;
                        let a2delta = antenna1 - antenna2;
                        let mut antinode1 = antenna2 + a1delta;
                        let mut antinode2 = antenna1 + a2delta;

                        let mut evaluated1 = false;
                        while antinode1.in_bounds(self.size) && (!bounded || !evaluated1){
                            result.push(antinode1);
                            antinode1 = antinode1 + a1delta;
                            evaluated1 = true;
                        }

                        let mut evaluated2 = false;
                        while antinode2.in_bounds(self.size) && (!bounded || !evaluated2) {
                            result.push(antinode2);
                            antinode2 = antinode2 + a2delta;
                            evaluated2 = true;
                        }
                    }
                }
                
                result
            })
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
    let city_map = CityMap::from(input_string.as_str());
    let unique_antinodes: HashSet<Point> = city_map.get_antinodes(true).collect();
    println!("{:?}", unique_antinodes.len());
}

fn part2(input_path: &str) {
    let input_string = std::fs::read_to_string(input_path).unwrap();
    let city_map = CityMap::from(input_string.as_str());
    let unique_antinodes: HashSet<Point> = city_map.get_antinodes(false).collect();
    println!("{:?}", unique_antinodes.len());
}

fn get_pairs<T>(input: &Vec<T>) -> impl Iterator<Item = (&T, &T)> + '_ {
    input.iter()
        .enumerate()
        .flat_map(|(i, x)| {
            input.iter().skip(i + 1).map(move |y| (x, y))
        })
}