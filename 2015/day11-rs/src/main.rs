use itertools::Itertools;
use std::collections::HashMap;
use std::env::args;
use std::sync::OnceLock;

#[derive(Debug)]
struct Password {
    digits: [u8; 8],
}

fn get_number_for_letter(letter: char) -> u8 {
    static LETTERS_TO_NUMBERS: OnceLock<HashMap<char, u8>> = OnceLock::new();
    let letters_to_numbers = LETTERS_TO_NUMBERS.get_or_init(|| {
        HashMap::from([
            ('a', 0),
            ('b', 1),
            ('c', 2),
            ('d', 3),
            ('e', 4),
            ('f', 5),
            ('g', 6),
            ('h', 7),
            ('j', 8),
            ('k', 9),
            ('m', 10),
            ('n', 11),
            ('p', 12),
            ('q', 13),
            ('r', 14),
            ('s', 15),
            ('t', 16),
            ('u', 17),
            ('v', 18),
            ('w', 19),
            ('x', 20),
            ('y', 21),
            ('z', 22),
        ])
    });
    *letters_to_numbers.get(&letter).unwrap()
}

fn get_letter_for_number(number: u8) -> char {
    static NUMBERS_TO_LETTERS: OnceLock<HashMap<u8, char>> = OnceLock::new();
    let numbers_to_letters = NUMBERS_TO_LETTERS.get_or_init(|| {
        HashMap::from([
            (0, 'a'),
            (1, 'b'),
            (2, 'c'),
            (3, 'd'),
            (4, 'e'),
            (5, 'f'),
            (6, 'g'),
            (7, 'h'),
            (8, 'j'),
            (9, 'k'),
            (10, 'm'),
            (11, 'n'),
            (12, 'p'),
            (13, 'q'),
            (14, 'r'),
            (15, 's'),
            (16, 't'),
            (17, 'u'),
            (18, 'v'),
            (19, 'w'),
            (20, 'x'),
            (21, 'y'),
            (22, 'z'),
        ])
    });
    *numbers_to_letters.get(&number).unwrap()
}

impl Password {
    const CARRY_OVER: u8 = 23;

    fn increment(&mut self) {
        let mut digit_index = 7;
        let mut carry = true;
        while carry {
            let mut digit = self.digits[digit_index];
            digit += 1;
            if digit == Password::CARRY_OVER {
                digit = 0;
                carry = true;
            } else {
                carry = false;
            }
            self.digits[digit_index] = digit;
            digit_index -= 1;
        }
    }

    fn from_string(string: String) -> Password {
        assert!(string.len() == 8);
        let digits: [u8; 8] = string
            .chars()
            .map(|c| get_number_for_letter(c))
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        Password { digits }
    }

    fn to_string(&self) -> String {
        self.digits
            .into_iter()
            .map(|n| get_letter_for_number(n))
            .collect()
    }

    fn validate(&self) -> bool {
        let mut increasing_straights_found = 0u32;

        for t in self.digits.into_iter().tuple_windows::<(u8, u8, u8)>() {
            if t.1 == t.0 + 1 && t.2 == t.1 + 1 {
                increasing_straights_found += 1;
            }
        }

        let mut doubles = Vec::new();

        for (i, t) in self
            .digits
            .into_iter()
            .tuple_windows::<(u8, u8)>()
            .enumerate()
        {
            if t.0 == t.1 {
                doubles.push(i);
            }
        }

        let mut num_doubles = doubles.len();

        for t in doubles.into_iter().tuple_windows::<(usize, usize)>() {
            if t.0 + 1 == t.1 {
                num_doubles -= 1;
            }
        }

        increasing_straights_found == 1 && num_doubles >= 2
    }
}

fn main() {
    let pargs = Vec::from_iter(args());
    if pargs.len() != 2 {
        eprintln!("Please specify 'part1' or 'part2'");
        return;
    }
    let input = "hepxcrrq".to_string();
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

fn part1(input: String) -> String {
    let mut password = Password::from_string(input);
    while !password.validate() {
        password.increment();
    }
    password.to_string()
}

fn part2(input: String) -> String {
    let mut password = Password::from_string(input);
    while !password.validate() {
        password.increment();
    }
    password.increment();
    while !password.validate() {
        password.increment();
    }
    password.to_string()
}
