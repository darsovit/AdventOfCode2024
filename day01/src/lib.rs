
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub struct Day01 {
    left: BinaryHeap<i32>,
    right: BinaryHeap<i32>
}

fn split_line_ints(line: &str) -> (i32, i32) {
    let mut parts = line.split_whitespace().map(|s| s.parse::<i32>());
    match (parts.next(), parts.next()) {
        (Some(Ok(a)), Some(Ok(b))) => {
            (a, b)
        }
        _ => { panic!("{} doesn't split to two integers", line); }
    }
}

impl Day01 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut left = BinaryHeap::new();
        let mut right = BinaryHeap::new();

        for line in lines {
            let (a, b) = split_line_ints(line);
            left.push(a);
            right.push(b);
            assert_eq!(left.len(), right.len());
        }
        Day01{left, right}
    }

    pub fn run(&mut self) -> i32 {
        let mut sum_of_distances = 0;
        loop {
            match( self.left.pop(), self.right.pop() ) {
                (Some(a), Some(b)) => {
                    let distance = if a > b { a-b } else { b-a };
                    sum_of_distances += distance;
                },
                _ => { panic!("Left and right are mismatched in size"); }
            }
            if self.left.len() == 0 && self.right.len() == 0 { break; }
        }
        sum_of_distances
    }
}

pub struct Day01p2 {
    left: HashMap<i32, i32>,
    right: HashMap<i32, i32>,
}

impl Day01p2 {
    fn update_count(map: &mut HashMap<i32, i32>, value: i32) {
        let stat = map.entry(value).or_insert(0);
        *stat += 1;
    }
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut left = HashMap::<i32, i32>::new();
        let mut right = HashMap::<i32, i32>::new();
        for line in lines {
            let (a, b) = split_line_ints(line);
            Self::update_count(&mut left, a);
            Self::update_count(&mut right, b);
        }
        Day01p2{left, right}
    }

    pub fn run(&self) -> i32 {
        let mut similarity_score = 0;
        for (key, value) in &self.left {
            if let Some(a) = self.right.get(&key) {
                similarity_score += key * value * a;
            }
        }
        similarity_score
    }
}

#[cfg(test)]
mod tests {
    use crate::Day01;
    use crate::Day01p2;
    const SAMPLE: &str =
"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn day01_sample_1() {
        let mut day01 = Day01::new(SAMPLE.lines());
        assert_eq!(11, day01.run());
    }

    #[test]
    fn day01p2_sample_1() {
        let mut day01p2 = Day01p2::new(SAMPLE.lines());
        assert_eq!(31, day01p2.run());
    }
}

