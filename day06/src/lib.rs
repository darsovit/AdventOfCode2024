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



    fn is_forward_blocked(&self, guard_pos: &((usize, usize), Direction)) -> Option<bool> {
        match self.calc_next_pos_in_direction(guard_pos) {
            None => None,
            Some(forward_position) => {
                match self.obstacles.get(&forward_position.0) {
                    None => Some(false),
                    Some(_) => Some(true)
                }
            }
        }
    }


}

pub fn part1(day: Day06) -> usize {
    let mut guard_views: HashSet<(i32, i32)> = HashSet::new();
    //let mut guard_pos = (self.guard_start_pos_and_dir.0.0 as i32, self.guard_start_pos_and_dir.0.1 as i32, self.guard_start_pos_and_dir.1);
    let mut guard_pos = day.guard_start_pos_and_dir;
    guard_views.insert((guard_pos.0.0 as i32, guard_pos.0.1 as i32));

    loop {
        match day.is_forward_blocked(&guard_pos) {
            None => { break; },
            Some(true) => { guard_pos = (guard_pos.0, Day06::next_direction(guard_pos.1)); }
            Some(false) => {
                if let Some(a_guard_pos) = day.calc_next_pos_in_direction(&guard_pos) {
                    guard_views.insert((a_guard_pos.0.0 as i32, a_guard_pos.0.1 as i32));
                    guard_pos = a_guard_pos;
                }
            }
        }
    }
    guard_views.len()
}

struct Day06p2<'a> {
    day: &'a Day06,
    guard_past_state: HashMap<(usize, usize), HashSet<Direction>>,
    added_obstacle: (usize, usize),
}

impl<'a> Day06p2<'a> {
    pub fn new(day06: &'a Day06, guard_past_state: &HashMap<(usize, usize), HashSet<Direction>>, added_obstacle: (usize, usize)) -> Self {
        Day06p2{day: day06, guard_past_state: guard_past_state.clone(), added_obstacle}
    }

    fn calc_next_pos_in_direction(&self, guard_pos: &((usize, usize), Direction)) -> Option<((usize, usize), Direction)> {
        self.day.calc_next_pos_in_direction(guard_pos)
    }

    fn check_for_guard_visited_already(guard_views: &HashMap<(usize, usize), HashSet<Direction>>, guard_pos: ((usize, usize), Direction)) -> bool {
        match guard_views.get(&guard_pos.0) {
            None => false,
            Some(directions) => {
                match directions.get(&guard_pos.1) {
                    None => false,
                    Some(_) => true
                }
            }
        }
    }


    fn check_for_direct_loop_if_obstacle_added(guard_views: &HashMap<(usize, usize), HashSet<Direction>>, guard_pos: ((usize, usize), Direction)) -> bool {
        Self::check_for_guard_visited_already(guard_views, (guard_pos.0, Day06::next_direction(guard_pos.1)))
    }

    fn is_forward_blocked(&self, guard_pos: &((usize, usize), Direction)) -> Option<bool> {
        match self.day.is_forward_blocked(&guard_pos) {
            None => None,
            Some(true) => Some(true),
            Some(false) => Some(guard_pos.0 == self.added_obstacle)
        }
    }

    fn check_for_deep_loop(&self, guard_pos: &((usize, usize), Direction)) -> bool {
        let mut guard_views = self.guard_past_state.clone();
        let mut guard_pos = *guard_pos;
        loop {
            match self.is_forward_blocked(&guard_pos) {
                None => {
                    // End condition and no loop, because the guard has walked off the map
                    return false;
                },
                Some(true) => {
                    // Guard has to turn
                    guard_pos = Day06::turn_guard(guard_pos);
                    add_to_guard_visited(&mut guard_views, &guard_pos);
                },
                Some(false) => {
                    // Check if we take a step forward, if it has already been visited, if so, loop found, otherwise continue
                    let forward_step = self.day.calc_next_pos_in_direction(&guard_pos).unwrap();
                    if Self::check_for_guard_visited_already(&guard_views, forward_step) {
                        return true;
                    }
                    add_to_guard_visited(&mut guard_views, &forward_step);
                    guard_pos = forward_step;
                }
            }
        }
    }

}

fn is_forward_available_for_obstacle_positioning(day: &Day06, guard_views: &HashMap<(usize, usize), HashSet<Direction>>, guard_pos: &((usize, usize), Direction)) -> bool {
    let forward_pos = day.calc_next_pos_in_direction(guard_pos);
    match forward_pos {
        None => false,
        Some(position) => {
            let has_guard_visited = if let Some(_) = guard_views.get(&position.0) { true } else { false };
            let has_obstacle = if let Some(_) = day.obstacles.get(&position.0) { true } else { false };
            !has_guard_visited && !has_obstacle
        }
    }
}

fn add_to_guard_visited(guard_views: &mut HashMap<(usize, usize), HashSet<Direction>>, guard_pos: &((usize, usize), Direction)) {
    let create_new_direction_set = || -> HashSet<Direction> {
        let mut initial_hash = HashSet::<Direction>::new();
        initial_hash.insert(guard_pos.1);
        initial_hash
    };

    guard_views.entry(guard_pos.0).and_modify(|values| { values.insert(guard_pos.1); }).or_insert_with(create_new_direction_set);
}


pub fn part2(day06: Day06) -> u32 {
    let mut count_of_possible_loops = 0;
    let mut guard_views: HashMap<(usize, usize), HashSet<Direction>> = HashMap::new();
    let mut guard_pos = day06.guard_start_pos_and_dir;
    add_to_guard_visited(&mut guard_views, &guard_pos);

    loop {
        match day06.is_forward_blocked(&guard_pos) {
            None => {
                println!("Next from {:?} is off", guard_pos); 
                break;
            }
            Some(true) => {
                guard_pos = Day06::turn_guard(guard_pos);
                add_to_guard_visited(&mut guard_views, &guard_pos);
                println!("Forward was blocked, turned to {:?}", guard_pos);
            },
            Some(false) => {
                let forward_step = day06.calc_next_pos_in_direction(&guard_pos).unwrap();
                if is_forward_available_for_obstacle_positioning(&day06, &guard_views, &guard_pos) {
                    println!("Testing for additional block introucing loop at {:?}", forward_step);
                    let day = Day06p2::new(&day06, &guard_views, forward_step.0);
                    if day.check_for_deep_loop(&guard_pos) {
                        count_of_possible_loops += 1;
                    }
                }
                add_to_guard_visited(&mut guard_views, &forward_step);
                guard_pos = forward_step;
            }
        }
    }
    count_of_possible_loops
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
        assert_eq!(41, part1(day));
    }

    #[test]
    fn part2_sample_results_in_6() {
        let day = Day06::new(SAMPLE_INPUT.lines());
        assert_eq!(6, part2(day));
    }
}
