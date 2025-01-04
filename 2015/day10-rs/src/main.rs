use std::env::args;

fn main() {
    let pargs = Vec::from_iter(args());
    if pargs.len() != 2 {
        eprintln!("Please specify 'part1' or 'part2'");
        return;
    }
    let input = "1113122113".to_string();
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
    let mut chars = input.chars().collect();
    for _ in 0..40 {
        chars = look_and_say(chars);
    }
    chars.len() as u32
}

fn part2(input: String) -> u32 {
    let mut chars = input.chars().collect();
    for _ in 0..50 {
        chars = look_and_say(chars);
    }
    chars.len() as u32
}

fn look_and_say(input: Vec<char>) -> Vec<char> {
    let mut out_queue = Vec::new();
    for input_char in input {
        let last = out_queue.last_mut();
        match last {
            None => {
                out_queue.push((input_char, 1u32));
            }
            Some(entry) => {
                if entry.0 == input_char {
                    entry.1 += 1;
                } else {
                    out_queue.push((input_char, 1));
                }
            }
        };
    }
    let mut output = Vec::new();
    for entry in out_queue {
        output.extend(entry.1.to_string().chars());
        output.push(entry.0);
    }
    output
}
