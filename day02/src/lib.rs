use std::iter::Iterator;

pub struct Day02 {
    reports: Vec::<Vec::<i32>>,
}

impl Day02 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut reports = Vec::<Vec::<i32>>::new();
        for line in lines {
            let mut report = Vec::<i32>::new();
            for value in line.split(' ').map(&str::parse::<i32>) {
                match value {
                    Ok(value) => { report.push(value); },
                    _ => { panic!("Invalid parsed result for {:?} in {}", value, line); }
                }
            }
            reports.push(report);
        }
        Day02{ reports }
    }

    fn determine_if_increment(report: &Vec<i32>) -> bool {
        let mut count_incr = 0;
        let mut count_decr = 0;

        for (index, level) in report.iter().enumerate() {
            if index > 0 {
                if report[index-1] < *level {
                    count_incr += 1;
                }
                else {
                    count_decr += 1;
                }
            }
        }
        count_incr > count_decr
    }

    fn evaluate_safe(report: &Vec::<i32>, f: &dyn Fn(i32, i32) -> bool) -> bool {
        for (index, _) in report.iter().enumerate() {
            if index > 0 {
                if !f(report[index-1], report[index]) { return false; }
            }
        }
        return true;
    }

    fn test_incr_safe(first: i32, second: i32) -> bool {
        second > first && second - first <= 3
    }
    fn test_decr_safe(first: i32, second: i32) -> bool {
        first > second && first - second <= 3
    }

    fn evaluate(report: &Vec::<i32>) -> bool {
        if report[0] > report[1] {
            Self::evaluate_safe(report, &Self::test_decr_safe)
        }
        else {
            Self::evaluate_safe(report, &Self::test_incr_safe)
        }
    }

    pub fn part1(&self) -> u32 {
        let mut count_of_safe_reports = 0;
        for report in &self.reports {
            if Self::evaluate(&report) {
                count_of_safe_reports += 1;
            }
        }
        count_of_safe_reports
    }

    fn evaluate_safe_dampened(report: &Vec::<i32>, f: &dyn Fn(i32, i32) -> bool) -> bool {
        for (index, _) in report.iter().enumerate() {
            if index > 0 {
                if !f(report[index-1], report[index]) {

                    let mut removed_index_vec: Vec::<i32> = Vec::new();
                    removed_index_vec.extend_from_slice(&report[0..index]);
                    if report.len() > index+1 {
                        removed_index_vec.extend_from_slice(&report[index+1..]);
                    }
                    let mut past_removed_index_vec: Vec::<i32> = Vec::new();
                    past_removed_index_vec.extend_from_slice(&report[0..index-1]);
                    past_removed_index_vec.extend_from_slice(&report[index..]);
                    return Self::evaluate_safe(&removed_index_vec, f) || Self::evaluate_safe(&past_removed_index_vec, f);
                }
            }
        }
        return true;
    }

    fn safe_dampened(report: &Vec::<i32>) -> bool {
        if Self::determine_if_increment(report) {
            Self::evaluate_safe_dampened(report, &Self::test_incr_safe)
        }
        else {
            Self::evaluate_safe_dampened(report, &Self::test_decr_safe)
        }
    }

    pub fn part2(&self) -> u32 {
        let mut count_of_safe_reports = 0;
        for report in &self.reports {
            if Self::safe_dampened(&report) {
                count_of_safe_reports += 1;
            }
        }
        count_of_safe_reports
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str =
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_sample_generates_2() {
        let day02 = Day02::new(SAMPLE_INPUT.lines());
        assert_eq!(2, day02.part1());
    }

    #[test]
    fn part2_sample_generates_2() {
        let day02 = Day02::new(SAMPLE_INPUT.lines());
        assert_eq!(4, day02.part2());
    }
}