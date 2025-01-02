use hex;
use md5::{Digest, Md5};
use std::env::args;

fn main() {
    let pargs = Vec::from_iter(args());
    if pargs.len() != 2 {
        eprintln!("Please specify 'part1' or 'part2'");
        return;
    }
    let input = "iwrupvqb";
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
    find_zeroes(input, 5)
}

fn part2(input: &str) -> u32 {
    find_zeroes(input, 6)
}

fn find_zeroes(input: &str, zeroes: usize) -> u32 {
    let target = "0".repeat(zeroes);
    for i in 0..2_000000_000 {
        let candidate_str = i.to_string();
        let test_str = input.to_owned() + &candidate_str;
        let mut hasher = Md5::new();
        hasher.update(test_str.as_bytes());
        let result = hasher.finalize();
        let hexval = hex::encode(&result);
        if hexval.starts_with(&target) {
            return i;
        }
    }
    0
}
