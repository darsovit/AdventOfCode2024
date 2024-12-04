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
            println!("{}", line);
            for pairs in re.captures_iter(line).map(|caps| {
                let (_, [multiplicand, multiplier]) = caps.extract();
                (multiplicand.parse::<i32>().unwrap(), multiplier.parse::<i32>().unwrap())
            }) {
                sum_of_good_multiplies += pairs.0 * pairs.1
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
}
