use regex::Regex;
use std::collections::HashMap;
use std::env::args;
use std::iter::Iterator;
use std::sync::OnceLock;

#[derive(Debug)]
enum InputCommand {
    Not {
        source: String,
        target: String,
    },
    NumAnd {
        value: u16,
        b: String,
        target: String,
    },
    And {
        a: String,
        b: String,
        target: String,
    },
    Or {
        a: String,
        b: String,
        target: String,
    },
    Lshift {
        source: String,
        value: u16,
        target: String,
    },
    Rshift {
        source: String,
        value: u16,
        target: String,
    },
    Value {
        value: u16,
        target: String,
    },
    Assign {
        source: String,
        target: String,
    },
}

#[derive(Copy, Clone)]
enum LogicGate {
    NumAnd { value: u16, b: usize },
    Not { source: usize },
    And { a: usize, b: usize },
    Or { a: usize, b: usize },
    Lshift { source: usize, value: u16 },
    Rshift { source: usize, value: u16 },
    Value { value: u16 },
    Assign { source: usize },
}

#[derive(Copy, Clone)]
struct Wire {
    value: Option<u16>,
    source: usize,
}

struct Circuit {
    wires: HashMap<usize, Wire>,
    wire_names: HashMap<String, usize>,
    logic_gates: HashMap<usize, LogicGate>,
}

impl Circuit {
    fn build_from_commands(commands: impl Iterator<Item = InputCommand>) -> Circuit {
        let mut wires = HashMap::<usize, Wire>::new();
        let mut wire_names = HashMap::<String, usize>::new();
        let mut logic_gates = HashMap::<usize, LogicGate>::new();

        let commands_vec = commands.collect::<Vec<InputCommand>>();

        // Scan the commands for wire names.
        for command in &commands_vec {
            let target = match command {
                InputCommand::NumAnd {
                    value: _,
                    b: _,
                    target,
                } => target,
                InputCommand::Not { source: _, target } => target,
                InputCommand::And { a: _, b: _, target } => target,
                InputCommand::Or { a: _, b: _, target } => target,
                InputCommand::Lshift {
                    source: _,
                    value: _,
                    target,
                } => target,
                InputCommand::Rshift {
                    source: _,
                    value: _,
                    target,
                } => target,
                InputCommand::Value { value: _, target } => target,
                InputCommand::Assign { source: _, target } => target,
            };
            let wire_index = wires.len();
            wires.insert(
                wire_index,
                Wire {
                    value: None,
                    source: 0,
                },
            );
            wire_names.insert(target.clone(), wire_index);
        }

        // Build the logic gates and connect them.
        for command in &commands_vec {
            let (logic_gate, target_id) = match command {
                InputCommand::NumAnd { value, b, target } => {
                    let b_wire_id = wire_names.get(b).unwrap();
                    let target_wire_id = wire_names.get(target).unwrap();
                    (
                        LogicGate::NumAnd {
                            b: *b_wire_id,
                            value: *value,
                        },
                        target_wire_id,
                    )
                }
                InputCommand::Not { source, target } => {
                    let source_wire_id = wire_names.get(source).unwrap();
                    let target_wire_id = wire_names.get(target).unwrap();
                    (
                        LogicGate::Not {
                            source: *source_wire_id,
                        },
                        target_wire_id,
                    )
                }
                InputCommand::And { a, b, target } => {
                    let a_wire_id = wire_names.get(a).unwrap();
                    let b_wire_id = wire_names.get(b).unwrap();
                    let target_wire_id = wire_names.get(target).unwrap();
                    (
                        LogicGate::And {
                            a: *a_wire_id,
                            b: *b_wire_id,
                        },
                        target_wire_id,
                    )
                }
                InputCommand::Or { a, b, target } => {
                    let a_wire_id = wire_names.get(a).unwrap();
                    let b_wire_id = wire_names.get(b).unwrap();
                    let target_wire_id = wire_names.get(target).unwrap();
                    (
                        LogicGate::Or {
                            a: *a_wire_id,
                            b: *b_wire_id,
                        },
                        target_wire_id,
                    )
                }
                InputCommand::Lshift {
                    source,
                    value,
                    target,
                } => {
                    let source_wire_id = wire_names.get(source).unwrap();
                    let target_wire_id = wire_names.get(target).unwrap();
                    (
                        LogicGate::Lshift {
                            source: *source_wire_id,
                            value: *value,
                        },
                        target_wire_id,
                    )
                }
                InputCommand::Rshift {
                    source,
                    value,
                    target,
                } => {
                    let source_wire_id = wire_names.get(source).unwrap();
                    let target_wire_id = wire_names.get(target).unwrap();
                    (
                        LogicGate::Rshift {
                            source: *source_wire_id,
                            value: *value,
                        },
                        target_wire_id,
                    )
                }
                InputCommand::Value { value, target } => {
                    let target_wire_id = wire_names.get(target).unwrap();
                    (LogicGate::Value { value: *value }, target_wire_id)
                }
                InputCommand::Assign { source, target } => {
                    let source_wire_id = wire_names.get(source).unwrap();
                    let target_wire_id = wire_names.get(target).unwrap();
                    (
                        LogicGate::Assign {
                            source: *source_wire_id,
                        },
                        target_wire_id,
                    )
                }
            };
            let logic_gate_id = logic_gates.len();
            logic_gates.insert(logic_gate_id, logic_gate);
            let wire = wires.get_mut(&target_id).unwrap();
            wire.source = logic_gate_id;
        }

        Circuit {
            wires,
            wire_names,
            logic_gates,
        }
    }

