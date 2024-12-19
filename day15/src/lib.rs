pub struct Day15 {
    map: Vec<Vec<char>>,
    robot_pos: (usize, usize),
    directions: Vec<char>,
}

impl Day15 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut map: Vec::<Vec<char>> = Vec::new();
        let mut robot_pos: Option<(usize, usize)> = None;
        let mut directions: Vec::<char> = Vec::new();
        let mut read_map = false;

        for (yindex, line) in lines.enumerate() {
            if !read_map && line != "" {
                let row: Vec::<char> = line.chars().collect();
                for (xindex, pos) in (&row).into_iter().enumerate() {
                    if *pos == '@' {
                        assert_eq!(None, robot_pos);
                        robot_pos = Some((yindex, xindex));
                    }
                }
                map.push(row);
            } else if !read_map && line == "" {
                read_map = true;
            } else {
                assert!(read_map);
                let mut row_of_directions: Vec::<char> = line.chars().collect();
                directions.append(&mut row_of_directions);
            }
        }
        let robot_pos = robot_pos.unwrap();
        assert_eq!('@', map[robot_pos.0][robot_pos.1]);
        Day15{map, robot_pos, directions}
    }

    fn move_up(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) {
        if pos.0 > 0 {
            if map[pos.0 - 1][pos.1] == '#' { return; }
            if map[pos.0 - 1][pos.1] == 'O' {
                let mut adjusted_pos: (usize, usize) = (pos.0 - 1, pos.1);
                Self::move_up(map, &mut adjusted_pos);
            }
            if map[pos.0 - 1][pos.1] == '.'  {
                map[pos.0 - 1][pos.1] = map[pos.0][pos.1];
                map[pos.0][pos.1] = '.';
                *pos = (pos.0 - 1, pos.1);
            }
        }
    }

    fn move_right(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) {
        if pos.1 + 1 < map[0].len() {
            if map[pos.0][pos.1 + 1] == '#' { return; }
            if map[pos.0][pos.1 + 1] == 'O' {
                let mut adjusted_pos: (usize, usize) = (pos.0, pos.1 + 1);
                Self::move_right(map, &mut adjusted_pos);
            }
            if map[pos.0][pos.1 + 1] == '.' {
                map[pos.0][pos.1 + 1] = map[pos.0][pos.1];
                map[pos.0][pos.1] = '.';
                *pos = (pos.0, pos.1 + 1);
            }
        }
    }

    fn move_down(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) {
        if pos.0 + 1 < map.len() {
            if map[pos.0 + 1][pos.1] == '#' { return; }
            if map[pos.0 + 1][pos.1] == 'O' {
                let mut adjusted_pos: (usize, usize) = (pos.0 + 1, pos.1);
                Self::move_down(map, &mut adjusted_pos);
            }
            if map[pos.0 + 1][pos.1] == '.' {
                map[pos.0 + 1][pos.1] = map[pos.0][pos.1];
                map[pos.0][pos.1] = '.';
                *pos = (pos.0 + 1, pos.1)
            }
        }
    }

    fn move_left(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) {
        if pos.1 > 0 {
            if map[pos.0][pos.1 - 1] == '#' { return; }
            if map[pos.0][pos.1 - 1] == 'O' {
                let mut adjusted_pos: (usize, usize) = (pos.0, pos.1 - 1);
                Self::move_left(map, &mut adjusted_pos);
            }
            if map[pos.0][pos.1 - 1] == '.'  {
                map[pos.0][pos.1 - 1] = map[pos.0][pos.1];
                map[pos.0][pos.1] = '.';
                *pos = (pos.0, pos.1 - 1);
            }
        }
    }
    pub fn part1(&self) -> usize {
        let mut map = self.map.clone();
        let mut robot_pos: (usize, usize) = self.robot_pos;
        let mut directions = self.directions.clone();

        for direction in directions {
            match direction {
                '^' => { Self::move_up(&mut map, &mut robot_pos); },
                '>' => { Self::move_right(&mut map, &mut robot_pos); },
                'v' => { Self::move_down(&mut map, &mut robot_pos); },
                '<' => { Self::move_left(&mut map, &mut robot_pos); },
                something_else => { panic!("Invalid direction: {}", something_else); }
            }
        }

        let mut sum_of_boxes = 0;
        for (yindex, row) in (&map).into_iter().enumerate() {
            for (xindex, col) in row.into_iter().enumerate() {
                if 'O' == *col {
                    sum_of_boxes += yindex * 100 + xindex;
                }
            }
        }
        sum_of_boxes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SMALL_SAMPLE: &str =
"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const SAMPLE_INPUT: &str =
"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn small_sample_part1_results_in_2028() {
        let day = Day15::new(SMALL_SAMPLE.lines());
        assert_eq!(2028, day.part1());
    }

    #[test]
    fn sample_part1_results_in_10092() {
        let day = Day15::new(SAMPLE_INPUT.lines());
        assert_eq!(10092, day.part1());
    }
}
