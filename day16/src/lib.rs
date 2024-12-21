use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;

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
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
enum MajorDirection {
    NorthSouth,
    EastWest,
}

fn get_major_direction(dir: Direction) -> MajorDirection {
    match dir {
        Direction::North => MajorDirection::NorthSouth,
        Direction::South => MajorDirection::NorthSouth,
        Direction::East  => MajorDirection::EastWest,
        Direction::West  => MajorDirection::EastWest,
    }
}

fn flood_to_best_paths(day: &Day16) -> (usize, HashMap::<((usize, usize), MajorDirection), (Direction, usize)>) {
    const DEBUG: bool = false;
    let mut visited_loc_costs = HashMap::<((usize, usize), MajorDirection), (Direction, usize)>::new();
    let mut flood_steps = BinaryHeap::<MazeFloodStep>::new();
    let start = day.start;
    let end   = day.end;
    flood_steps.push(MazeFloodStep{cost: 0, pos: start});

    loop {
        let maze_flood_step: MazeFloodStep = flood_steps.pop().unwrap();
        //
        //
        //   consider:
        // #^###^#####^###############^#^###^#v#^###########v#v#v#^###v#^#v#####^#####v#####v#^###.###^#######^#v#v#####^###^###^###########^#####^#v#^#
        // #<<<<<<^>>>>>>>>>>>>>>#^>>>>#^#^>>#v#^#^>>>>>>>>#v#v#v#^#<<v#^#v>>>>#^#<<^#v#^<^#v#^#.....#<<<<<<^#<<v#v>>>>>>>>>>>>#<<<<<<<<<<^#<<^#^#<<v#^#
        // #######^#############v#^#####^#^#v###^#^#####v#v#v#v###^#v###^#####v#^#v#^###^#^#v#^#########v#v#^#############v###v###########^###^#^#####^#
        // #^>>>>>X>>>>>>#<<^>>#v#<<<<^#^#^#v#^>>#^#^>>#v#v#v#v>>#^#v>>#^>>>>#v#<<v#<<^#^#^#v>>>>>>>>>>#v#v#<<<<<<<<<<^#<<v>>#v#<<<<<<^#^#<<^#<<<<<<^#^#
        // #^#v###^#####v###^###^#####^#^#^###^###^#^#v#v###v###v#^###^#^#v###v#######^#^#^###########v###v###########^###v#v#v#####v#^#^###^#######^#^#
        //   -- The X position can be visited two different ways and the cost, but the first one getting there chooses
        //   -- the cost of the positions beyond the X to the top and right when we don't allow the second to 'pass through'
        //
        let visited_key = (maze_flood_step.pos.0, get_major_direction(maze_flood_step.pos.1));
        if let None = visited_loc_costs.get(&visited_key) {
            visited_loc_costs.insert(visited_key, (maze_flood_step.pos.1, maze_flood_step.cost));
            if maze_flood_step.pos.0 == end {
                if DEBUG {
                    println!("Map at end: ");
                    fn print_direction(dir: Direction) {
                        match dir {
                            Direction::North => { print!("^"); },
                            Direction::East  => { print!(">"); },
                            Direction::South => { print!("v"); },
                            Direction::West  => { print!("<"); },
                        }
                    }
                    for (yindex, row) in (&day.maze).into_iter().enumerate() {
                        for (xindex, val) in row.into_iter().enumerate() {
                            match (visited_loc_costs.get(&((yindex, xindex), MajorDirection::NorthSouth)), visited_loc_costs.get(&((yindex, xindex), MajorDirection::EastWest))) {
                                (Some((_dir1, _cost1)), Some((_dir2, _cost2))) => {
                                    print!("X");
                                }
                                (Some((dir, _)), None) => {
                                    print_direction(*dir);
                                }
                                (None, Some((dir, _))) => {
                                    print_direction(*dir);
                                }
                                (None, None) => {
                                    print!("{val}");
                                }
                            }
                        }
                        println!("");
                    }
                    println!("{:?}", flood_steps);
                }
                //println!("visited_loc_costs: {:?}", visited_loc_costs);
                return (maze_flood_step.cost, visited_loc_costs);
            }
            load_next_steps(&mut flood_steps, day, &maze_flood_step);
        }
    }
}

