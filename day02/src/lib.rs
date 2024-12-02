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

    fn evaluate_incr_safe(report: &Vec::<i32>) -> bool {
        let mut prior_value = report[0]-1;
        for level in report {
            if *level <= prior_value { return false; };
            if *level - prior_value > 3 { return false; };
            prior_value = *level;
        }
        return true;
    }
    fn evaluate_decr_safe(report: &Vec::<i32>) -> bool {
        let mut prior_value = report[0]+1;
        for level in report {
            if *level >= prior_value { return false; };
            if prior_value - *level > 3 { return false; };
            prior_value = *level;
        }
        return true;
    }
    fn evaluate_safe(report: &Vec::<i32>) -> bool {
        if report[0] > report[1] {
            Self::evaluate_decr_safe(report)
        }
        else {
            Self::evaluate_incr_safe(report)
        }

    }
    pub fn part1(&self) -> u32 {
        let mut count_of_safe_reports = 0;
        for report in &self.reports {
            if Self::evaluate_safe(&report) {
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
}