    fn get_value_from_gate(&mut self, logic_gate_id: usize) -> u16 {
        let logic_gate = *self.logic_gates.get(&logic_gate_id).unwrap();
        match logic_gate {
            LogicGate::NumAnd { value, b } => {
                let b_value = self.get_wire_value(b);
                value & b_value
            }
            LogicGate::Not { source } => {
                let source_value = self.get_wire_value(source);
                !source_value
            }
            LogicGate::And { a, b } => {
                let a_value = self.get_wire_value(a);
                let b_value = self.get_wire_value(b);
                a_value & b_value
            }
            LogicGate::Or { a, b } => {
                let a_value = self.get_wire_value(a);
                let b_value = self.get_wire_value(b);
                a_value | b_value
            }
            LogicGate::Lshift { source, value } => {
                let source_value = self.get_wire_value(source);
                source_value << value
            }
            LogicGate::Rshift { source, value } => {
                let source_value = self.get_wire_value(source);
                source_value >> value
            }
            LogicGate::Value { value } => value,
            LogicGate::Assign { source } => {
                let source_value = self.get_wire_value(source);
                source_value
            }
        }
    }

    fn get_wire_value(&mut self, wire_id: usize) -> u16 {
        let wire = *self.wires.get(&wire_id).unwrap();
        match wire.value {
            Some(value) => value,
            None => {
                let value = self.get_value_from_gate(wire.source);
                let new_wire = Wire {
                    value: Some(value),
                    source: wire.source,
                };
                self.wires.insert(wire_id, new_wire);
                value
            }
        }
    }

    fn get_wire_id_for_name(&self, name: &String) -> usize {
        *self.wire_names.get(name).unwrap()
    }

    fn add_logic_gate(&mut self, logic_gate: LogicGate) -> usize {
        let logic_gate_id = self.logic_gates.len();
        self.logic_gates.insert(logic_gate_id, logic_gate);
        logic_gate_id
    }

    fn update_wire_source(&mut self, wire_id: usize, logic_gate_id: usize) {
        let wire = self.wires.get(&wire_id).unwrap();
        self.wires.insert(
            wire_id,
            Wire {
                value: wire.value,
                source: logic_gate_id,
            },
        );
    }

    fn clear_wire_values(&mut self) {
        for wire_id in 0..self.wires.len() {
            let wire = self.wires.get(&wire_id).unwrap();
            self.wires.insert(
                wire_id,
                Wire {
                    value: None,
                    source: wire.source,
                },
            );
        }
    }
}

