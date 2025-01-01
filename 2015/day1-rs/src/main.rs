use std::env::args;

fn main() {
    let pargs = Vec::from_iter(args());
    if pargs.len() != 2 {
        eprintln!("Please specify 'part1' or 'part2'");
        return;
    }
    match pargs[1].as_str() {
        "part1" => {
            println!("{}", part1())
        }
        "part2" => {
            println!("{}", part2())
        }
        _ => {
            eprintln!("Invalid argument.")
        }
    };
}

fn part1() -> i32 {
    let input = load_input();
    input.chars().map(input_unit_value).sum()
}

fn part2() -> i32 {
    let input = load_input();
    let mut floor: i32 = 0;
    for (i, c) in input.chars().enumerate() {
        floor += input_unit_value(c);
        if floor == -1 {
            return (i + 1) as i32;
        }
    }
    0
}

fn input_unit_value(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

fn load_input() -> String {
    use std::fs::read_to_string;

    read_to_string("./input.txt").unwrap()
}
