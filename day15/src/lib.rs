use std::collections::HashSet;

pub struct Day15 {
    map: Vec<Vec<char>>,
    robot_pos: (usize, usize),
    directions: Vec<char>,
}

impl Day15 {

    fn build_map_from_row(map: &mut Vec<Vec<char>>, row: Vec<char>) -> Option<usize> {
        let mut robot_pos: Option<usize> = None;
        for (xindex, pos) in (&row).into_iter().enumerate() {
            if *pos == '@' {
                assert_eq!(None, robot_pos);
                robot_pos = Some(xindex);
            }
        }
        map.push(row);
        robot_pos        
    }

    fn build_part2_map_from_row(map: &mut Vec<Vec<char>>, row: Vec<char>) -> Option<usize> {
        let mut robot_pos: Option<usize> = None;
        let mut wide_row: Vec::<char> = Vec::new();

        for (xindex, pos) in (&row).into_iter().enumerate() {
            match *pos {
                '#' => { wide_row.append(&mut vec!['#', '#']); },
                'O' => { wide_row.append(&mut vec!['[', ']']); },
                '.' => { wide_row.append(&mut vec!['.', '.']); },
                '@' => {
                    wide_row.append(&mut vec!['@', '.']);
                    assert_eq!(None, robot_pos);
                    robot_pos = Some(xindex * 2);
                }
                another => { panic!("Found an unexpected item in the map: {}", another); }
            }
        }
        map.push(wide_row);
        robot_pos
    }

