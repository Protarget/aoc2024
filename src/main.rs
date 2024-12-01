#![feature(test)]

extern crate test;

mod day1;

fn main() {
    let day: i32 = std::env::args().nth(1).unwrap_or("1".into()).parse().expect("Invalid day");
    let part: i32 = std::env::args().nth(2).unwrap_or("1".into()).parse().expect("Invalid part");
    let default_input_path = format!("day{}.txt", day);
    let input_path: String = std::env::args().nth(3).unwrap_or(default_input_path);

    match day {
        1 => day1::run(input_path.as_str(), part),
        _ => panic!("Unknown day specified")
    };
}