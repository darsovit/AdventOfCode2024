use std::cell::Cell;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
enum Operand {
    Combo(u8),
    Literal(u8),
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    ADV(Operand),
    BXL(Operand),
    BST(Operand),
    JNZ(Operand),
    BXC(Operand),
    OUT(Operand),
    BDV(Operand),
    CDV(Operand),
}

fn interpret_opcode_and_operand(opcode: u8, operand: u8) -> Instruction {
    assert!(operand < 8);
    match opcode {
        0 => Instruction::ADV(Operand::Combo(operand)),
        1 => Instruction::BXL(Operand::Literal(operand)),
        2 => Instruction::BST(Operand::Combo(operand)),
        3 => Instruction::JNZ(Operand::Literal(operand)),
        4 => Instruction::BXC(Operand::Literal(operand)),
        5 => Instruction::OUT(Operand::Combo(operand)),
        6 => Instruction::BDV(Operand::Combo(operand)),
        7 => Instruction::CDV(Operand::Combo(operand)),
        _ => { panic!("Invalid opcode: {opcode}"); }
    }
}

#[derive(Debug)]
struct Computer {
    program: Vec<Instruction>,
    ip: Cell<usize>,
    a: Cell<u32>,
    b: Cell<u32>,
    c: Cell<u32>,
}

impl Computer {
    fn new(program: Vec<Instruction>, ip: usize, a: u32, b: u32, c: u32) -> Self {
        Computer{program: program.clone(), ip: Cell::new(ip), a: Cell::new(a), b: Cell::new(b), c: Cell::new(c)}
    }

    fn operand_value(&self, operand: &Operand) -> u32 {
        match operand {
            Operand::Combo(val) => {
                if *val < 4 { return *val as u32; }
                match val {
                    4 => self.a.get(),
                    5 => self.b.get(),
                    6 => self.c.get(),
                    _ => { panic!("Invalid combo operand"); }
                }
            },
            Operand::Literal(val) => *val as u32
        }
    }

    fn handle(&self, inst: &Instruction, output: &mut Option<String>) -> Option<usize> {
        match inst {
            Instruction::ADV(operand) => {
                let numerator = self.a.get();
                let power_of_2 = self.operand_value(&operand);
                let denominator = 1 << power_of_2;
                self.a.set(numerator / denominator);
                None
            },
            Instruction::BXL(operand) => {
                let b = self.b.get();
                let value = self.operand_value(&operand);
                let xor_value = b ^ value;
                self.b.set(xor_value);
                None
            },
            Instruction::BST(operand) => {
                let value = self.operand_value(&operand);
                let value = value % 8;
                self.b.set(value);
                None
            },
            Instruction::JNZ(operand) => {
                let possible_next_ip = self.operand_value(operand);
                let a = self.a.get();
                if a == 0 {
                    None
                } else {
                    Some(possible_next_ip as usize)
                }
            },
            Instruction::BXC(_) => {
                let b = self.b.get();
                let c = self.c.get();
                let b = b ^ c;
                self.b.set(b);
                None
            },
            Instruction::BDV(operand) => {
                let numerator = self.a.get();
                let power_of_2 = self.operand_value(&operand);
                let denominator = 1 << power_of_2;
                self.b.set(numerator / denominator);
                None
            },
            Instruction::CDV(operand) => {
                let numerator = self.a.get();
                let power_of_2 = self.operand_value(&operand);
                let denominator = 1 << power_of_2;
                self.c.set(numerator / denominator);
                None
            },
            Instruction::OUT(operand) => {
                let value = self.operand_value(&operand) % 8;
                *output = Some(format!("{}", value));
                None
            }
        }
    }

    fn step(&self) -> (bool, Option<String>) {
        assert!(self.ip.get() < self.program.len());
        let mut output: Option<String> = None;
        if let Some(new_ip) = self.handle(&self.program[self.ip.get()], &mut output) {
            self.ip.set(new_ip);
        } else {
            let new_ip = self.ip.get() + 1;
            self.ip.set(new_ip);
        }
        (self.ip.get() < self.program.len(), output)
    }

    fn run(&self) -> String {
        let mut done = false;
        let mut output: Option<String> = None;
        while !done {
            let (work_to_do, possible_string) = self.step();
            done = !work_to_do;
            if let Some(string) = possible_string {
                if let Some(exist_string) = output {
                    output = Some(format!("{},{}", exist_string, string));
                }
                else {
                    output = Some(string);
                }
            }
        }
        if let Some(string) = output {
            string
        } else {
            "".to_string()
        }
    }
}

fn interpret_program(program: &Vec<u8>) -> Vec<Instruction> {
    let mut interpreted_program = Vec::new();
    assert!(program.len() % 2 == 0);
    let mut offset = 0;
    while offset < program.len() {
        interpreted_program.push(interpret_opcode_and_operand(program[offset], program[offset+1]));
        offset += 2;
    }
    interpreted_program
}

pub struct Day17 {
    computer: Computer,
}