    fn consume_input_to_build_day(lines: std::str::Lines<'_>, interpret_map: fn (&mut Vec<Vec<char>>, Vec<char>) -> Option<usize>) -> Self {
        let mut map: Vec::<Vec<char>> = Vec::new();
        let mut robot_pos: Option<(usize, usize)> = None;
        let mut directions: Vec::<char> = Vec::new();
        let mut read_map = false;

        for (yindex, line) in lines.enumerate() {
            if !read_map && line != "" {
                let row: Vec::<char> = line.chars().collect();
                if let Some(xindex) = interpret_map(&mut map, row) {
                    assert_eq!(None, robot_pos);
                    robot_pos = Some((yindex, xindex));
                }
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

    pub fn new(lines: std::str::Lines<'_>) -> Self {
        Self::consume_input_to_build_day(lines, Self::build_map_from_row)
    }

    pub fn new_part2(lines: std::str::Lines<'_>) -> Self {
        Self::consume_input_to_build_day(lines, Self::build_part2_map_from_row)
    }

    fn move_up(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) {
        if pos.0 > 0 {
            if map[pos.0 - 1][pos.1] == '#' { return; }
            if map[pos.0 - 1][pos.1] != '.' {
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
            if map[pos.0][pos.1 + 1] != '.' {
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
            if map[pos.0 + 1][pos.1] != '.' {
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
            if map[pos.0][pos.1 - 1] != '.' {
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

        for direction in &self.directions {
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

    fn build_ranges_to_move(items_at_needed_positions: &Vec<(&[char], (usize, usize))>) -> Vec<(usize, usize)> {
        let mut ranges_to_move = Vec::<(usize, usize)>::new();
        for (items_slice, positions) in items_at_needed_positions {
            let mut offset = 0;
            while positions.0 + offset < positions.1 {
                if items_slice[offset] == ']' && offset == 0 {
                    ranges_to_move.push((positions.0-1, positions.0 + 1));
                    offset += 1;
                } else if items_slice[offset] == '[' {
                    ranges_to_move.push((positions.0+offset, positions.0+offset+2));
                    offset += 2; /* skip past the closing char of box */
                } else if items_slice[offset] == '.' {
                    offset += 1;
                } else {
                    panic!("Unknown item {} found at {offset}", items_slice[offset]);
                }
            }
        }
        ranges_to_move
    }

    fn get_items_above_ranges_to_move<'a>(map: &'a Vec<Vec<char>>, ranges_to_move: &'a(usize, Vec<(usize, usize)>)) -> Vec<(&'a [char], (usize, usize))> {
        let mut items_at_needed_pos: Vec<(&[char], (usize, usize))> = Vec::new();
        let cur_y = ranges_to_move.0;
        for range_to_move in &ranges_to_move.1 {
            let items_above_range: &[char] = &map[cur_y - 1][range_to_move.0..range_to_move.1];
            items_at_needed_pos.push((items_above_range, *range_to_move));
        }
        items_at_needed_pos
    }

    fn get_items_below_ranges_to_move<'a>(map: &'a Vec<Vec<char>>, ranges_to_move: &'a(usize, Vec<(usize, usize)>)) -> Vec<(&'a [char], (usize, usize))> {
        let mut items_at_needed_pos: Vec<(&[char], (usize, usize))> = Vec::new();
        let cur_y = ranges_to_move.0;
        for range_to_move in &ranges_to_move.1 {
            let items_below_range: &[char] = &map[cur_y + 1][range_to_move.0..range_to_move.1];
            items_at_needed_pos.push((items_below_range, *range_to_move));
        }
        items_at_needed_pos
    }

    fn move_up_range(map: &mut Vec<Vec<char>>, ranges_to_move: &(usize, Vec<(usize, usize)>)) -> bool {
        let cur_y = ranges_to_move.0;
        if 0 < cur_y {
            let items_at_needed_positions = Self::get_items_above_ranges_to_move(map, ranges_to_move);
            let mut all_space = true;

            for items in &items_at_needed_positions {
                for item in items.0 {
                    if *item == '#' { return false; } // can't push up, we're blocked
                    if *item != '.' { all_space = false; } // can't push up til we push up above us
                }
            }

            if !all_space {
                let ranges_to_move = (cur_y - 1, Self::build_ranges_to_move(&items_at_needed_positions));
                if !Self::move_up_range(map, &ranges_to_move) { return false; }
            }

            let mut moved_xindex = HashSet::<usize>::new();
            for range_to_move in &ranges_to_move.1 {
                for xindex in range_to_move.0..range_to_move.1 {
                    if let None = moved_xindex.get(&xindex) {
                        moved_xindex.insert(xindex);
                        assert_eq!('.', map[cur_y-1][xindex]);
                        assert!(map[cur_y][xindex] == '[' || map[cur_y][xindex] == ']');
                        map[cur_y-1][xindex] = map[cur_y][xindex];
                        map[cur_y][xindex] = '.';
                    }
                }
            }
            return true;
        }
        false
    }

    fn move_down_range(map: &mut Vec<Vec<char>>, ranges_to_move: &(usize, Vec<(usize, usize)>)) -> bool {
        let cur_y = ranges_to_move.0;
        if cur_y + 1 < map.len() {
            let items_at_needed_positions = Self::get_items_below_ranges_to_move(map, ranges_to_move);
            let mut all_space = true;

            for items in &items_at_needed_positions {
                for item in items.0 {
                    if *item == '#' { return false; } // can't push up, we're blocked
                    if *item != '.' { assert!(*item == '[' || *item == ']'); all_space = false; } // can't push up til we push up above us
                }
            }

            if !all_space {
                let ranges_to_move = (cur_y + 1, Self::build_ranges_to_move(&items_at_needed_positions));
                if !Self::move_down_range(map, &ranges_to_move) { return false; }
            }

            let mut moved_xindex = HashSet::<usize>::new();
            for range_to_move in &ranges_to_move.1 {
                for xindex in range_to_move.0..range_to_move.1 {
                    if let None = moved_xindex.get(&xindex) {
                        moved_xindex.insert(xindex);
                        assert_eq!('.', map[cur_y+1][xindex]);
                        assert!(map[cur_y][xindex] == '[' || map[cur_y][xindex] == ']');
                        map[cur_y+1][xindex] = map[cur_y][xindex];
                        map[cur_y][xindex] = '.';
                    }
                }
            }
            return true;
        }
        false
    }

    fn move_up_wide(map: &mut Vec<Vec<char>>, robot_pos: &mut (usize, usize)) {
        if 0 < robot_pos.0 {
            let item_at_new_pos = map[robot_pos.0 - 1][robot_pos.1];
            if item_at_new_pos == '#' { return; }
            let mut moved_up_range = true;
            if item_at_new_pos != '.' {
                assert!(item_at_new_pos == '[' || item_at_new_pos == ']');
                let box_range = if item_at_new_pos == '[' { (robot_pos.1, robot_pos.1 + 2) } else { (robot_pos.1 - 1, robot_pos.1 + 1) };
                let range_to_move: (usize, Vec<(usize, usize)>) = (robot_pos.0 - 1, vec![box_range]);
                moved_up_range = Self::move_up_range(map, &range_to_move);
            }
            if map[robot_pos.0 - 1][robot_pos.1] == '.' {
                assert!(moved_up_range);
                map[robot_pos.0 - 1][robot_pos.1] = map[robot_pos.0][robot_pos.1];
                map[robot_pos.0][robot_pos.1] = '.';
                *robot_pos = (robot_pos.0 - 1, robot_pos.1);
            }
        }
    }

    fn move_down_wide(map: &mut Vec<Vec<char>>, robot_pos: &mut (usize, usize)) {
        if robot_pos.0 + 1 < map.len() {
            let item_at_new_pos = map[robot_pos.0 + 1][robot_pos.1];
            if item_at_new_pos == '#' { return; }
            let mut moved_range = true;
            if item_at_new_pos != '.' {
                assert!(item_at_new_pos == '[' || item_at_new_pos == ']');
                let box_range = if item_at_new_pos == '[' { (robot_pos.1, robot_pos.1 + 2) } else { (robot_pos.1 - 1, robot_pos.1 + 1) };
                let range_to_move = (robot_pos.0 + 1, vec![box_range]);
                moved_range = Self::move_down_range(map, &range_to_move);
            }
            if map[robot_pos.0 + 1][robot_pos.1] == '.' {
                assert!(moved_range);
                map[robot_pos.0 + 1][robot_pos.1] = map[robot_pos.0][robot_pos.1];
                map[robot_pos.0][robot_pos.1] = '.';
                *robot_pos = (robot_pos.0 + 1, robot_pos.1);
            }
        }
    }

    pub fn part2(&self) -> usize {
        let mut map = self.map.clone();
        let mut robot_pos: (usize, usize) = self.robot_pos;
        let debug = false;
        for direction in &self.directions {
            match direction {
                '^' => { Self::move_up_wide(&mut map, &mut robot_pos); },
                '>' => { Self::move_right(&mut map, &mut robot_pos); },
                'v' => { Self::move_down_wide(&mut map, &mut robot_pos); },
                '<' => { Self::move_left(&mut map, &mut robot_pos); },
                something_else => { panic!("Invalid direction: {}", something_else); }
            }
            if debug {
                println!("After: {direction}:");
                for row in &map {
                    for a_char in row {
                        print!("{a_char}");
                    }
                    println!("");
                }
            }
        }

        let mut sum_of_boxes = 0;
        for (yindex, row) in (&map).into_iter().enumerate() {
            for (xindex, col) in row.into_iter().enumerate() {
                if '[' == *col {
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

    #[test]
    fn small_sample_part2_results_in_105() {
        const SMALL_SAMPLE_PART2: &str =
"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        let day = Day15::new_part2(SMALL_SAMPLE_PART2.lines());
        assert_eq!(105 + 207 + 306, day.part2());
    }
    #[test]
    fn sample_part2_results_in_9021() {
        let day = Day15::new_part2(SAMPLE_INPUT.lines());
        assert_eq!(9021, day.part2());
    }
}
