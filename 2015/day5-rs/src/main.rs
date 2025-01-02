use itertools::Itertools;
use std::collections::{HashMap, HashSet};
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

fn part1(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|line| nice_string(line.trim()))
        .count() as u32
}

fn part2(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|line| nice_string_2(line.trim()))
        .count() as u32
}

fn nice_string(input: &str) -> bool {
    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    let bad_pairs = HashSet::from([('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')]);
    let mut vowel_count = 0;
    let mut doubles = 0;

    // Check the first letter, since we only check the second in the loop.
    if input.starts_with(|c| vowels.contains(&c)) {
        vowel_count += 1
    }

    for pair in input.chars().tuple_windows::<(char, char)>() {
        if bad_pairs.contains(&pair) {
            return false;
        }
        if vowels.contains(&pair.1) {
            vowel_count += 1;
        }
        if pair.0 == pair.1 {
            doubles += 1;
        }
    }
    if vowel_count > 2 && doubles > 0 {
        true
    } else {
        false
    }
}

fn nice_string_2(input: &str) -> bool {
    let pairs = input
        .chars()
        .tuple_windows::<(char, char)>()
        .enumerate()
        .fold(HashMap::new(), |mut pairs, (index, pair)| {
            pairs.entry(pair).or_insert(Vec::<usize>::new()).push(index);
            pairs
        });

    if !pairs.into_values().any(|indices| {
        indices.into_iter().combinations(2).any(|index_comb| {
            let a = index_comb[0];
            let b = index_comb[1];
            a.max(b) - a.min(b) > 1
        })
    }) {
        return false;
    }

    if !input
        .chars()
        .tuple_windows::<(char, char, char)>()
        .any(|triplet| triplet.0 == triplet.2)
    {
        return false;
    }

    true
}

fn load_input() -> String {
    use std::fs::read_to_string;

    String::from(read_to_string("./input.txt").unwrap().trim())
}
