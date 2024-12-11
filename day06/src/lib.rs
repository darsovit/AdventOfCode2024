use std::collections::HashSet;
use std::collections::HashMap;

pub struct Day06 {
    obstacles: HashSet<(usize, usize)>,
    max_y: usize,
    max_x: usize,
    guard_start_pos_and_dir: ((usize, usize), Direction),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Day06 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
        let mut guard_pos_and_dir: Option<((usize, usize), Direction)> = None;
        let map: Vec<&str> = lines.collect();
        let max_y = map.len();
        let max_x = map[0].len();
        for (y_index, line) in map.into_iter().enumerate() {
            for (x_index, a_char) in line.chars().enumerate() {
                match a_char {
                    '#' => { obstacles.insert((y_index, x_index)); },
                    '^' => { if let None = guard_pos_and_dir { guard_pos_and_dir = Some(((y_index, x_index), Direction::North )); } },
                    _ => { /* no-op */ }
                }
            }
        }
        Day06{obstacles, max_y, max_x, guard_start_pos_and_dir: guard_pos_and_dir.unwrap()}
    }

    fn direction_to_increment(dir: Direction) -> (i32, i32) {
        match dir {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    fn calc_next_pos_in_direction(&self, guard_pos: &((usize, usize), Direction)) -> Option<((usize, usize), Direction)> {
        let dir_increment = Self::direction_to_increment(guard_pos.1);

        let new_position = (guard_pos.0.0 as i32 + dir_increment.0, guard_pos.0.1 as i32 + dir_increment.1);
        if new_position.0 < 0 || new_position.0 as usize >= self.max_y || new_position.1 < 0 || new_position.1 as usize >= self.max_x {
            None
        }
        else {
            let next_pos = ((new_position.0 as usize, new_position.1 as usize), guard_pos.1);
            Some(next_pos)
        }
    }

    fn next_direction(dir: Direction) -> Direction {
        match dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    fn turn_guard(guard_pos: ((usize, usize), Direction)) -> ((usize, usize), Direction) {
        (guard_pos.0, Self::next_direction(guard_pos.1))
    }
    fn find_next_guard_position(&self, guard_pos: ((usize, usize), Direction)) -> Option<((usize, usize), Direction)> {
        let mut cur_guard_pos = guard_pos;
        let mut loop_count = 0;
        loop {
            let next_guard_pos = self.calc_next_pos_in_direction(&cur_guard_pos);
            if let Some(guard_pos) = next_guard_pos {
                if let Some(_) = self.obstacles.get(&guard_pos.0) {
                    cur_guard_pos = Self::turn_guard(cur_guard_pos);
                    loop_count += 1;
                }
                else {
                    if loop_count == 2 { println!("Had a second turn"); }
                    return Some(guard_pos);
                }
            } else {
                return None;
            }
        }
    }

    pub fn part1(&self) -> usize {
        let mut guard_views: HashSet<(i32, i32)> = HashSet::new();
        //let mut guard_pos = (self.guard_start_pos_and_dir.0.0 as i32, self.guard_start_pos_and_dir.0.1 as i32, self.guard_start_pos_and_dir.1);
        let mut guard_pos = self.guard_start_pos_and_dir;
        guard_views.insert((guard_pos.0.0 as i32, guard_pos.0.1 as i32));

        loop {
            let possible_guard_pos = self.find_next_guard_position(guard_pos);
            if let Some(a_guard_pos) = possible_guard_pos {
                guard_views.insert((a_guard_pos.0.0 as i32, a_guard_pos.0.1 as i32));
                guard_pos = a_guard_pos;
            } else {
                break;
            }
        }
        guard_views.len()
    }

    pub fn part2(&self) -> u32 {
        let mut count_of_possible_loops = 0;

        let mut guard_views: HashMap<(usize, usize), HashSet<Direction>> = HashMap::new();
        let mut guard_pos = self.guard_start_pos_and_dir;
        let mut initial_guard_hash: HashSet<Direction> = HashSet::new();
        initial_guard_hash.insert(guard_pos.1);
        guard_views.insert(guard_pos.0, initial_guard_hash);
        loop {
            let possible_guard_pos = self.find_next_guard_position(guard_pos);
            if let Some(a_guard_pos) = possible_guard_pos {
                if a_guard_pos.1 == guard_pos.1 {
                    let directions = guard_views.get(&a_guard_pos.0);
                    let loop_direction = Self::next_direction(a_guard_pos.1);
                    if let Some(directions) = directions {
                        if directions.contains(&loop_direction) {
                            count_of_possible_loops += 1;
                        }
                    }
                }
                guard_views.entry(a_guard_pos.0).or_insert(HashSet::new());
                guard_views.entry(a_guard_pos.0).and_modify(|e| { e.insert(a_guard_pos.1); });
                guard_pos = a_guard_pos;
            }
            else {
                break;
            }
        }
        count_of_possible_loops
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_sample_results_in_41() {
        let day = Day06::new(SAMPLE_INPUT.lines());
        assert_eq!(41, day.part1());
    }

    #[test]
    fn part2_sample_results_in_6() {
        let day = Day06::new(SAMPLE_INPUT.lines());
        assert_eq!(6, day.part2());
    }
}
