use std::collections::VecDeque;

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

    fn calculate_bucket(bananas: &VecDeque<u8>) -> usize {
        let bananas: Vec<usize> = (&bananas).into_iter().map(|v| *v as usize).collect();
        assert_eq!(5, bananas.len());
        ((((((9 + bananas[1]- bananas[0]) * 19) + (9 + bananas[2] - bananas[1])) * 19) + (9 + bananas[3] - bananas[2])) * 19) + (9 + bananas[4] - bananas[3])
    }

    fn walk_through_secrets_to_setup_options(secret: u64, n: usize, options: &mut Vec<Option<u8>>) {
        let mut banana_sequence = VecDeque::<u8>::new();
        assert!(n > 5);

        let mut secret = secret;
        for _ in 0..5 {
            banana_sequence.push_back((secret % 10) as u8);
            secret = Self::next_secret_number(secret);
        }

        for _ in 5..n {
            let pos = Self::calculate_bucket(&banana_sequence);
            if let None = options[pos] {
                options[pos] = Some(*banana_sequence.back().unwrap());
            }
            banana_sequence.pop_front();
            banana_sequence.push_back((secret % 10) as u8);
            secret = Self::next_secret_number(secret);
        }
    }

    pub fn part2(&self) -> u32 {
        let mut max_bananas_for_sequence: Vec<u32> = vec![0; 130322];
        for val in &self.initial {
            let mut this_individuals_options: Vec<Option<u8>> = vec![None; 130322];

            Self::walk_through_secrets_to_setup_options(*val, 2000, &mut this_individuals_options);
            for i in 0..130322 {
                if let Some(val) = this_individuals_options[i] {
                    max_bananas_for_sequence[i] += val as u32;
                }
            }
        }

        let mut max_bananas = 0;
        for num_bananas in max_bananas_for_sequence {
            if num_bananas > max_bananas { max_bananas = num_bananas; }
        }
        max_bananas
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

    #[test]
    fn test_bucket_maker() {
        let mut bananas = VecDeque::<u8>::new();
        bananas.push_back(0);
        bananas.push_back(0);
        bananas.push_back(0);
        bananas.push_back(0);
        bananas.push_back(0);
        assert_eq!((((((9 * 19) + 9) * 19) + 9) * 19) + 9, Day22::calculate_bucket(&bananas));
    }

    #[test]
    fn test_part2_small_sample() {
        const SMALL_SAMPLE: &str =
"1
2
3
2024";
        let day = Day22::new(SMALL_SAMPLE.lines());
        assert_eq!(23, day.part2());
    }
}