pub fn part1(day: &Day16) -> usize {
    let (best_cost, _) = flood_to_best_paths(day);
    best_cost
}

fn get_backwards_tile(pos: (usize, usize), dir: &Direction) -> (usize, usize) {
    match dir {
        Direction::North => (pos.0 + 1, pos.1),
        Direction::East  => (pos.0, pos.1 - 1),
        Direction::South => (pos.0 - 1, pos.1),
        Direction::West  => (pos.0, pos.1 + 1),
    }
}
fn count_num_tiles_walking_back_best_paths(day: &Day16, best_cost: usize, visited_loc_costs: &HashMap<((usize, usize), MajorDirection), (Direction, usize)>) -> usize {
    let mut sum_of_num_tiles = 0;
    let mut tiles_on_backwards_paths = VecDeque::<((usize, usize), (usize, usize))>::new();
    let mut tiles_on_best_paths = HashSet::<(usize, usize)>::new();
    let mut cur_step_cost = best_cost % 1000;
    tiles_on_backwards_paths.push_back((day.end, (best_cost % 1000, best_cost / 1000)));

    while tiles_on_backwards_paths.len() > 0 {
        let (pos, cost) = tiles_on_backwards_paths.pop_front().unwrap();
        if let None = tiles_on_best_paths.get(&pos) {
            tiles_on_best_paths.insert(pos);
            assert!(cost.0 == cur_step_cost || cost.0 == cur_step_cost - 1);
            cur_step_cost = cost.0;

            if cur_step_cost > 0 {
                match (visited_loc_costs.get(&(pos, MajorDirection::NorthSouth)), visited_loc_costs.get(&(pos, MajorDirection::EastWest))) {
                    (Some((dir1, cost1)), Some((dir2, cost2))) => {
                        //println!("pos: {:?}, cost: {:?}, cost1: {cost1}, dir1: {:?}, cost2: {cost2}, dir2: {:?}", pos, cost, dir1, dir2);
                        if *cost1 % 1000 == cur_step_cost && *cost1 / 1000 <= cost.1 {
                            tiles_on_backwards_paths.push_back((get_backwards_tile(pos, dir1), (cur_step_cost - 1, cost1 / 1000)));
                        }
                        if *cost2 % 1000 == cur_step_cost && *cost2 / 1000 <= cost.1 {
                            tiles_on_backwards_paths.push_back((get_backwards_tile(pos, dir2), (cur_step_cost - 1, cost1 / 1000)));
                        }
                    },
                    (Some((dir1, cost1)), None) => {
                        assert_eq!(*cost1 % 1000, cur_step_cost);
                        tiles_on_backwards_paths.push_back((get_backwards_tile(pos, dir1), (cur_step_cost - 1, cost1 / 1000)));
                    },
                    (None, Some((dir2, cost2))) => {
                        assert_eq!(*cost2 % 1000, cur_step_cost);
                        tiles_on_backwards_paths.push_back((get_backwards_tile(pos, dir2), (cur_step_cost - 1, cost2 / 1000)));
                    },
                    (None, None) => {
                        /* Nothing to do if no tile this direction */
                    }
                }
            }
        }
    }
    const DEBUG: bool = false;
    if DEBUG {
        println!("Checking tiles on best paths:");
        for (yindex, row) in (&day.maze).into_iter().enumerate() {
            for (xindex, val) in row.into_iter().enumerate() {
                if let None = tiles_on_best_paths.get(&(yindex, xindex)) {
                    print!("{val}");
                } else {
                    print!("O");
                }
            }
            println!("");
        }
    }
    tiles_on_best_paths.len()
}

pub fn part2(day: &Day16) -> usize {
    let (best_cost, visited_loc_costs) = flood_to_best_paths(day);
    count_num_tiles_walking_back_best_paths(day, best_cost, &visited_loc_costs)
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

    #[test]
    fn first_example_part2_equals_45() {
        let day = Day16::new(FIRST_EXAMPLE.lines());
        assert_eq!(45, part2(&day));
    }

    #[test]
    fn second_example_part2_equals_64() {
        let day = Day16::new(SECOND_EXAMPLE.lines());
        assert_eq!(64, part2(&day));
    }
}
