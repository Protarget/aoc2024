#[derive(Debug)]
enum Token {
    Number(i64),
    Mul,
    Do,
    Dont,
    OpenParen,
    CloseParen,
    Comma,
    Junk
}

#[derive(Debug)]
enum TokenizerMode {
    Unknown,
    Number,
    Text,
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

    println!("{}", process(input_string.as_str(), true));
}

fn part2(input_path: &str) {
    let input_string = std::fs::read_to_string(input_path).unwrap();

    println!("{}", process(input_string.as_str(), false));
}


fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut buffer: Vec<char> = vec![];
    let mut mode = TokenizerMode::Unknown;
    let mut pointer: usize = 0;
    let input_bytes = input.as_bytes();

    while pointer < input.len() {
        let c: char = input_bytes[pointer].into();
        match mode {
            TokenizerMode::Unknown => {
                if c.is_digit(10) {
                    mode = TokenizerMode::Number;
                }
                else if c.is_alphabetic() {
                    mode = TokenizerMode::Text;
                }
                else {
                    if c == '(' {
                        tokens.push(Token::OpenParen);
                    }
                    else if c == ')' {
                        tokens.push(Token::CloseParen)
                    }
                    else if c == ',' {
                        tokens.push(Token::Comma)
                    }
                    else if !c.is_whitespace() {
                        tokens.push(Token::Junk);
                    }

                    pointer += 1;
                }
            },
            TokenizerMode::Number => {
                if !c.is_digit(10) {
                    if buffer.len() > 0 {
                        let buffer_string: String = buffer.iter().collect();
                        tokens.push(Token::Number(buffer_string.parse().unwrap()));
                        buffer.clear();
                    }
                    mode = TokenizerMode::Unknown
                }
                else {
                    buffer.push(c);
                    pointer += 1;
                }
            },
            TokenizerMode::Text => {
                if !c.is_alphabetic() && c != '\'' {
                    if buffer.len() > 0 {
                        let buffer_string: String = buffer.iter().collect();

                        let generated_token = if buffer_string.ends_with("mul") {
                            Token::Mul
                        }
                        else if buffer_string.ends_with("do") {
                            Token::Do
                        }
                        else if buffer_string.ends_with("don't") {
                            Token::Dont
                        }
                        else {
                            Token::Junk
                        };

                        tokens.push(generated_token);
                        buffer.clear();
                    }
                    mode = TokenizerMode::Unknown
                }
                else {
                    buffer.push(c);
                    pointer += 1;
                }
            }
        }
    }
    tokens
}

fn process(input: &str, ignore_control: bool) -> i64 {
    let mut sum = 0;
    let tokens = tokenize(input);
    let mut collect = true;

    for window in tokens.windows(6) {
        let matchable_window = <&[Token; 6]>::try_from(window).unwrap();

        match matchable_window {
            [Token::Do, _, _, _, _, _] => {
                collect = true;
            },
            [Token::Dont, _, _, _, _, _] => {
                collect = false;
            },
            [Token::Mul, Token::OpenParen, Token::Number(n1), Token::Comma, Token::Number(n2), Token::CloseParen] => {
                if collect || ignore_control {
                    sum += n1 * n2;
                }
            },
            _ => {
                
            }
        }
    }

    sum
}