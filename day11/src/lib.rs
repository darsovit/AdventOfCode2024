use std::collections::HashMap;

pub struct Day11 {
    stones: Vec<u64>
}

impl Day11 {
    pub fn new(line: &str) -> Self {
        let stones = line.split(" ").map(|v| v.parse::<u64>().unwrap()).collect();
        Day11{stones}
    }

    pub fn blink(stones: &Vec<u64>) -> Vec<u64> {
        let mut answer = Vec::new();
        for stone in stones {
            if 0 == *stone {
                answer.push(1);
            } else {
                let decimal_number = format!("{}", *stone);
                if decimal_number.len() % 2 == 0 {
                    let half_number = decimal_number.len() / 2;
                    answer.push(decimal_number[0..half_number].parse::<u64>().unwrap());
                    answer.push(decimal_number[half_number..].parse::<u64>().unwrap());
                } else {
                    answer.push(*stone * 2024);
                }
            }
        }
        answer
    }

    pub fn blink_n_times(condition: &Vec<u64>, n: usize) -> Vec<u64> {
        let mut answer = condition.clone();
        for _ in 0..n {
            answer = Self::blink(&answer);
        }
        answer
    }

    pub fn part1(&self) -> usize {
        Self::blink_n_times(&self.stones, 25).len()
    }

    fn count_num_of_stone_values(stones: &Vec<u64>) -> HashMap<u64, u64> {
        let mut stone_num_to_count: HashMap<u64, u64> = HashMap::new();
        for stone in stones {
            stone_num_to_count.entry(*stone).and_modify(|v| *v += 1).or_insert(1);
        }
        stone_num_to_count
    }
    
    pub fn part2(&self) -> u64 {
        // we need to repeat 75 times
        // we don't care about the actual order of the stones
        let answer_to_25 = Self::blink_n_times(&self.stones, 25);
        let stone_num_to_count_answer_to_25 = Self::count_num_of_stone_values(&answer_to_25);

        let mut cache25: HashMap<u64, HashMap<u64, u64>> = HashMap::new();
        let mut answer_to_50: HashMap<u64, u64> = HashMap::new();

        for (stone_num, count) in stone_num_to_count_answer_to_25 {
            let result = cache25.entry(stone_num).or_insert(Self::count_num_of_stone_values(&Self::blink_n_times(&vec![stone_num], 25)));
            for (result_stone, result_count) in result {
                let val = answer_to_50.entry(*result_stone).or_insert(0);
                *val += *result_count * count;
            }
        }

        let mut answer_to_75: HashMap<u64, u64> = HashMap::new();
        for (stone_num, count) in answer_to_50 {
            let result = cache25.entry(stone_num).or_insert(Self::count_num_of_stone_values(&Self::blink_n_times(&vec![stone_num], 25)));
            for (result_stone, result_count) in result {
                let val = answer_to_75.entry(*result_stone).or_insert(0);
                *val += *result_count * count;
            }
        }

        let mut num_stones_in_75: u64 = 0;
        for (stone_num, count) in answer_to_75 {
            num_stones_in_75 += count;
        }
        num_stones_in_75
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_blink_example() {
        assert_eq!(vec![1, 2024, 1, 0, 9, 9, 2021976], Day11::blink(&vec![0, 1, 10, 99, 999]))
    }

    
    #[test]
    fn larger_test_one_blink() {
        let larger_example: Vec<u64> = vec![125, 17];
        assert_eq!(vec![253000, 1, 7], Day11::blink(&larger_example));
    }

    #[test]
    fn larger_test_two_blinks() {
        let larger_example: Vec<u64> = vec![125, 17];
        assert_eq!(vec![253, 0, 2024, 14168], Day11::blink_n_times(&larger_example, 2));
    }

    #[test]
    fn larger_test_three_blinks() {
        let larger_example: Vec<u64> = vec![125, 17];
        assert_eq!(vec![512072, 1, 20, 24, 28676032], Day11::blink_n_times(&larger_example, 3));
    }

    #[test]
    fn larger_test_four_blinks() {
        let larger_example: Vec<u64> = vec![125, 17];
        assert_eq!(vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032], Day11::blink_n_times(&larger_example, 4));
    }

    #[test]
    fn larger_test_five_blinks() {
        let larger_example: Vec<u64> = vec![125, 17];
        assert_eq!(vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32], Day11::blink_n_times(&larger_example, 5));
    }

    #[test]
    fn larger_test_six_blinks() {
        let larger_example: Vec<u64> = vec![125, 17];
        assert_eq!(vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2], Day11::blink_n_times(&larger_example, 6));
    }

    #[test]
    fn test_part1_with_larger_example_has_55312_stones() {
        let day = Day11::new("125 17");
        assert_eq!(55312, day.part1());
    }
}