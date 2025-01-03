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

fn part1(input: String) -> u32 {
    let (distance_map, locations) = parse_input(input);
    find_route_length(distance_map, locations, RouteOrdering::Min)
}

fn part2(input: String) -> u32 {
    let (distance_map, locations) = parse_input(input);
    find_route_length(distance_map, locations, RouteOrdering::Max)
}

fn find_route_length(
    distance_map: HashMap<(String, String), u32>,
    locations: Vec<String>,
    ordering: RouteOrdering,
) -> u32 {
    let num_locations = locations.len();
    let mut selected_route = match ordering {
        RouteOrdering::Min => u32::MAX,
        RouteOrdering::Max => 0,
    };
    for combo in locations.into_iter().permutations(num_locations) {
        let mut route_length = 0;
        for route_leg in combo.into_iter().tuple_windows::<(String, String)>() {
            let a = &route_leg.0.clone().min(route_leg.1.clone());
            let b = &route_leg.0.clone().max(route_leg.1.clone());
            let route_leg_distance = distance_map.get(&(a.clone(), b.clone())).unwrap();
            route_length += route_leg_distance;
        }

        selected_route = match ordering {
            RouteOrdering::Max => selected_route.max(route_length),
            RouteOrdering::Min => selected_route.min(route_length),
        };
    }
    selected_route
}

enum RouteOrdering {
    Min,
    Max,
}

fn parse_input(input: String) -> (HashMap<(String, String), u32>, Vec<String>) {
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let mut distance_map = HashMap::new();
    let mut locations = HashSet::new();
    for line in input.split("\n") {
        let captures = re.captures(line).unwrap();
        let a = captures[1].to_string().min(captures[2].to_string());
        let b = captures[1].to_string().max(captures[2].to_string());
        let dist: u32 = captures[3].parse().unwrap();
        distance_map.insert((a.clone(), b.clone()), dist);
        locations.insert(a.clone());
        locations.insert(b.clone());
    }
    let locations_vec = locations.into_iter().collect::<Vec<String>>();

    return (distance_map, locations_vec);
}

fn load_input() -> String {
    use std::fs::read_to_string;

    read_to_string("./input.txt").unwrap().trim().to_string()
}
