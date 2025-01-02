use itertools::iproduct;
use regex::Regex;
use std::env::args;

struct Point(u32, u32);

enum InstructionType {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    instruction_type: InstructionType,
    a: Point,
    b: Point,
}

trait Decoration {
    fn toggle_light(&mut self, x: u32, y: u32);
    fn turn_on_light(&mut self, x: u32, y: u32);
    fn turn_off_light(&mut self, x: u32, y: u32);
    fn get_light_result(&self) -> u32;

    fn process_inst(&mut self, inst: Instruction) {
        match inst.instruction_type {
            InstructionType::Toggle => {
                for (x, y) in iproduct!(inst.a.0..=inst.b.0, inst.a.1..=inst.b.1) {
                    self.toggle_light(x, y);
                }
            }
            InstructionType::TurnOn => {
                for (x, y) in iproduct!(inst.a.0..=inst.b.0, inst.a.1..=inst.b.1) {
                    self.turn_on_light(x, y);
                }
            }
            InstructionType::TurnOff => {
                for (x, y) in iproduct!(inst.a.0..=inst.b.0, inst.a.1..=inst.b.1) {
                    self.turn_off_light(x, y);
                }
            }
        };
    }
}

struct DecorationV1 {
    width: u32,
    lights: Vec<bool>,
}

impl DecorationV1 {
    fn new(width: u32, height: u32) -> Self {
        DecorationV1 {
            width,
            lights: vec![false; (width * height) as usize],
        }
    }
}

impl Decoration for DecorationV1 {
    fn turn_on_light(&mut self, x: u32, y: u32) {
        let i = coord(self.width, x, y);
        self.lights[i] = true;
    }

    fn turn_off_light(&mut self, x: u32, y: u32) {
        let i = coord(self.width, x, y);
        self.lights[i] = false;
    }

    fn toggle_light(&mut self, x: u32, y: u32) {
        let i = coord(self.width, x, y);
        self.lights[i] = !self.lights[i]
    }

    fn get_light_result(&self) -> u32 {
        let mut lit_count = 0;
        for light in &self.lights {
            if *light {
                lit_count += 1;
            }
        }
        lit_count
    }
}

struct DecorationV2 {
    width: u32,
    lights: Vec<u32>,
}

impl DecorationV2 {
    fn new(width: u32, height: u32) -> Self {
        DecorationV2 {
            width,
            lights: vec![0; (width * height) as usize],
        }
    }
}

impl Decoration for DecorationV2 {
    fn turn_on_light(&mut self, x: u32, y: u32) {
        let i = coord(self.width, x, y);
        self.lights[i] += 1;
    }

    fn turn_off_light(&mut self, x: u32, y: u32) {
        let i = coord(self.width, x, y);
        let light = self.lights[i];
        if light > 0 {
            self.lights[i] = light - 1
        }
    }

    fn toggle_light(&mut self, x: u32, y: u32) {
        let i = coord(self.width, x, y);
        self.lights[i] += 2
    }

    fn get_light_result(&self) -> u32 {
        let mut lit_count = 0;
        for light in &self.lights {
            lit_count += light;
        }
        lit_count
    }
}

fn process_input(input: &str, decoration: &mut impl Decoration) -> u32 {
    input
        .split("\n")
        .map(|line| parse_line(line))
        .for_each(|inst| decoration.process_inst(inst));
    decoration.get_light_result()
}

fn parse_line(line: &str) -> Instruction {
    let re = Regex::new(r"(turn|toggle) (on |off |)(\d+),(\d+) through (\d+),(\d+)").unwrap();
    let captures = re.captures(line).unwrap();
    let a = Point(
        captures[3].parse::<u32>().unwrap(),
        captures[4].parse::<u32>().unwrap(),
    );
    let b = Point(
        captures[5].parse::<u32>().unwrap(),
        captures[6].parse::<u32>().unwrap(),
    );
    match &captures[1] {
        "toggle" => Instruction {
            instruction_type: InstructionType::Toggle,
            a,
            b,
        },
        "turn" => {
            let arg = &captures[2];
            match arg {
                "on " => Instruction {
                    instruction_type: InstructionType::TurnOn,
                    a,
                    b,
                },
                "off " => Instruction {
                    instruction_type: InstructionType::TurnOff,
                    a,
                    b,
                },
                _ => panic!("Invalid argument."),
            }
        }
        _ => panic!("Invalid instruction."),
    }
}

fn coord(width: u32, x: u32, y: u32) -> usize {
    (y * width + x) as usize
}

fn load_input() -> String {
    use std::fs::read_to_string;

    String::from(read_to_string("./input.txt").unwrap().trim())
}

fn part1(input: &str) -> u32 {
    let mut decoration = DecorationV1::new(1000, 1000);
    process_input(input, &mut decoration)
}

fn part2(input: &str) -> u32 {
    let mut decoration = DecorationV2::new(1000, 1000);
    process_input(input, &mut decoration)
}

fn main() {
    let pargs = Vec::from_iter(args());
    if pargs.len() != 2 {
        eprintln!("Please specify 'part1' or 'part2'");
        return;
    }
    let input = load_input();
    match pargs[1].as_str() {
        "part1" => {
            println!("{}", part1(&input))
        }
        "part2" => {
            println!("{}", part2(&input))
        }
        _ => {
            eprintln!("Invalid argument.")
        }
    };
}
