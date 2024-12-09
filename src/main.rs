use std::time::Instant;

mod geometry;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let day: i32 = std::env::args().nth(1).unwrap_or("1".into()).parse().expect("Invalid day");
    let part: i32 = std::env::args().nth(2).unwrap_or("1".into()).parse().expect("Invalid part");
    let default_input_path = format!("inputs/day{}.txt", day);
    let input_path: String = std::env::args().nth(3).unwrap_or(default_input_path);

    println!("Running Day {} Part {} on {}", day, part, input_path);

    let now = Instant::now();
    match day {
        1 => day1::run(input_path.as_str(), part),
        2 => day2::run(input_path.as_str(), part),
        3 => day3::run(input_path.as_str(), part),
        4 => day4::run(input_path.as_str(), part),
        5 => day5::run(input_path.as_str(), part),
        6 => day6::run(input_path.as_str(), part),
        7 => day7::run(input_path.as_str(), part),
        8 => day8::run(input_path.as_str(), part),
        9 => day9::run(input_path.as_str(), part),
        _ => panic!("Unknown day specified")
    };

    let after = now.elapsed();

    println!("Runtime: {}", after.as_secs_f64());
}