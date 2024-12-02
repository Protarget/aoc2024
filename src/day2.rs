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
    let valid_rows: Vec<Vec<i64>> = parse_input(input_string.as_str())
    .into_iter()
    .filter(|x| row_valid(x, x.len() + 1))
    .collect();

    println!("{}", valid_rows.len())
}

fn part2(input_path: &str) {
    let input_string = std::fs::read_to_string(input_path).unwrap();
    let valid_rows: Vec<Vec<i64>> = parse_input(input_string.as_str())
    .into_iter()
    .filter(|x| row_valid_with_error(x))
    .collect();

    println!("{}", valid_rows.len())
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(parse_row)
        .collect()
}

fn parse_row(input: &str) -> Vec<i64> {
    input
        .split(" ")
        .map(|x| x.parse().unwrap_or(0))
        .collect()
}

fn row_valid_with_error(row: &Vec<i64>) -> bool {
    let is_already_valid = row_valid(row, row.len() + 1);
    
    if !is_already_valid {
        (0..row.len()).any(|i| row_valid(row, i))
    }
    else {
        true
    }
}

fn row_valid(row: &Vec<i64>, ignore: usize) -> bool {
    let mut last_sign = 0;
    let mut current_index = 1;
    let mut last_index = 0;
    let row_length = row.len();

    if ignore == 0 {
        last_index = 1;
        current_index = 2;
    }

    while current_index < row_length {
        if current_index != ignore {
            let x = *row.get(last_index).unwrap();
            let y = *row.get(current_index).unwrap();
            let delta = y - x;
            let mag = delta.abs();
            let sign = delta.signum();
            if mag < 1 || mag > 3 || (last_sign != 0 && delta.signum() != last_sign) {
                return false;
            }
            last_index = current_index;
            last_sign = sign;
        }
        current_index += 1;
    }

    true
}