use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Day18 {
    falling_data: Vec<(usize, usize)>,
    height: usize,
    width: usize,
}

impl Day18 {
    pub fn new(lines: std::str::Lines<'_>, height: usize, width: usize) -> Self {
        let mut falling_data = Vec::new();
        for line in lines {
            let pos: Vec<usize> = line.split(",").map(|v| v.parse::<usize>().unwrap()).collect();
            falling_data.push((pos[0], pos[1]));
        }
        Day18{falling_data, height: height + 1, width: width + 1}
    }

    pub fn get_neighbors(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::<(usize, usize)>::new();
        if pos.0 > 0 { neighbors.push((pos.0 - 1, pos.1)); }
        if pos.1 > 0 { neighbors.push((pos.0, pos.1 - 1)); }
        if pos.0 + 1 < self.height { neighbors.push((pos.0 + 1, pos.1)); }
        if pos.1 + 1 < self.width  { neighbors.push((pos.0, pos.1 + 1)); }
        neighbors
    }

    pub fn find_shortest_path_steps(&self, start: (usize, usize), end: (usize, usize), invalid: HashSet<(usize, usize)>) -> usize {
        let mut visited_sectors = HashMap::<(usize, usize), usize>::new();
        let mut steps_to_go = VecDeque::<((usize, usize), usize)>::new();
        steps_to_go.push_back((start, 0));
        while steps_to_go.len() > 0 {
            let (pos, cost) = steps_to_go.pop_front().unwrap();
            if let None = visited_sectors.get(&pos) {
                visited_sectors.insert(pos, cost);
                if pos == end { break; }
                for neighbor in self.get_neighbors(&pos) {
                    if let (None, None) = (invalid.get(&neighbor), visited_sectors.get(&neighbor)) {
                        steps_to_go.push_back((neighbor, cost + 1));
                    }
                }
            }
        }
        *visited_sectors.get(&end).unwrap()
    }

    pub fn part1(&self, time: usize) -> usize {
        println!("{:?}", self);
        assert!(time < self.falling_data.len());

        let mut corrupted_sectors = HashSet::<(usize, usize)>::new();
        for i in 0..time {
            corrupted_sectors.insert(self.falling_data[i]);
        }

        self.find_shortest_path_steps((0, 0), (self.height - 1, self.width - 1), corrupted_sectors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str =
"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_sample_input_is_22_at_time_12() {
        let day = Day18::new(SAMPLE_INPUT.lines(), 6, 6);
        assert_eq!(22, day.part1(12));
    }

}
