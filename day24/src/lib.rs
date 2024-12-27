use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

pub struct Day24 {
    wires: HashSet<String>,
    init_values: HashMap<String, bool>,
    gates: HashMap<String, Gate>,
}

pub enum Gate {
    AND(String, String),
    XOR(String, String),
    OR(String, String),
}

impl Day24 {

    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let init_values_re = Regex::new(r"^([a-z0-9]+): (0|1)$").unwrap();
        let gate_connections_re = Regex::new(r"^([a-z0-9]+) (AND|OR|XOR) ([a-z0-9]+) -> ([a-z0-9]+)$").unwrap();

        let mut wires = HashSet::<String>::new();
        let mut init_values = HashMap::<String, bool>::new();
        let mut gates       = HashMap::<String, Gate>::new();
        let mut init_values_done = false;

        for line in lines {
            if !init_values_done && line == "" {
                init_values_done = true;
            } else if !init_values_done {
                let init_values_caps = init_values_re.captures(line).unwrap();
                let target = init_values_caps[1].to_string();
                wires.insert(target.clone());
                init_values.insert(target, &init_values_caps[2] == "1");
            } else {
                assert!(init_values_done);
                let gates_cap = gate_connections_re.captures(line).unwrap();
                let target = gates_cap[4].to_string();
                let left   = gates_cap[1].to_string();
                let right  = gates_cap[3].to_string();
                let gate   = &gates_cap[2];
                wires.insert(target.clone());
                wires.insert(left.clone());
                wires.insert(right.clone());
                match gate {
                    "AND" => gates.insert(target, Gate::AND(left, right)),
                    "OR" =>  gates.insert(target, Gate::OR(left, right)),
                    "XOR" => gates.insert(target, Gate::XOR(left, right)),
                    _ => todo!()
                };
            }
        }
        Day24{wires, init_values, gates}
    }

    pub fn part1(&self) -> usize {
        let mut stack_of_wires = Vec::<&str>::new();

        let z_wire_re = Regex::new(r"z\d+").unwrap();
        let mut z_value_wires = Vec::<&str>::new();

        for wire in &self.wires {
            if z_wire_re.is_match(&wire) {
                let z_wire = self.wires.get(&wire as &str).unwrap();
                z_value_wires.push(&z_wire);
            }
        }
        z_value_wires.sort();

        let mut wire_values = HashMap::<&str, bool>::new();
        for (wire, initial_value) in &self.init_values {
            wire_values.insert(&wire, *initial_value);
        }

        for z_value_wire in &z_value_wires {
            if let None = wire_values.get(z_value_wire) {
                stack_of_wires.push(z_value_wire);
            }
        }

        while stack_of_wires.len() > 0 {
            let wire_to_find = stack_of_wires.pop().unwrap();
            if let None = wire_values.get(wire_to_find) {
                if let Some(gate) = self.gates.get(wire_to_find) {
                    match gate {
                        Gate::AND(left, right) => {
                            match (wire_values.get(&left as &str), wire_values.get(&right as &str)) {
                                (Some(a), Some(b)) => { wire_values.insert(wire_to_find, *a && *b); },
                                (Some(a), None)    => { if *a { stack_of_wires.push(wire_to_find); stack_of_wires.push(right); } else { wire_values.insert(wire_to_find, *a); } },
                                (None, Some(b))    => { if *b { stack_of_wires.push(wire_to_find); stack_of_wires.push(left); } else { wire_values.insert(wire_to_find, *b); } },
                                (None, None)       => { stack_of_wires.push(wire_to_find); stack_of_wires.push(left); stack_of_wires.push(right); }
                            }
                        },
                        Gate::OR(left, right) => {
                            match (wire_values.get(&left as &str), wire_values.get(&right as &str)) {
                                (Some(a), Some(b)) => { wire_values.insert(wire_to_find, *a || *b); },
                                (Some(a), None)    => { if !*a { stack_of_wires.push(wire_to_find); stack_of_wires.push(right); } else { wire_values.insert(wire_to_find, *a); } },
                                (None, Some(b))    => { if !*b { stack_of_wires.push(wire_to_find); stack_of_wires.push(left); } else { wire_values.insert(wire_to_find, *b); }},
                                (None, None)       => { stack_of_wires.push(wire_to_find); stack_of_wires.push(left); stack_of_wires.push(right); }
                            }
                        },
                        Gate::XOR(left, right) => {
                            match (wire_values.get(&left as &str), wire_values.get(&right as &str)) {
                                (Some(a), Some(b)) => { wire_values.insert(wire_to_find, *a ^ *b); },
                                (Some(_), None)    => { stack_of_wires.push(wire_to_find); stack_of_wires.push(right); },
                                (None, Some(_))    => { stack_of_wires.push(wire_to_find); stack_of_wires.push(left); },
                                (None, None)       => { stack_of_wires.push(wire_to_find); stack_of_wires.push(left); stack_of_wires.push(right); }
                            }
                        }
                    }
                }
            }
        }

        let mut value: usize = 0;

        for z_wire in (&z_value_wires).into_iter().rev() {
            match wire_values.get(&z_wire as &str) {
                Some(true) => value = (value << 1) | 1,
                Some(false) => value = (value << 1) | 0,
                None => panic!("Missing z_wire {} value still!", z_wire),
            }
        }

        value
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    const SMALL_SAMPLE_INPUT: &str =
"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const LARGE_SAMPLE_INPUT: &str =
"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn small_sample_input_part1() {
        let day = Day24::new(SMALL_SAMPLE_INPUT.lines());
        assert_eq!(4, day.part1());
    }

    #[test]
    fn large_sample_input_part2() {
        let day = Day24::new(LARGE_SAMPLE_INPUT.lines());
        assert_eq!(2024, day.part1());
    }
}
