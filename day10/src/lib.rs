use std::collections::HashSet;

pub struct Day10 {
    topography: Vec<Vec<i8>>,
    trailheads: Vec<(usize, usize)>,
}

fn char_to_num(a_char: char) -> Option<i8> {
    match a_char {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None
    }
}

impl Day10 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut topography = Vec::<Vec<i8>>::new();
        let mut trailheads = Vec::<(usize, usize)>::new();
        for (yindex, line) in lines.enumerate() {
            let mut row = Vec::new();
            for (xindex, num) in line.chars().enumerate() {
                if let Some(val) = char_to_num(num) {
                    if val == 0 { trailheads.push((yindex, xindex)); }
                    row.push(val);
                }
            }
            topography.push(row);
        }
        Day10{topography, trailheads}
    }

    fn validate_neighbor(&self, pos: (usize, usize), offset: (i64, i64)) -> Option<(usize, usize)> {
        let possible_pos = (pos.0 as i64 + offset.0, pos.1 as i64 + offset.1);
        if possible_pos.0 >= 0 && self.topography.len() > possible_pos.0 as usize && possible_pos.1 >= 0 && self.topography[0].len() > possible_pos.1 as usize {
            Some((possible_pos.0 as usize, possible_pos.1 as usize))
        }
        else {
            None
        }
    }

    fn neighbor_list(&self, pos: (usize, usize)) -> Vec<Option<(usize, usize)>> {
        let mut neighbors = Vec::new();
        neighbors.push(self.validate_neighbor(pos, (-1, 0)));
        neighbors.push(self.validate_neighbor(pos, (1, 0)));
        neighbors.push(self.validate_neighbor(pos, (0, -1)));
        neighbors.push(self.validate_neighbor(pos, (0, 1)));
        neighbors
    }

    fn walk_gradual_climb_to_peaks(&self, pos: (usize, usize), next_target: i8, peaks_found: &mut HashSet<(usize, usize)>) {
        if next_target == 10 {
            peaks_found.insert(pos);
        }
        let mut score = 0;
        for neighbor in self.neighbor_list(pos) {
            if let Some((yindex, xindex)) = neighbor {
                if self.topography[yindex][xindex] == next_target {
                    self.walk_gradual_climb_to_peaks((yindex, xindex), next_target+1, peaks_found);
                }
            }
        }
    }

    fn score_trailhead(&self, trailhead: (usize, usize)) -> usize {
        let mut peaks = HashSet::<(usize, usize)>::new();
        self.walk_gradual_climb_to_peaks(trailhead, 1, &mut peaks);
        peaks.len()
    }

    pub fn part1(&self) -> usize {
        let mut sum_of_trailhead_scores = 0;
        for trailhead in &self.trailheads {
            sum_of_trailhead_scores += self.score_trailhead(*trailhead);
        }
        sum_of_trailhead_scores
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample1_is_1() {
        const SAMPLE_LINES: &str =
"0123
1234
8765
9876";
        let day = Day10::new(SAMPLE_LINES.lines());
        assert_eq!(1, day.part1());
    }

    #[test]
    fn part1_larger_sample_is_36() {
        const SAMPLE_LINES: &str =
"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let day = Day10::new(SAMPLE_LINES.lines());
        assert_eq!(36, day.part1());
    }
}