use std::time::Instant;

mod day1;
mod day2;

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
        _ => panic!("Unknown day specified")
    };

    let after = now.elapsed();

    println!("Runtime: {}", after.as_secs_f64());
}