use std::env::args;

fn main() {
    let pargs = Vec::from_iter(args());
    if pargs.len() != 2 {
        eprintln!("Please specify 'part1' or 'part2'");
        return;
    }
    let input = load_input();
    match pargs[1].as_str() {
        "part1" => {
            println!("{}", part1(input))
        }
        "part2" => {
            println!("{}", part2(input))
        }
        _ => {
            eprintln!("Invalid argument.")
        }
    };
}

fn part1(input: String) -> u32 {
    let mut bytes = 0;
    for line in input.split("\n") {
        bytes += line.len() as u32 - bytes_in_line(line);
    }
    bytes
}

fn part2(input: String) -> u32 {
    let mut bytes = 0;
    for line in input.split("\n") {
        bytes += bytes_in_expanded_line(line) - line.len() as u32;
    }
    bytes
}

fn bytes_in_line(line: &str) -> u32 {
    let chars = line[1..line.len() - 1].chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut bytes = 0;
    while i < chars.len() {
        if chars[i] == '\\' {
            match chars[i + 1] {
                '\\' | '"' => {
                    i += 2;
                    bytes += 1;
                }
                'x' => {
                    i += 4;
                    bytes += 1;
                }
                _ => {
                    panic! {"unknown kind of escape!!"}
                }
            }
        } else {
            i += 1;
            bytes += 1;
        }
    }
    bytes
}

fn bytes_in_expanded_line(line: &str) -> u32 {
    let mut bytes = 2;
    for char in line.chars() {
        match char {
            '"' | '\\' => {
                bytes += 2;
            }
            _ => {
                bytes += 1;
            }
        }
    }
    bytes
}

fn load_input() -> String {
    use std::fs::read_to_string;

    read_to_string("./input.txt").unwrap().trim().to_string()
}
