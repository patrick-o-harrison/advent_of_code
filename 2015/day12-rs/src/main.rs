use regex::Regex;
use serde_json;
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
    find_numbers(input).into_iter().sum()
}

fn part2(input: String) -> i64 {
    let json_values = serde_json::from_str(&input).unwrap();
    find_non_red_values(&json_values)
}

fn find_numbers(string: String) -> Vec<i32> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(&string)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

fn find_non_red_values(json: &serde_json::Value) -> i64 {
    match json {
        serde_json::Value::Null => 0,
        serde_json::Value::Bool(_) => 0,
        serde_json::Value::Number(number) => number.as_i64().unwrap(),
        serde_json::Value::String(_) => 0,
        serde_json::Value::Array(vec) => vec.into_iter().map(|v| find_non_red_values(v)).sum(),
        serde_json::Value::Object(map) => {
            let red_value_found = map.values().any(|v| match v {
                serde_json::Value::String(s) => s.as_str() == "red",
                _ => false,
            });

            if !red_value_found {
                map.values().map(|v| find_non_red_values(v)).sum()
            } else {
                0
            }
        }
    }
}

fn load_input() -> String {
    use std::fs::read_to_string;

    String::from(read_to_string("./input.txt").unwrap().trim())
}
