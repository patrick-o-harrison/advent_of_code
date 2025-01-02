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
    parse_presents(&input)
        .map(|present| present.required_paper())
        .sum()
}

fn part2(input: String) -> u32 {
    parse_presents(&input)
        .map(|present| present.required_ribbon())
        .sum()
}

fn parse_presents(input: &String) -> impl Iterator<Item = Present> + '_ {
    input
        .split("\n")
        .filter(|line| *line != "")
        .map(|line| Present::parse_from_line(line))
}

fn load_input() -> String {
    use std::fs::read_to_string;

    read_to_string("./input.txt").unwrap()
}

struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn side_areas(self: &Self) -> Vec<u32> {
        let mut sides: Vec<u32> = vec![0, 0, 0];
        sides[0] = self.l * self.w;
        sides[1] = self.w * self.h;
        sides[2] = self.h * self.l;
        sides
    }

    fn side_perimeters(self: &Self) -> Vec<u32> {
        let mut sides: Vec<u32> = vec![0, 0, 0];
        sides[0] = self.l * 2 + self.w * 2;
        sides[1] = self.w * 2 + self.h * 2;
        sides[2] = self.h * 2 + self.l * 2;
        sides
    }

    fn volume(self: &Self) -> u32 {
        self.l * self.w * self.h
    }

    fn required_ribbon(self: &Self) -> u32 {
        let mut required = self.volume();
        let mut sides = self.side_perimeters();
        sides.sort();
        required += sides[0];
        required
    }

    fn required_paper(self: &Self) -> u32 {
        let mut sides = self.side_areas();
        sides.sort();
        let mut required = sides[0];
        required += sides.into_iter().sum::<u32>() * 2;
        required
    }

    fn parse_from_line(line: &str) -> Self {
        let terms: Vec<u32> = line.trim().split("x").map(|n| n.parse().unwrap()).collect();
        Present {
            l: terms[0],
            w: terms[1],
            h: terms[2],
        }
    }
}
