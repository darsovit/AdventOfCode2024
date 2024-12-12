use regex::Regex;

struct EquationParts {
    sum: u32,
    parts: Vec<u32>,
}

pub struct Day07 {
    equations: Vec<EquationParts>,
}

impl Day07 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut equations: Vec<EquationParts> = Vec::new();
        let equation_regex = Regex::new(r"(?<sum>\d+): (?<parts>(\d+ )\d+)").unwrap();
        for line in lines {
            if let Some(captures) = equation_regex.captures(line) {
                let parts = captures["parts"].split(" ").map(|e| e.parse::<u32>().unwrap()).collect();
                let sum = captures["sum"].parse::<u32>().unwrap();
                equations.push(EquationParts{sum, parts});
            }
        }
        Day07{equations}
    }

    pub fn part1(&self) -> u32 {
        let mut sum = 0;
        for equation in &self.equations[..] {
            println!("{:?} = {:?}", equation.sum, equation.parts);
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_LINES: &str =
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    #[test]
    fn sample_with_part1_is_3749() {
        let day = Day07::new(SAMPLE_LINES.lines());
        assert_eq!(3749, day.part1());
    }
}