use std::collections::HashSet;

pub struct Day25 {
    locks: Vec<[i8; 5]>,
    keys:  Vec<[i8; 5]>,
}

impl Day25 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut locks = Vec::<[i8; 5]>::new();
        let mut keys  = Vec::<[i8; 5]>::new();

        let mut line_iter = lines.into_iter();
        let mut done = false;
        while !done {
            let mut array: [i8; 5] = [0; 5];
            if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g)) = (line_iter.next(), line_iter.next(), line_iter.next(), line_iter.next(), line_iter.next(), line_iter.next(), line_iter.next()) {
                if a == "#####" {
                    for line in [b, c, d, e, f] {
                        for (index, val) in line.chars().into_iter().enumerate() {
                            if val == '#' { array[index] += 1; }
                        }
                    }
                    locks.push(array);
                }
                else {
                    assert_eq!(g, "#####");
                    for line in [f, e, d, c, b] {
                        for (index, val) in line.chars().into_iter().enumerate() {
                            if val == '#' { array[index] += 1; }
                        }
                    }
                    keys.push(array);
                }
            }
            if let None = line_iter.next() { done = true; }
        }
        Day25{locks, keys}
    }

    pub fn part1(&self) -> usize {
        let mut fitting_key_lock_pairs = HashSet::<(usize, usize)>::new();

        for (lock_index, lock) in (&self.locks).into_iter().enumerate() {
            for (key_index, key) in (&self.keys).into_iter().enumerate() {
                let mut fits = true;
                for i in 0..5 {
                    if lock[i] + key[i] > 5 { fits = false; }
                }
                if fits { fitting_key_lock_pairs.insert((key_index, lock_index)); }
            }
        }
        fitting_key_lock_pairs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_LINES: &str =
"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn sample_input_part1_counts_3() {
        let day = Day25::new(SAMPLE_LINES.lines());
        assert_eq!(3, day.part1());
    }
}
