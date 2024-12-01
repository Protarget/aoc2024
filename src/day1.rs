use std::collections::HashMap;

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
    let (left_list, right_list) = parse_and_sort_input(input_string.as_str());

    let total = Iterator::zip(left_list.into_iter(), right_list.into_iter())
        .map(|(x, y)| (x - y).abs())
        .fold(0, |total, x| total + x);

    println!("{}", total);
}

fn part2(input_path: &str) {
    let input_string = std::fs::read_to_string(input_path).unwrap();
    let (left_list, right_list) = parse_input(input_string.as_str());
    let frequencies = count_frequencies(&right_list);

    let total = left_list.into_iter()
        .map(|x| x * frequencies.get(&x).unwrap_or(&0))
        .fold(0, |total, x| total + x);

    println!("{}", total);
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut first_list: Vec<i64> = Vec::new();
    let mut second_list: Vec<i64> = Vec::new();

    for line in input.lines() {
        let mut split_line = line.split("   ");
        first_list.push(split_line.nth(0).unwrap().parse().unwrap());
        second_list.push(split_line.nth(0).unwrap().parse().unwrap());
    }

    (first_list, second_list)
}

fn parse_and_sort_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    let (mut left_list, mut right_list) = parse_input(input);

    left_list.sort();
    right_list.sort();

    return (left_list, right_list)
}

fn count_frequencies(input: &Vec<i64>) -> HashMap<i64, i64> {
    let mut frequencies: HashMap<i64, i64> = HashMap::new();

    for &v in input {
        *frequencies.entry(v).or_insert(0) += 1;
    }

    frequencies
}