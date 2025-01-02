use std::{collections::HashMap, env::args};

fn main() {
    let pargs = Vec::from_iter(args());
    if pargs.len() != 2 {
        eprintln!("Please specify 'part1' or 'part2'");
        return;
    }
    let raw_input = load_input();
    let input = raw_input.trim();

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

fn part1(input: &str) -> u32 {
    visit_houses(1, input)
}

fn part2(input: &str) -> u32 {
    visit_houses(2, input)
}

fn visit_houses(num_visitors: usize, input: &str) -> u32 {
    let mut visitors = vec![(0, 0); num_visitors as usize];
    let mut visiterator = (0..num_visitors).cycle();
    let mut visited = HashMap::from([
        ((0i32,0i32), 1u32)
    ]);
    for c in input.chars() {
        let visitor = visiterator.next().unwrap();
        let(mut x, mut y) = visitors[visitor as usize];
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => {}
        };
        let new_location = (x, y);
        let visits = match visited.get(&new_location) {
            Some(v) => v + 1,
            None => 1,
        };
        visited.insert(new_location, visits);
        visitors[visitor] = new_location;
    }
    visited.len() as u32
}

fn load_input() -> String {
    use std::fs::read_to_string;

    read_to_string("./input.txt").unwrap()
}