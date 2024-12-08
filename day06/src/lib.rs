use std::collections::HashSet;

pub struct Day06<'a> {
    map: Vec<&'a str>,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl<'a> Day06<'a> {
    pub fn new(lines: std::str::Lines<'a>) -> Self {
        Day06{map: lines.collect()}
    }

    fn find_guard_start_position(map: &Vec<&str>) -> Option<(i32, i32, Direction)> {
        for (yindex, line) in map.into_iter().enumerate() {
            let guard_pos = line.find("^");
            if let Some(xindex) = guard_pos { return Some((yindex as i32, xindex as i32, Direction::North)); }
        }
        None
    }
    fn direction_to_increment(dir: Direction) -> (i32, i32) {
        match dir {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    fn calc_next_pos_in_direction(&self, guard_pos: &(i32, i32, Direction)) -> Option<((i32, i32, Direction), &str)> {
        let dir_increment = Self::direction_to_increment(guard_pos.2);
        let new_position = (guard_pos.0 + dir_increment.0, guard_pos.1 + dir_increment.1, guard_pos.2);
        if new_position.0 < 0 || new_position.0 as usize >= self.map.len() || new_position.1 < 0 || new_position.1 as usize >= self.map[0].len() {
            None
        }
        else {
            let slicer = new_position.1 as usize;
            let yindex: usize = new_position.0 as usize;
            Some((new_position, &self.map[yindex][slicer..slicer+1]))
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
    fn turn_guard(guard_pos: (i32, i32, Direction)) -> (i32, i32, Direction) {
        (guard_pos.0, guard_pos.1, Self::next_direction(guard_pos.2))
    }
    fn find_next_guard_position(&self, guard_pos: (i32, i32, Direction)) -> Option<(i32, i32, Direction)> {
        let mut cur_guard_pos = guard_pos;
        loop {
            let next_guard_pos_and_item = self.calc_next_pos_in_direction(&cur_guard_pos);
            if let Some(guard_pos_and_item) = next_guard_pos_and_item {
                if guard_pos_and_item.1 == "#" {
                    cur_guard_pos = Self::turn_guard(cur_guard_pos);
                }
                else {
                    return Some(guard_pos_and_item.0);
                }
            } else {
                return None;
            }
        }
    }

    pub fn part1(&self) -> usize {
        let mut guard_views: HashSet<(i32, i32)> = HashSet::new();
        let mut guard_pos = Self::find_guard_start_position(&self.map).unwrap();
        guard_views.insert((guard_pos.0, guard_pos.1));

        loop {
            let possible_guard_pos = self.find_next_guard_position(guard_pos);
            if let Some(a_guard_pos) = possible_guard_pos {
                guard_views.insert((a_guard_pos.0, a_guard_pos.1));
                guard_pos = a_guard_pos;
            } else {
                break;
            }
        }
        guard_views.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_results_in_41() {
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
        let day = Day06::new(SAMPLE_INPUT.lines());
        assert_eq!(41, day.part1());
    }
}