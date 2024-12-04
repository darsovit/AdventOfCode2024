use regex::Regex;

pub struct Day03<'a> {
    input: Vec<&'a str>
}

impl<'a> Day03<'a> {
    pub fn new(lines: std::str::Lines<'a>) -> Self {
        Day03{input: lines.collect()}
    }

    pub fn part1(&self) -> i32 {
        let re = Regex::new(r"mul\((?<multiplicand>\d+),(?<multiplier>\d+)\)").unwrap();
        let mut sum_of_good_multiplies = 0;
        for line in &self.input {
            for pairs in re.captures_iter(line).map(|caps| {
                let (_, [multiplicand, multiplier]) = caps.extract();
                (multiplicand.parse::<i32>().unwrap(), multiplier.parse::<i32>().unwrap())
            }) {
                sum_of_good_multiplies += pairs.0 * pairs.1
            }
        }
        sum_of_good_multiplies
    }

    pub fn part2(&self) -> i32 {
        let instruction_re = Regex::new(r"(mul|do|don\'t)(\(\)|\(\d+,\d+\))").unwrap();
        let multiply_args_re = Regex::new(r"^\((?<multiplicand>\d+),(?<multiplier>\d+)\)$").unwrap();
        let mut enable_instructions = true;
        let mut sum_of_good_multiplies = 0;
        for line in &self.input {
            for parts in instruction_re.captures_iter(line).map(|caps| {
                let (_, [instr, args]) = caps.extract();
                (instr,args)
            }) {
                match parts {
                    ("do", "()") => { enable_instructions = true; },
                    ("don't", "()") => { enable_instructions = false; },
                    ("mul", args) => {
                        if enable_instructions {
                            let capture = multiply_args_re.captures(args);
                            if let Some(captures) = capture {
                                let multiplicand = captures["multiplicand"].parse::<i32>().unwrap();
                                let multiplier   = captures["multiplier"].parse::<i32>().unwrap();
                                sum_of_good_multiplies += multiplicand * multiplier;
                            }
                        }
                    },
                    (_, _) => {
                        // Unused combination
                    }
                }
            }
        }
        sum_of_good_multiplies
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    #[test]
    fn sample_input_results_in_161() {
        let day03 = Day03::new(SAMPLE_INPUT.lines());
        assert_eq!(161, day03.part1());
    }

    #[test]
    fn sample_input_results_in_48() {
        const SAMPLE_INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let day03 = Day03::new(SAMPLE_INPUT.lines());
        assert_eq!(48, day03.part2());
    }
}
