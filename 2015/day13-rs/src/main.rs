use itertools::Itertools;
use regex::Regex;
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

fn part1(input: String) -> i32 {
    let (guests, preferences) = parse_input(input);
    find_optimal_happiness(guests, preferences)
}

fn part2(input: String) -> i32 {
    let (mut guests, mut preferences) = parse_input(input);
    add_me(&mut guests, &mut preferences);
    find_optimal_happiness(guests, preferences)
}

fn find_optimal_happiness(guests: Vec<String>, preferences: HashMap<(String, String), i32>) -> i32 {
    let mut optimal_happiness = 0;
    let num_guests = guests.len();
    for seating_arrangement in guests.into_iter().permutations(num_guests) {
        let happiness = find_arrangement_happiness(&preferences, seating_arrangement);
        optimal_happiness = optimal_happiness.max(happiness);
    }
    optimal_happiness
}

fn find_arrangement_happiness(
    preferences: &HashMap<(String, String), i32>,
    seating_arrangement: Vec<String>,
) -> i32 {
    let mut arrangement_happiness = 0;

    for pair in seating_arrangement
        .into_iter()
        .circular_tuple_windows::<(String, String)>()
    {
        arrangement_happiness += preferences.get(&(pair.0.clone(), pair.1.clone())).unwrap();
        arrangement_happiness += preferences.get(&(pair.1, pair.0)).unwrap();
    }

    arrangement_happiness
}

fn parse_input(input: String) -> (Vec<String>, HashMap<(String, String), i32>) {
    let mut preferences = HashMap::new();
    let mut guests_set = HashSet::new();
    let re = Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).")
        .unwrap();
    for line in input.split("\n") {
        let captures = re.captures(line).unwrap();
        let guest = captures[1].to_string();
        let neighbor = captures[4].to_string();
        let gain_or_lose = &captures[2];
        let abs_happiness: i32 = captures[3].parse().unwrap();
        let happiness = match gain_or_lose {
            "gain" => abs_happiness,
            "lose" => -abs_happiness,
            _ => panic!(),
        };
        guests_set.insert(guest.clone());
        preferences.insert((guest, neighbor), happiness);
    }
    let guests = guests_set.into_iter().collect();
    (guests, preferences)
}

fn add_me(guests: &mut Vec<String>, preferences: &mut HashMap<(String, String), i32>) {
    let me = "Me".to_string();
    let mut new_pairings = Vec::new();
    for guest in guests.into_iter() {
        new_pairings.push((me.clone(), guest.clone()));
        new_pairings.push((guest.clone(), me.clone()));
    }
    for pairing in new_pairings {
        preferences.insert(pairing, 0);
    }
    guests.push(me.clone());
}

fn load_input() -> String {
    use std::fs::read_to_string;

    String::from(read_to_string("./input.txt").unwrap().trim())
}
