use crate::algebra::Matrix;

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
    let problems = parse_input(input_string.as_str(), 0);
    println!("{}", solve(&problems));
}

fn part2(input_path: &str) {
    let input_string = std::fs::read_to_string(input_path).unwrap();
    let problems = parse_input(input_string.as_str(), 10000000000000);
    println!("{}", solve(&problems));
}

fn solve(problems: &Vec<Matrix<f64>>) -> i64 {
    let mut sum: i64 = 0;
    for problem in problems {
        let solution = problem.solve();

        if solution.is_some() {
            let solution_vector = solution.unwrap();
            let solution_rounded = (solution_vector.get(0).round() as i64, solution_vector.get(1).round() as i64);

            let check_x = (solution_rounded.0 * problem.get(0, 0) as i64 + solution_rounded.1 * problem.get(0, 1) as i64) == problem.get(0, 2) as i64;
            let check_y = (solution_rounded.0 * problem.get(1, 0) as i64 + solution_rounded.1 * problem.get(1, 1) as i64) == problem.get(1, 2) as i64;

            if check_x && check_y {
                sum +=  solution_rounded.0 * 3 + solution_rounded.1;
            }
        }
    }
    sum
}

fn parse_input(input_string: &str, offset: i64) -> Vec<Matrix<f64>> {
    let mut problems = vec![];
    let mut current = Matrix::new(2, 3, 0.0);
    let mut new = true;
    for line in input_string.lines() {
        new = false;
        if line.starts_with("Button A:") {
            let (x, y) = parse_button(line, "Button A: ", "+");
            current.set(0, 0, x);
            current.set(1, 0, y);
        }
        else if line.starts_with("Button B:") {
            let (x, y) = parse_button(line, "Button B: ","+");
            current.set(0, 1, x);
            current.set(1, 1, y);
        }
        else if line.starts_with("Prize:") {
            let (x, y) = parse_button(line, "Prize: ","=");
            current.set(0, 2, x + offset as f64);
            current.set(1, 2, y + offset as f64);
        }
        else {
            problems.push(current);
            current = Matrix::new(2, 3, 0.0);
            new = true;
        }
    }

    if !new {
        problems.push(current);
    }

    problems
}

fn parse_button(button_string: &str, prefix: &str, delimiter: &str) -> (f64, f64) {
    let button_segments: Vec<&str> = button_string[prefix.len()..].split(", ").collect();

    let x: f64 = button_segments[0].split(delimiter).last().unwrap().parse().unwrap();
    let y: f64 = button_segments[1].split(delimiter).last().unwrap().parse().unwrap();

    (x, y)
}