fn parse_line(line: &str) -> InputCommand {
    const NUMAND: usize = 0;
    const AND: usize = 1;
    const OR: usize = 2;
    const LSHIFT: usize = 3;
    const RSHIFT: usize = 4;
    const NOT: usize = 5;
    const VALUE: usize = 6;
    const ASSIGN: usize = 7;

    static RE: OnceLock<Vec<Regex>> = OnceLock::new();
    let patterns = &RE.get_or_init(|| {
        vec![
            Regex::new(r"(\d+) AND (\w+) -> (\w+)").unwrap(),
            Regex::new(r"(\w+) AND (\w+) -> (\w+)").unwrap(),
            Regex::new(r"(\w+) OR (\w+) -> (\w+)").unwrap(),
            Regex::new(r"(\w+) LSHIFT (\d+) -> (\w+)").unwrap(),
            Regex::new(r"(\w+) RSHIFT (\d+) -> (\w+)").unwrap(),
            Regex::new(r"NOT (\w+) -> (\w+)").unwrap(),
            Regex::new(r"(\d+) -> (\w+)").unwrap(), // Matches LSHIFT/RSHIFT early
            Regex::new(r"(\w+) -> (\w+)").unwrap(), // Matches AND/OR early
        ]
    });

    let info = patterns
        .into_iter()
        .enumerate()
        .find_map(|(index, re)| match re.captures(line) {
            Some(captures) => Some((index, captures)),
            None => None,
        });

    let (match_index, captures) = match info {
        Some(captures_info) => captures_info,
        None => {
            panic!("{}", line);
        }
    };

    match match_index {
        NUMAND => {
            let value = captures[1].parse().unwrap();
            let b = captures[2].to_string();
            let target = captures[3].to_string();
            InputCommand::NumAnd { value, b, target }
        }
        AND | OR => {
            let a = String::from(&captures[1]);
            let b = String::from(&captures[2]);
            let target = String::from(&captures[3]);
            match match_index {
                AND => InputCommand::And { a, b, target },
                OR => InputCommand::Or { a, b, target },
                _ => {
                    unreachable!()
                }
            }
        }
        LSHIFT | RSHIFT => {
            let source = String::from(&captures[1]);
            let value = captures[2].parse().unwrap();
            let target = String::from(&captures[3]);
            match match_index {
                LSHIFT => InputCommand::Lshift {
                    source,
                    value,
                    target,
                },
                RSHIFT => InputCommand::Rshift {
                    source,
                    value,
                    target,
                },
                _ => {
                    unreachable!()
                }
            }
        }
        NOT | ASSIGN => {
            let source = String::from(&captures[1]);
            let target = String::from(&captures[2]);
            match match_index {
                NOT => InputCommand::Not { source, target },
                ASSIGN => InputCommand::Assign { source, target },
                _ => {
                    unreachable!()
                }
            }
        }
        VALUE => {
            let value = captures[1].parse().unwrap();
            let target = String::from(&captures[2]);
            InputCommand::Value { value, target }
        }
        _ => {
            panic!("Invalid match ID {}", match_index)
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = InputCommand> + '_ {
    input.split("\n").map(|line| parse_line(line))
}

fn part1(input: String) -> u16 {
    let commands = parse_input(input.as_str());
    let mut circuit = Circuit::build_from_commands(commands);
    let wire_id = circuit.get_wire_id_for_name(&"a".to_string());
    circuit.get_wire_value(wire_id)
}

fn part2(input: String) -> u16 {
    let commands = parse_input(input.as_str());
    let mut circuit = Circuit::build_from_commands(commands);
    let wire_a_id = circuit.get_wire_id_for_name(&"a".to_string());
    let wire_a_value = circuit.get_wire_value(wire_a_id);
    let new_lg_id = circuit.add_logic_gate(LogicGate::Value {
        value: wire_a_value,
    });
    let wire_b_id = circuit.get_wire_id_for_name(&"b".to_string());
    circuit.update_wire_source(wire_b_id, new_lg_id);
    circuit.clear_wire_values();
    circuit.get_wire_value(wire_a_id)
}

fn load_input() -> String {
    use std::fs::read_to_string;

    read_to_string("./input.txt").unwrap().trim().to_string()
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
