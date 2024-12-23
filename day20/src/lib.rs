use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Pos {
    y: usize,
    x: usize,
}

#[derive(Debug)]
pub struct Day20 {
    maze: Vec<Vec<char>>,
    start: Pos,
    end: Pos,
}

impl Day20 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut map: Vec::<Vec<char>> = Vec::new();
        let mut start_pos: Option<Pos> = None;
        let mut end_pos: Option<Pos> = None;

        for (yindex, line) in lines.enumerate() {
            let row: Vec::<char> = line.chars().collect();
            for (xindex, pos) in (&row).into_iter().enumerate() {
                if *pos == 'S' {
                    assert_eq!(None, start_pos);
                    start_pos = Some(Pos{y:yindex, x:xindex});
                }
                if *pos == 'E' {
                    assert_eq!(None, end_pos);
                    end_pos = Some(Pos{y:yindex, x:xindex});
                }
            }
            map.push(row);
        }
        Day20{maze: map, start: start_pos.unwrap(), end: end_pos.unwrap()}
    }


    fn get_position(&self, pos: &Pos, adjust: (i64, i64)) -> Option<Pos> {
        let new_coords = (pos.y as i64 + adjust.0, pos.x as i64 + adjust.1);
        if 0 > new_coords.0 || 0 > new_coords.1 {
            None
        } else {
            let new_coords = (new_coords.0 as usize, new_coords.1 as usize);
            if new_coords.0 < self.maze.len() && new_coords.1 < self.maze[0].len() {
                Some(Pos{y: new_coords.0, x: new_coords.1})
            }
            else {
                None
            }
        }
    }

    fn is_wall(&self, pos: Pos) -> bool {
        self.maze[pos.y][pos.x] == '#'
    }

    fn is_valid_neighbor(&self, pos: &Pos, adjust: (i64, i64)) -> Option<Pos> {
        if let Some(neighbor) = self.get_position(pos, adjust) {
            if self.is_wall(neighbor) { None } else { Some(neighbor) }
        } else {
            None
        }
    }
    fn find_valid_neighbors(&self, pos: &Pos) -> Vec<Pos> {
        let mut neighbors = Vec::<Pos>::new();
        for adjustment in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let Some(neighbor) = self.is_valid_neighbor(pos, adjustment) { neighbors.push(neighbor); }
        }
        neighbors
    }

    fn find_valid_2space_neighbors(&self, pos: &Pos) -> Vec<Pos> {
        let mut neighbors = Vec::<Pos>::new();
        for adjustment in [(-2, 0), (2, 0), (0, -2), (0, 2)] {
            if let Some(neighbor) = self.is_valid_neighbor(pos, adjustment) { neighbors.push(neighbor); }
        }
        neighbors
    }

    fn walk_the_maze(&self) -> HashMap<Pos, usize> {
        let mut time_to_point = HashMap::<Pos, usize>::new();
        let mut work_to_do = VecDeque::<(Pos, usize)>::new();
        work_to_do.push_back((self.start, 0));
        while work_to_do.len() > 0 {
            let (next_pos, time) = work_to_do.pop_front().unwrap();
            time_to_point.insert(next_pos, time);
            if next_pos != self.end {
                for neighbor in self.find_valid_neighbors(&next_pos) {
                    if let None = time_to_point.get(&neighbor) {
                        work_to_do.push_back((neighbor, time + 1));
                    }
                }
            }
        }
        time_to_point
    }

    fn get_cheats_for_at_least(&self, saves_at_least: usize) -> HashMap<usize, Vec<(Pos, Pos)>> {
        let steps_along_path = self.walk_the_maze();
        let mut cheats_found = HashMap::<usize, Vec<(Pos, Pos)>>::new();
        for (step_along_path, time) in &steps_along_path {
            for neighbor in self.find_valid_2space_neighbors(&step_along_path) {
                if let Some(time2) = steps_along_path.get(&neighbor) {
                    if *time2 > *time && *time2 - *time > 2 {
                        let time_saved = *time2 - *time - 2;
                        if time_saved >= saves_at_least {
                            cheats_found.entry(*time2 - *time - 2).or_insert(Vec::<(Pos, Pos)>::new()).push((*step_along_path, neighbor));
                        }
                    }
                }
            }
        }
        println!("{:?}", cheats_found);
        cheats_found
    }

    pub fn part1(&self, saves_at_least: usize) -> usize {
        let cheat_savings_and_locs = self.get_cheats_for_at_least(saves_at_least);
        let mut count_of_cheats_for_at_least = 0;
        for (k,v) in cheat_savings_and_locs {
            assert!(k >= saves_at_least);
            count_of_cheats_for_at_least += v.len();
        }
        count_of_cheats_for_at_least
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str =
"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn sample_input_has_1_cheat_at_least_64() {
        let day = Day20::new(SAMPLE_INPUT.lines());
        assert_eq!(1, day.part1(64));
    }

    #[test]
    fn sample_input_has_2_cheats_at_least_40() {
        let day = Day20::new(SAMPLE_INPUT.lines());
        assert_eq!(2, day.part1(40));
    }

    #[test]
    fn sample_input_has_3_cheats_at_least_38() {
        let day = Day20::new(SAMPLE_INPUT.lines());
        assert_eq!(3, day.part1(38));
    }

    #[test]
    fn sample_input_has_4_cheats_at_least_36() {
        let day = Day20::new(SAMPLE_INPUT.lines());
        assert_eq!(4, day.part1(36));
    }

    #[test]
    fn sample_input_has_5_cheats_at_least_20() {
        let day = Day20::new(SAMPLE_INPUT.lines());
        assert_eq!(5, day.part1(20));
    }
}