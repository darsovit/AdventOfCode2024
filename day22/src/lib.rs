pub struct Day22 {
    initial: Vec<u64>
}

impl Day22 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut initial = Vec::new();
        for line in lines {
            initial.push(line.parse::<u64>().unwrap());
        }
        Day22{initial}
    }

    fn next_secret_number(secret: u64) -> u64 {
        const PRUNE: u64 = 16777216;
        let secret = ((secret * 64) ^ secret)   % PRUNE;
        let secret = (secret / 32) ^ secret;
        let secret = ((secret * 2048) ^ secret) % PRUNE;
        secret
    }

    fn get_nth_secret(secret: u64, n: usize) -> u64 {
        let mut secret = secret;
        for _ in 0..n {
            secret = Self::next_secret_number(secret);
        }
        secret
    }
    pub fn part1(&self) -> u64 {
        let mut sum_of_2000th_secrets = 0;
        for val in &self.initial {
            sum_of_2000th_secrets += Self::get_nth_secret(*val, 2000);
        }
        sum_of_2000th_secrets
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    const SAMPLE_INPUT: &str =
"1
10
100
2024";

    #[test]
    fn part1_with_sample_input() {
        let day = Day22::new(SAMPLE_INPUT.lines());
        assert_eq!(37327623, day.part1());
    }

    #[test]
    fn first_secret_from_123() {
        assert_eq!(15887950, Day22::next_secret_number(123));
    }

    #[test]
    fn second_secret_from_123() {
        assert_eq!(16495136, Day22::next_secret_number(15887950));
        assert_eq!(16495136, Day22::get_nth_secret(123, 2));
    }

    #[test]
    fn third_secret_from_123() {
        assert_eq!(527345, Day22::next_secret_number(16495136));
        assert_eq!(527345, Day22::get_nth_secret(123, 3));
    }
}
