use std::collections::BinaryHeap;
use std::collections::HashMap;

use std::cmp::Reverse;

#[derive(Debug,Copy,Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub struct Day16 {
    maze: Vec<Vec<char>>,
    start: ((usize, usize), Direction),
    end: (usize, usize),
}

impl Day16 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut map: Vec::<Vec<char>> = Vec::new();
        let mut start_pos: Option<(usize, usize)> = None;
        let mut end_pos: Option<(usize, usize)> = None;

        for (yindex, line) in lines.enumerate() {
            let row: Vec::<char> = line.chars().collect();
            for (xindex, pos) in (&row).into_iter().enumerate() {
                if *pos == 'S' {
                    assert_eq!(None, start_pos);
                    start_pos = Some((yindex, xindex));
                }
                if *pos == 'E' {
                    assert_eq!(None, end_pos);
                    end_pos = Some((yindex, xindex));
                }
            }
            map.push(row);
        }
        Day16{maze: map, start: (start_pos.unwrap(), Direction::East), end: end_pos.unwrap()}
    }

    fn get_north(&self, pos: &(usize, usize)) -> Option<((usize, usize), Direction)> {
        if pos.0 > 0 && self.maze[pos.0-1][pos.1] != '#' {
            Some(((pos.0 - 1, pos.1), Direction::North))
        } else {
            None
        }
    }
    fn get_west(&self, pos: &(usize, usize)) -> Option<((usize, usize), Direction)> {
        if pos.1 > 0 && self.maze[pos.0][pos.1 - 1] != '#' {
            Some(((pos.0, pos.1-1), Direction::West))
        } else {
            None
        }
    }
    fn get_east(&self, pos: &(usize, usize)) -> Option<((usize, usize), Direction)> {
        if pos.1 + 1 < self.maze[pos.0].len() && self.maze[pos.0][pos.1+1] != '#' {
            Some(((pos.0, pos.1+1), Direction::East))
        } else {
            None
        }
    }
    fn get_south(&self, pos: &(usize, usize)) -> Option<((usize, usize), Direction)> {
        if pos.0 + 1 < self.maze.len() && self.maze[pos.0+1][pos.1] != '#'  {
            Some(((pos.0+1, pos.1), Direction::South))
        } else {
            None
        }
    }
    pub fn get_right(&self, pos: &((usize, usize), Direction)) -> Option<((usize, usize), Direction)> {
        match pos.1 {
            Direction::North => self.get_east(&pos.0),
            Direction::East  => self.get_south(&pos.0),
            Direction::South => self.get_west(&pos.0),
            Direction::West  => self.get_north(&pos.0),
        }
    }

    pub fn get_left(&self, pos: &((usize, usize), Direction)) -> Option<((usize, usize), Direction)> {
        match pos.1 {
            Direction::North => self.get_west(&pos.0),
            Direction::East  => self.get_north(&pos.0),
            Direction::South => self.get_east(&pos.0),
            Direction::West  => self.get_south(&pos.0),
        }
    }
    pub fn get_forward(&self, pos: &((usize, usize), Direction)) -> Option<((usize, usize), Direction)> {
        match pos.1 {
            Direction::North => self.get_north(&pos.0),
            Direction::East => self.get_east(&pos.0),
            Direction::West => self.get_west(&pos.0),
            Direction::South => self.get_south(&pos.0),
        }
    }
}
#[derive(Debug)]
struct MazeFloodStep {
    cost: usize,
    pos: ((usize, usize), Direction)
}
impl Ord for MazeFloodStep {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.cost == other.cost {
            std::cmp::Ordering::Equal
        } else if self.cost < other.cost {
            std::cmp::Ordering::Greater
        } else {
            assert!(self.cost > other.cost);
            std::cmp::Ordering::Less
        }
    }
}
impl PartialOrd for MazeFloodStep {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for MazeFloodStep {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl Eq for MazeFloodStep {

}

fn get_right_maze_step(day: &Day16, prev_step: &MazeFloodStep) -> Option<MazeFloodStep> {
    if let Some(pos) = day.get_right(&prev_step.pos) {
        Some(MazeFloodStep{pos, cost: prev_step.cost + 1001})
    } else {
        None
    }
}

fn get_left_maze_step(day: &Day16, prev_step: &MazeFloodStep) -> Option<MazeFloodStep> {
    if let Some(pos) = day.get_left(&prev_step.pos) {
        Some(MazeFloodStep{pos, cost: prev_step.cost + 1001})
    } else {
        None
    }
}

fn get_forward_maze_step(day: &Day16, prev_step: &MazeFloodStep) -> Option<MazeFloodStep> {
    if let Some(pos) = day.get_forward(&prev_step.pos) {
        Some(MazeFloodStep{pos, cost: prev_step.cost + 1})
    } else {
        None
    }
}


fn load_next_steps(flood_steps: &mut BinaryHeap<MazeFloodStep>, day: &Day16, maze_flood_step: &MazeFloodStep) {
    if let Some(flood_step) = get_forward_maze_step(day, maze_flood_step) {
        //println!("From: {:?}, Next Forward: {:?}", maze_flood_step, flood_step);
        flood_steps.push(flood_step);
    }
    if let Some(flood_step) = get_left_maze_step(day, maze_flood_step) {
        //println!("From: {:?}, Next Left: {:?}", maze_flood_step, flood_step);
        flood_steps.push(flood_step);
    }
    if let Some(flood_step) = get_right_maze_step(day, maze_flood_step) {
        //println!("From: {:?}, Next Forward: {:?}", maze_flood_step, flood_step);
        flood_steps.push(flood_step);
    }
}

pub fn part1(day: &Day16) -> usize {
    let mut visited_loc_costs = HashMap::<(usize, usize), usize>::new();
    let mut flood_steps = BinaryHeap::<MazeFloodStep>::new();
    flood_steps.push(MazeFloodStep{cost: 0, pos: (&day).start});

    while flood_steps.len() > 0 {
        let maze_flood_step: MazeFloodStep = flood_steps.pop().unwrap();
        if let None = visited_loc_costs.get(&maze_flood_step.pos.0) {
            visited_loc_costs.insert(maze_flood_step.pos.0, maze_flood_step.cost);
            if maze_flood_step.pos.0 == day.end {
                //println!("visited_loc_costs: {:?}", visited_loc_costs);
                return maze_flood_step.cost;
            }
            load_next_steps(&mut flood_steps, day, &maze_flood_step);
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_EXAMPLE: &str =
"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const SECOND_EXAMPLE: &str =
"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn first_example_part1_best_path_cost_7036() {
        let day = Day16::new(FIRST_EXAMPLE.lines());
        assert_eq!(7036, part1(&day));
    }

    #[test]
    fn second_example_part1_best_path_cost_11048() {
        let day = Day16::new(SECOND_EXAMPLE.lines());
        assert_eq!(11048, part1(&day));
    }
}
