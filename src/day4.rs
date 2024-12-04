struct WordSearch {
    width: i32,
    height: i32,
    content: String
}

static DIRECTIONS: [(i32, i32); 8] = [(1, 1), (1, 0), (1, -1), (0, 1), (0, -1), (-1, 1), (-1, 0), (-1, -1)];

impl WordSearch {
    fn get(&self, x: i32, y: i32) -> Option<char> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let index = y * (self.width + 1) + x;
            Some(self.content.as_bytes()[index as usize] as char)
        }
        else {
            None
        }
    }

    fn get_sequence(&self, sx: i32, sy: i32, ox: i32, oy: i32, from: i32, to: i32) -> String {
        let mut buffer: Vec<char> = vec![];

        for index in from..to {
            let mc = self.get(sx + ox * index, sy + oy * index);
            if mc.is_some() {
                buffer.push(mc.unwrap())
            }
            else {
                break
            }
        }

        buffer.iter().collect()
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
    let grid = parse_input(input_string);
    let mut sum = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid.get(x, y) == Some('X') {
                for (dx, dy) in DIRECTIONS {
                    if grid.get_sequence(x, y, dx, dy, 1, 4) == "MAS" {
                        sum += 1;
                    }
                }
            }
        }
    }


    println!("{}", sum);
}

fn part2(input_path: &str) {
    let input_string = std::fs::read_to_string(input_path).unwrap();
    let grid = parse_input(input_string);
    let mut sum = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid.get(x, y) == Some('A') {
                let s1 = grid.get_sequence(x, y, 1, 1, -1, 2);
                let s2 = grid.get_sequence(x, y, 1, -1, -1, 2);
                if (s1 == "MAS" || s1 == "SAM") && (s2 == "MAS" || s2 == "SAM") {
                    sum += 1
                }
            }
        }
    }


    println!("{}", sum);
}

fn parse_input(input: String) -> WordSearch {
    let width = input.find('\n').unwrap();
    let height = input.len() / width;

    WordSearch {
        width: i32::try_from(width).unwrap(), 
        height: i32::try_from(height).unwrap(), 
        content: input
    }
}