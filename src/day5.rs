
use std::{cmp::Ordering, collections::HashMap};

pub struct Manual {
    ordering_rules: HashMap<(i64, i64), Ordering>,
    updates: Vec<Vec<i64>>
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
    let manual = parse_input(input_string);
    let mut sum = 0;

    for update in manual.updates {
        let mut sorted_update = update.to_vec();
        sorted_update.sort_by(|x, y| *manual.ordering_rules.get(&(*x, *y)).unwrap_or(&Ordering::Equal));

        if sorted_update.iter().eq(update.iter()) {
            let middle_index = sorted_update.len() / 2;
            sum += sorted_update[middle_index];
        }
        
    }

    println!("{}", sum);
}

fn part2(input_path: &str) {
    let input_string = std::fs::read_to_string(input_path).unwrap();
    let manual = parse_input(input_string);
    let mut sum = 0;

    for update in manual.updates {
        let mut sorted_update = update.to_vec();
        sorted_update.sort_by(|x, y| *manual.ordering_rules.get(&(*x, *y)).unwrap_or(&Ordering::Equal));

        if !sorted_update.iter().eq(update.iter()) {
            let middle_index = sorted_update.len() / 2;
            sum += sorted_update[middle_index];
        }
        
    }

    println!("{}", sum);
}

fn parse_input(input: String) -> Manual {
    let mut parsing_rules = true;
    let mut ordering_rules: HashMap<(i64, i64), Ordering> = HashMap::new();
    let mut updates = vec![];
    for line in input.lines() {
        if line.is_empty() {
            parsing_rules = false
        }
        else if parsing_rules {
            let rule: Vec<&str> = line.split("|").collect();
            let x: i64 = rule[0].parse().unwrap();
            let y: i64 = rule[1].parse().unwrap();
            ordering_rules.insert((x, y), Ordering::Less);
            ordering_rules.insert((y, x), Ordering::Greater);
        } else {
            let update = line.split(",").map(|x| x.parse().unwrap()).collect();
            updates.push(update);
        }
    }

    Manual {
        ordering_rules,
        updates
    }
}