impl Day17 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let register_re = Regex::new(r"^Register [A|B|C]: (\d+)$").unwrap();
        let program_re  = Regex::new(r"^Program: (\d(,\d)+)$").unwrap();
        let mut lines_iter = lines.into_iter();
        let register_a = register_re.captures(lines_iter.next().unwrap()).unwrap();
        let register_b = register_re.captures(lines_iter.next().unwrap()).unwrap();
        let register_c = register_re.captures(lines_iter.next().unwrap()).unwrap();
        lines_iter.next();
        let program_codes = &program_re.captures(lines_iter.next().unwrap()).unwrap()[1];

        let a = register_a[1].parse::<u32>().unwrap();
        let b = register_b[1].parse::<u32>().unwrap();
        let c = register_c[1].parse::<u32>().unwrap();

        let code: Vec<u8> = program_codes.split(",").map(|v| v.parse::<u8>().unwrap()).collect();
        let program = interpret_program(&code);

        Day17{computer: Computer::new(program, 0, a, b, c)}
    }

    pub fn part1(&self) -> String {
        self.computer.run()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str =
"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn sample_input_part1_check_output() {
        let day = Day17::new(SAMPLE_INPUT.lines());
        assert_eq!("4,6,3,5,6,3,5,2,1,0", day.part1());
    }

    #[test]
    fn instruction_creation() {
        assert_eq!(Instruction::ADV(Operand::Combo(1)), interpret_opcode_and_operand(0, 1));
        assert_eq!(Instruction::BXL(Operand::Literal(2)), interpret_opcode_and_operand(1, 2));
        assert_eq!(Instruction::BST(Operand::Combo(3)), interpret_opcode_and_operand(2, 3));
        assert_eq!(Instruction::JNZ(Operand::Literal(4)), interpret_opcode_and_operand(3, 4));
        assert_eq!(Instruction::BXC(Operand::Literal(5)), interpret_opcode_and_operand(4, 5));
        assert_eq!(Instruction::OUT(Operand::Combo(6)), interpret_opcode_and_operand(5, 6));
        assert_eq!(Instruction::BDV(Operand::Combo(7)), interpret_opcode_and_operand(6, 7));
        assert_eq!(Instruction::CDV(Operand::Combo(0)), interpret_opcode_and_operand(7, 0));
    }

    #[test]
    fn interpreted_program_creation() {
        assert_eq!(vec![Instruction::ADV(Operand::Combo(2))], interpret_program(&vec![0, 2]));
        assert_eq!(vec![Instruction::BXL(Operand::Literal(2)), Instruction::BST(Operand::Combo(3))], interpret_program(&vec![1, 2, 2, 3]));
    }

    #[test]
    fn computer_interprets_adv_correctly() {
        let computer = Computer::new(vec![Instruction::ADV(Operand::Combo(2))], 0, 4, 0, 0);
        assert_eq!((false, None), computer.step());
        assert_eq!(1, computer.a.get());
        assert_eq!(1, computer.ip.get());
    }

    #[test]
    fn computer_interprets_bxl_correctly() {
        let computer = Computer::new(vec![Instruction::BXL(Operand::Literal(7))], 0, 0, 5, 0);
        assert_eq!((false, None), computer.step());
        assert_eq!(2, computer.b.get());
        assert_eq!(1, computer.ip.get());
    }

    #[test]
    fn computer_interprets_bst_correctly() {
        let computer = Computer::new(vec![Instruction::BST(Operand::Combo(4))], 0, 19, 0, 0);
        assert_eq!((false, None), computer.step());
        assert_eq!(3, computer.b.get());
        assert_eq!(1, computer.ip.get());
    }

    #[test]
    fn computer_interprets_jnz_with_jump_correctly() {
        let computer = Computer::new(vec![Instruction::JNZ(Operand::Literal(0))], 0, 1, 0, 0);
        assert_eq!((true, None), computer.step());
        assert_eq!(0, computer.ip.get());
    }

    #[test]
    fn computer_interprets_jnz_without_jump_correctly() {
        let computer = Computer::new(vec![Instruction::JNZ(Operand::Literal(0))], 0, 0, 0, 0);
        assert_eq!((false, None), computer.step());
        assert_eq!(1, computer.ip.get());
    }

    #[test]
    fn computer_interprets_bxc_correctly() {
        let computer = Computer::new(vec![Instruction::BXC(Operand::Literal(0))], 0, 0, 4, 3);
        assert_eq!((false, None), computer.step());
        assert_eq!(1, computer.ip.get());
        assert_eq!(7, computer.b.get());
    }

    #[test]
    fn computer_interprets_bdv_correctly() {
        let computer = Computer::new(vec![Instruction::BDV(Operand::Combo(2))], 0, 12, 0, 0);
        assert_eq!((false, None), computer.step());
        assert_eq!(3, computer.b.get());
        assert_eq!(12, computer.a.get());
        assert_eq!(1, computer.ip.get());
    }

    #[test]
    fn computer_interprets_cdv_correctly() {
        let computer = Computer::new(vec![Instruction::CDV(Operand::Combo(2))], 0, 16, 0, 0);
        assert_eq!((false, None), computer.step());
        assert_eq!(4, computer.c.get());
        assert_eq!(16, computer.a.get());
        assert_eq!(1, computer.ip.get());
    }

    #[test]
    fn computer_interprets_out_correctly() {
        let computer = Computer::new(vec![Instruction::OUT(Operand::Combo(4))], 0, 21, 0, 0);
        let step_response = computer.step();
        assert_eq!(false, step_response.0);
        if let Some(output) = step_response.1 {
            assert_eq!("5", &output);
        }
        assert_eq!(21, computer.a.get());
    }
}
