#[derive(Debug)]
struct CalibrationEquation {
    target: u64,
    operands: Box<[u64]>
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
    let equations = parse_input(input_string.as_str());
    let result = equations.iter().map(evaluate_equation).fold(0, |x, y| x + y);
    println!("{:?}", result);
}

fn part2(input_path: &str) {
    let input_string = std::fs::read_to_string(input_path).unwrap();
    let equations = parse_input(input_string.as_str());
    let result = equations.iter().map(evaluate_equation_with_concatenation).fold(0, |x, y| x + y);
    println!("{:?}", result);
}

fn integer_concatenate(x: u64, y: u64) -> u64 {
    let digits = y.ilog10();
    x * 10u64.pow(digits + 1) + y
}

fn evaluate_equation(equation: &CalibrationEquation) -> u64 {
    let bit_count = 1 << equation.operands.len() - 1;
    evaluate_equation_with_concatenation_mask(equation, bit_count, 0)
}

fn evaluate_equation_with_concatenation(equation: &CalibrationEquation) -> u64 {
    let bit_count = 1 << equation.operands.len() - 1;

    for concatenation_mask in 0..=bit_count {
        let result = evaluate_equation_with_concatenation_mask(equation, bit_count, concatenation_mask);
        if result > 0 {
            return result;
        }
    }

    0
}

fn evaluate_equation_with_concatenation_mask(equation: &CalibrationEquation, bit_count: i32, concatenation_mask: i32) -> u64 {
    for operator_mask in (0..=bit_count).filter(|x| x & concatenation_mask == 0) {
        let first = equation.operands[0];
        let result = equation.operands
            .iter()
            .skip(1)
            .enumerate()
            .fold(first, |v, (i, &x)| if ((1 << i) & concatenation_mask) > 0 { 
                integer_concatenate(v, x) 
            } else if ((1 << i) & operator_mask) > 0 { 
                v * x 
            } else { 
                v + x 
            });
        
        if result == equation.target {
            return equation.target;
        }
    }

    0
}

fn parse_input(input: &str) -> Vec<CalibrationEquation> {
    let mut equations = vec![];

    for line in input.lines() {
        let split_line: Vec<&str> = line.split(':').collect();


        let target = split_line[0].parse().unwrap();
        let operands: Box<[u64]> = split_line[1].trim().split(" ").map(|x| x.parse().unwrap()).collect();
        equations.push(CalibrationEquation {
            target,
            operands
        });
    }

    equations
}