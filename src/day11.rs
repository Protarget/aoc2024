use std::collections::HashMap;

pub struct StoneMemoizer {
    stone_info: HashMap<(u64, u64), u64>
}

impl StoneMemoizer {
    fn new() -> StoneMemoizer {
        StoneMemoizer {
            stone_info: HashMap::new()
        }
    }

    fn check(&self, input: u64, remaining: u64) -> Option<&u64> {
        self.stone_info.get(&(input, remaining))
    }

    fn insert(&mut self, input: u64, remaining: u64, value: u64) {
        self.stone_info.insert((input, remaining), value);
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
    let stones = parse_input(input_string.as_str());
    let mut memoizer = StoneMemoizer::new();
    let mut sum = 0;
    for stone in stones {
        sum += process_stone(&mut memoizer, stone, 25);
    }
    println!("{}", sum);
}

fn part2(input_path: &str) {
    let input_string = std::fs::read_to_string(input_path).unwrap();
    let stones = parse_input(input_string.as_str());
    let mut memoizer = StoneMemoizer::new();
    let mut sum = 0;
    for stone in stones {
        sum += process_stone(&mut memoizer, stone, 75);
    }
    println!("{}", sum);
}

fn process_stone(memoizer: &mut StoneMemoizer, stone: u64, remaining: u64) -> u64 {
    let entry = memoizer.check(stone, remaining);

    match entry {
        Some(&x) => x,
        None => {
            if remaining == 0 {
                1
            }
            else if stone == 0 {
                let result = process_stone(memoizer, 1, remaining - 1);
                memoizer.insert(stone, remaining, result);
                result
            }
            else {
                let stone_digits: u32 = stone.ilog10() + 1;
                let digit_factor = 10u64.pow(stone_digits as u32 / 2);
                let second_half_digits = stone % digit_factor;
                let first_half_digits = stone / digit_factor;

                if stone_digits % 2 == 0 {
                    let result1 = process_stone(memoizer, first_half_digits, remaining - 1);
                    let result2 = process_stone(memoizer, second_half_digits, remaining - 1);
                    let result = result1 + result2;
                    memoizer.insert(stone, remaining, result);
                    result
                }
                else {
                    let result = process_stone(memoizer, stone * 2024, remaining - 1);
                    result
                }
            }
        }
    }


}

fn parse_input(input_string: &str) -> Vec<u64> {
    input_string.split(' ').map(|x| x.parse().unwrap()).collect()
}