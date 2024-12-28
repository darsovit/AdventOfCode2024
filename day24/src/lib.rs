use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

pub struct Day24 {
    wires: HashSet<String>,
    init_values: HashMap<String, bool>,
    gates: HashMap<String, Gate>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GateOp {
    AND,
    XOR,
    OR,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Gate {
    op: GateOp,
    left: String,
    right: String,
    target: String,
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
                    "AND" => gates.insert(target.clone(), Gate{op: GateOp::AND, left, right, target}),
                    "OR" =>  gates.insert(target.clone(), Gate{op: GateOp::OR, left, right, target}),
                    "XOR" => gates.insert(target.clone(), Gate{op: GateOp::XOR, left, right, target}),
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
                    match (wire_values.get(&gate.left as &str), wire_values.get(&gate.right as &str)) {
                        (Some(a), Some(b)) => {
                            match gate.op {
                                GateOp::AND => wire_values.insert(wire_to_find, *a && *b),
                                GateOp::XOR => wire_values.insert(wire_to_find, *a ^ *b),
                                GateOp::OR  => wire_values.insert(wire_to_find, *a || *b),
                            };
                        },
                        (Some(a), None) => { stack_of_wires.push(wire_to_find); stack_of_wires.push(&gate.right); },
                        (None, Some(b)) => { stack_of_wires.push(wire_to_find); stack_of_wires.push(&gate.left); },
                        (None, None) => { stack_of_wires.push(wire_to_find); stack_of_wires.push(&gate.right); stack_of_wires.push(&gate.left); },
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

    /*
    For part2, we need to identify which wires are miswired to 'solve' the problem that the machine isn't correctly
    performing x + y = z.

    This addition machine is made up of gates, the basic theory for adding the values together are going to be

    x00 ^ y00 => z00,                                               x00 & y00 => jfb (overflow of 1, used in constructing z01)
    x01 ^ y01 => jjj, jjj ^ jfb => z01                              x01 & y01 => cpp, jfb & jjj => pss, cpp | pss -> rtc (rtc is overflow from z01 -> z02)
    x02 ^ y02 => fkn, fkn ^ rtc => z02                              x02 & y02 => vrb, rtc & fkn => dbr, dbr | vrb -> psp (psp is overflow from z02 -> z03)
    x03 ^ y03 => fhp, fhp ^ psp => z03                              x03 & y03 => ttc, fhp & psp => vkp, vkp | ttc -> rsk (rsk is overflow from z03 -> z04)
    x04 ^ y04 => cwp, cwp ^ rsk => z04                              x04 & y04 => dsn, cwp & rsk => dmh, dsn | dmh -> tsw (tsw is overflow from z04 -> z05)
    x05 ^ y05 => wwm, wwm ^ tsw => hdt (should be z05)              x05 & y05 => mkq, wwm & tsw => rnk, rnk | mkq -> z05 (should be hdt)
    ... 
    x06 ^ y06 => gwg, gwg ^ hdt => z06                              x05 & y05 => wgp, gwg & hdt => ncj, wgp | hdt -> jjg (jjg is overflow from z06 -> z07)
    x07 ^ y07 => shj, shj ^ jjg => z07                              x06 & y07 => sdq, shj & jjg => pbk, sdq | pbk => ggp (ggp is overflow from z07 -> z08)
    x08 ^ y08 => cjc, cjc ^ ggp => z08                              x08 & y08 => hrv, cjc & ggp => wvc, hrv | wvc => vkd (vkd is overflow from z08 -> z09)
    x09 ^ y09 => wqr, wqr ^ vkd => gbf (should be z09)              x09 & y09 => z09 (should be gbf),  wqr & vkd => ttm, gbf | ttm => pdk (pdk is overflow from z09 -> z10)
    x10 ^ y10 => fpp, fpp ^ pdk => z10                              x10 & y10 => cpd, fpp & pdk => fnn, cpd | fnn => tfh (tfh is overflow from z10 -> z11)
    x11 ^ y11 => jrm, jrm ^ tfh => z11                              x11 & y11 => fqp, jrm & tfh => rng, fqp | rng => rfj (rfj is overflow from z11 -> z12)
    x12 ^ y12 => msr, msr ^ rfj => z12                              x12 & y12 => hmn, msr & rfj => qnh, hmn | qnh => sfh (sfh is overflow from z12 -> z13)
    x13 ^ y13 => fpd, fpd ^ sjf => z13                              x13 & y13 => dct, fpd & sjh => ffq, dct | ffq => gnt
    x14 ^ y14 => hsh, hsh ^ gnt => z14                              x14 & y14 => bwr, hsh & gnt => bkm, bwr | bkm => fgc
    x15 ^ y15 => jgt (should be mht), mht ^ fgc => z15              x15 & y15 => mht (should be jgt), mht & fgc => nwr, jgt | nwr => shs
    x16 ^ y16 => wnd, wnd ^ shs => z16                              x16 & y16 => qsm, wnd & shs => pgd, qsm | pgd => prk
    x17 ^ y17 => jhw, jhw ^ prk => z17                              x17 & y17 => cdh, jhw & prk => wjj, cdh | wjj => qnk
    x18 ^ y18 => fcm, fcm ^ qnk => z18                              x18 & y18 => nwb, fcm & qnk => pnt, nwb | pnt => bnk
    x19 ^ y19 => qww, qww ^ bnk => z19                              x19 & y19 => sgc, qww & bnk => frn, sgc | frn => kbw
    x20 ^ y20 => cnq, cnq ^ kbw => z20                              x20 & y20 => nqw, cnq & kbw => tdh, nqw | tdh => bfg
    x21 ^ y21 => rsw, rsw ^ bfg => z21                              x21 & y21 => rkv, rsw & bfg => mbt, rkv | mbt => tmd
    x22 ^ y22 => fsp, fsp ^ tmd => z22                              x22 & y22 => dvc, fsp & tmd => rtw, dvc | rtw => chk
    x23 ^ y23 => tqk, tqk ^ chk => z23                              x23 & y23 => sjk, tqk & chk => mnm, sjk | mnm => gmj
    x24 ^ y24 => swf, swf ^ gmj => z24                              x24 & y24 => dwp, swf & gmj => ccj, dwp | ccj => grc
    x25 ^ y25 => tqf, tqf ^ grc => z25                              x25 & y25 => dmw, tqf & grc => grd, dmw | grd => dgc
    x26 ^ y26 => vwb, vwb ^ dgc => z26                              x26 & y26 => hts, vwb & dgc => hqr, hts | hqr => djp
    x27 ^ y27 => vsk, vsk ^ djp => z27                              x27 & y27 => sfr, vsk & djp => wkn, sfr | wkn => jsd
    x28 ^ y28 => kbc, kbc ^ jsd => z28                              x28 & y28 => bmh, kbc & jsd => jbf, bmh | jbf => qdw
    x29 ^ y29 => mhh, mhh ^ qdw => z29                              x29 & y29 => jnk, mhh & qdw => hdf, jnk | hdf => nvv
    x30 ^ y30 => dpr, dpr ^ nvv => nbf (should be z30)                              x30 & y30 => kqh, dpr & nvv => z30 (should be nbf), kqh | nbf => rrc
    x31 ^ y31 => bsn, bsn ^ rrc => z31                              x31 & y31 => mfb, bsn & rrc => vfs, mfb | vfs => ssr

    x44 ^ y44 => wdq, wdq ^ ggg => z44                              x44 & y44 => qhs, ggg & wdq => vkm, vkm | qhs -> z45 (z45 is directly overflowed into)


gbf,hdt,jgt,mht,nbf,z05,z09,z30
     */

    fn get_expected_x_and_y_wires_from_z(z_value_wire: &str) -> (String, String) {
        let value_wire_id = Regex::new(r"^z(\d+)$").unwrap();
        let caps = value_wire_id.captures(z_value_wire).unwrap();
        let x_wire = format!("x{}", &caps[1]);
        let y_wire = format!("y{}", &caps[1]);
        (x_wire, y_wire)
    }

    fn get_expected_xy_combine_gates(&self, expected_x_wire: &str, expected_y_wire: &str) -> (&Gate, &Gate) {
        let mut xor_gate: Option<&Gate> = None;
        let mut and_gate: Option<&Gate> = None;
        for (result_wire, gate) in &self.gates {
            if (gate.left == expected_x_wire && gate.right == expected_y_wire) || (gate.left == expected_y_wire && gate.right == expected_x_wire) {
                if gate.op == GateOp::XOR { xor_gate = Some(gate); }
                else if gate.op == GateOp::AND { and_gate = Some(gate); }
            }
        }
        let (Some(xor), Some(and)) = (xor_gate, and_gate) else { panic!("Failed to find gates"); };
        (xor, and)
    }

/*
    fn part2(&self) -> String {

        let value_wire_re = Regex::new(r"^(x|y|z)\d+").unwrap();
        let mut z_value_wires = Vec::<&str>::new();
        let mut y_value_wires = HashSet::<&str>::new();
        let mut x_value_wires = HashSet::<&str>::new();

        for wire in &self.wires {
            if let Some(caps) = value_wire_re.captures(wire) {
                if &caps[1] == "x" { x_value_wires.insert(wire); }
                else if &caps[1] == "y" { y_value_wires.insert(wire); }
                else { assert_eq!("z", &caps[1]); z_value_wires.push(wire); }
            }
        }

        let mut and_gates = HashSet::<&Gate>::new();
        let mut xor_gates = HashSet::<&Gate>::new();
        let mut or_gates  = HashSet::<&Gate>::new();

        for (result, gate) in &self.gates {
            match gate.op {
                GateOp::AND => and_gates.insert(&gate),
                GateOp::XOR => xor_gates.insert(&gate),
                GateOp::OR  => or_gates.insert(&gate),
            };
        }

        z_value_wires.sort();
        y_value_wires.sort();
        x_value_wires.sort();

        let bad_gate_list: Vec::<&Gate, z_value_wire>::new();

        struct AdderGates<'a> {
            z_value_wire: String,
            x_xor_y: Option<&'a Gate>,
            xy_xor_prev_overflow: Option<&'a Gate>,
            overflow_x_and_y: Option<&'a Gate>,
            overflow_prev_overflow_and_x_xor_y: Option<&'a Gate>,
            overflow_gate: Option<&'a Gate>,
        }

        for (index, z_value_wire) in (&z_value_wires).into_iter().enumerate() {
            let (expected_x_wire, expected_y_wire) = Self::get_expected_x_and_y_wires_from_z(z_value_wire);
            let (expected_x_xor_y_gate, expected_overflow_calc_x_and_y_gate) = self.get_expected_xy_combine_gates(&expected_x_wire, &expected_y_wire);
            if index == 0 {

            }
        }
        /*
        let overflow_wire: &str = &z_value_wires.last().unwrap();

        for (index, z_value_wire) in (&z_value_wires).into_iter().rev().enumerate() {
            if index > 0 {
                let (expected_x_wire, expected_y_wire) = Self::get_expected_x_and_y_wires_from_z(z_value_wire);
                // unvalidated result wires
                let (expected_x_xor_y_gate, expected_overflow_calc_x_and_y_gate) = self.get_expected_xy_combine_gates(&expected_x_wire, &expected_y_wire);
                let expected_overflow_gate = (&self.gates).get(overflow_wire).unwrap();             
                let z_result_gate = (&self.gates).get(z_value_wire as &str).unwrap();

                if expected_x_xor_y_gate.op != GateOp::XOR {
                    bad_gate_list
                }
                if expected_overflow_calc_x_and_y_gate.op != GateOp::AND {

                }

                let mut xy_xor_prev_overflow: Option<&Gate> = None;  // 
                let mut overflow_from_prev_overflow_and_x_xor_y: Option<&Gate> = None;  // wdq & ggg (vkm)
                let mut overflow_gate: Option<&Gate> = None;         // qhs | vkm -> (z45)


                
                if z_result_gate.left != expected_x_xor_y_gate.target || z_result_gate.right != expected_x_xor_y_gate.target {
                    // expected_x_xor_y_gate has wrong result wire
                    // One of the wires should be the previous overflow wire, the other x_xor_y
                }


                if let Some(_(left_wire, right_wire, target_wire)) = (&self.gates).get(overflow_wire) {
                    overflow_gate = Some(&gate);
                    match ((&self.gates).get(gate.0), (&self.gates).get(gate.1)) {

                    }
                }
            }
            else {

            }
        }
        */
        "".to_string()
    }
    */
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
