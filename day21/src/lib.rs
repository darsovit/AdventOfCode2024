use regex::Regex;
use std::fmt;
use std::collections::HashMap;

pub struct Day21 {
    codes: Vec<String>,
}

#[derive(Debug, Copy, Clone)]
enum NumpadControl {
    Activate,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum RobotControl {
    Up,
    Activate,
    Left,
    Down,
    Right,
}

impl fmt::Debug for RobotControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
            match self {
                RobotControl::Up => "^",
                RobotControl::Down => "v",
                RobotControl::Left => "<",
                RobotControl::Right => ">",
                RobotControl::Activate => "A",
            }
        )
    }
}

fn get_robot_control_position(button: &RobotControl) -> (usize, usize) {
    match button {
        RobotControl::Activate => (0, 1),
        RobotControl::Up       => (1, 1),
        RobotControl::Left     => (2, 0),
        RobotControl::Down     => (1, 0),
        RobotControl::Right    => (0, 0),
    }
}

fn get_numpad_position(button: &NumpadControl) -> (usize, usize) {
    match button {
        NumpadControl::Activate => (0,0),
        NumpadControl::Zero     => (1,0),
        NumpadControl::One      => (2,1),
        NumpadControl::Two      => (1,1),
        NumpadControl::Three    => (0,1),
        NumpadControl::Four     => (2,2),
        NumpadControl::Five     => (1,2),
        NumpadControl::Six      => (0,2),
        NumpadControl::Seven    => (2,3),
        NumpadControl::Eight    => (1,3),
        NumpadControl::Nine     => (0,3),
    }
}

fn move_and_activate_robot_arm(init: (usize, usize), dest: (usize, usize), bad_spot: (usize, usize)) -> Vec<RobotControl> {
    let mut robot_controls = Vec::<RobotControl>::new();
    let mut pos = init;

    // If our cursor is on the row with the bad spot and if our destination column is on the bad spot column
    // we want to move up or down first, otherwise, prefer moving left moves first because a robot controller has 
    // the left control furthest away from activate
    if dest.0 == bad_spot.0 && pos.1 == bad_spot.1 {
        while pos.1 > dest.1 {
            robot_controls.push(RobotControl::Down);
            pos = (pos.0, pos.1 - 1);
            assert_ne!(pos, bad_spot);
        }
        while pos.1 < dest.1 {
            robot_controls.push(RobotControl::Up);
            pos = (pos.0, pos.1 + 1);
            assert_ne!(pos, bad_spot);
        }
    }
    while pos.0 < dest.0 {
        robot_controls.push(RobotControl::Left);
        pos = (pos.0 + 1, pos.1);
        assert_ne!(pos, bad_spot);
    }
    // For numpad
    // if our destination row is the same row as the bad spot, and our position column matches, prefer
    // to go right before moving down onto the bad spot
    if dest.1 == bad_spot.1 && pos.0 == bad_spot.0 {
        while pos.0 > dest.0 {
            robot_controls.push(RobotControl::Right);
            pos = (pos.0 - 1, pos.1);
            assert_ne!(pos, bad_spot);
        }            
    }
    while pos.1 > dest.1 {
        robot_controls.push(RobotControl::Down);
        pos = (pos.0, pos.1 - 1);
        assert_ne!(pos, bad_spot);
    }
    while pos.0 > dest.0 {
        robot_controls.push(RobotControl::Right);
        pos = (pos.0 - 1, pos.1);
        assert_ne!(pos, bad_spot);
    }
    while pos.1 < dest.1 {
        robot_controls.push(RobotControl::Up);
        pos = (pos.0, pos.1 + 1);
        assert_ne!(pos, bad_spot);
    }

    robot_controls.push(RobotControl::Activate);
    robot_controls
}

fn numpad_control_sequence(init: NumpadControl, dest: &NumpadControl) -> Vec<RobotControl> {
    let start_pos = get_numpad_position(&init);
    let end_pos   = get_numpad_position(dest);
    move_and_activate_robot_arm(start_pos, end_pos, (2, 0))
}


fn robot_control_sequence(init: &RobotControl, dest: &RobotControl) -> Vec<RobotControl> {
    let start_pos = get_robot_control_position(init);
    let end_pos   = get_robot_control_position(dest);
    move_and_activate_robot_arm(start_pos, end_pos, (2, 1))
}

fn char_to_numpad_control(a_char: char) -> NumpadControl {
    match a_char {
        'A' => NumpadControl::Activate,
        '0' => NumpadControl::Zero,
        '1' => NumpadControl::One,
        '2' => NumpadControl::Two,
        '3' => NumpadControl::Three,
        '4' => NumpadControl::Four,
        '5' => NumpadControl::Five,
        '6' => NumpadControl::Six,
        '7' => NumpadControl::Seven,
        '8' => NumpadControl::Eight,
        '9' => NumpadControl::Nine,
        _ => panic!("Unknown character: {a_char} requested from numpad control"),
    }
}

fn numpad_sequence(output: &str) -> Vec<RobotControl> {
    let mut robot_controls = Vec::<RobotControl>::new();
    let mut last_position = NumpadControl::Activate;
    for next in output.chars() {
        let next = char_to_numpad_control(next);
        robot_controls.append(&mut numpad_control_sequence(last_position, &next));
        last_position = next;
    }
    robot_controls
}

fn robot_sequence(robot_controls: &Vec<RobotControl>) -> Vec<RobotControl> {
    let mut output_controls = Vec::<RobotControl>::new();
    let mut last_position = RobotControl::Activate;
    for control in robot_controls {
        output_controls.append(&mut robot_control_sequence(&last_position, control));
        last_position = *control;
    }
    output_controls
}

fn code_as_number(code: &str) -> usize {
    let number_re = Regex::new(r"(\d+)A").unwrap();
    let number = number_re.captures(code).unwrap();
    number[1].parse::<usize>().unwrap()
}

fn complexity_length(code: &str) -> usize {
    let user_sequence = robot_sequence(&mut robot_sequence(&mut numpad_sequence(code)));
    user_sequence.len()
}

fn robot_control_sequence_n(last_control: &RobotControl, next_control: &RobotControl, n: usize) -> Vec<RobotControl> {
    let mut result = robot_control_sequence(last_control, next_control);
    for _ in 0..n-1 {
        // Once we're down at least one level, it will always start at Activate
        result = robot_sequence(&result);
    }
    result
}

fn build_control_counts(sequence: &Vec<RobotControl>) -> HashMap<(RobotControl, RobotControl), usize> {
    let mut control_pattern_counts: HashMap<(RobotControl, RobotControl), usize> = HashMap::new();
    let mut last_control = RobotControl::Activate;
    for control in sequence {
        *control_pattern_counts.entry((last_control, *control)).or_insert(0) += 1;
        last_control = *control;
    }
    control_pattern_counts
}

fn calculate_next_5(lower_count: &HashMap<(RobotControl, RobotControl), usize>,
                    cache_of_5_levels: &mut HashMap<(RobotControl, RobotControl), HashMap<(RobotControl, RobotControl), usize>>)
         -> HashMap<(RobotControl, RobotControl), usize> {
    let mut new_count_of_sequences = HashMap::new();
    for (pattern, count) in lower_count {
        let result = cache_of_5_levels.entry(*pattern).or_insert(build_control_counts(&robot_control_sequence_n(&pattern.0, &pattern.1, 5)));
        for (result_pattern, result_count) in result {
            let val = new_count_of_sequences.entry(*result_pattern).or_insert(0);
            *val += *result_count * count;
        }
    }
    new_count_of_sequences
}

fn calculate_next(lower_count: &HashMap<(RobotControl, RobotControl), usize>,
                  cache_of_controls: &mut HashMap<(RobotControl, RobotControl), HashMap<(RobotControl, RobotControl), usize>>)
         -> HashMap<(RobotControl, RobotControl), usize> {
    let mut new_count_of_sequences = HashMap::new();
    for (pattern, count) in lower_count {
        let result = cache_of_controls.entry(*pattern).or_insert(build_control_counts(&robot_control_sequence_n(&pattern.0, &pattern.1, 1)));
        for (result_pattern, result_count) in result {
            let val = new_count_of_sequences.entry(*result_pattern).or_insert(0);
            *val += *result_count * count;
        }
    }
    new_count_of_sequences
}

fn pt2_complexity_length_n_dir_robots(code: &str, n: usize) -> usize {
    assert!(n > 1);
    let first_control_sequence = numpad_sequence(code);
    let mut cache_for_dir: HashMap<(RobotControl, RobotControl), HashMap<(RobotControl, RobotControl), usize>> = HashMap::new();

    let mut sequence_for_first_control = Vec::<RobotControl>::new();
    let mut last_control = RobotControl::Activate;
    for next_control in first_control_sequence {
        let mut sequence = robot_control_sequence(&last_control, &next_control);
        cache_for_dir.entry((last_control, next_control)).or_insert(build_control_counts(&sequence));
        sequence_for_first_control.append(&mut sequence);
        last_control = next_control;
    }

    let mut count_of_sequences = build_control_counts(&sequence_for_first_control);
    for _ in 0..n-1 {
        count_of_sequences = calculate_next(&count_of_sequences, &mut cache_for_dir);
    }

    let mut num_steps_at_n_deep = 0;
    for (_, count) in count_of_sequences {
        num_steps_at_n_deep += count;
    }
    num_steps_at_n_deep
}

fn pt2_complexity_length(code: &str) -> usize {
    let numpad_control_sequence = numpad_sequence(code);
    let mut cache_of_5_levels: HashMap<(RobotControl, RobotControl), HashMap<(RobotControl, RobotControl), usize>> = HashMap::new();

    let mut last_control = RobotControl::Activate;
    let mut sequence_for_5_for_all = Vec::<RobotControl>::new();
    for next_control in numpad_control_sequence {
        let mut sequence_for_5 = robot_control_sequence_n(&last_control, &next_control, 5);
        cache_of_5_levels.entry((last_control, next_control)).or_insert(build_control_counts(&sequence_for_5));
        sequence_for_5_for_all.append(&mut sequence_for_5);
        last_control = next_control;
    }

    let mut count_of_sequences = build_control_counts(&sequence_for_5_for_all);
    for _ in 0..4 {
        count_of_sequences = calculate_next_5(&count_of_sequences, &mut cache_of_5_levels);
    }
    let mut num_steps_at_25_deep: usize = 0;
    for (_, count) in count_of_sequences {
        num_steps_at_25_deep += count;
    }
    num_steps_at_25_deep
}

impl Day21 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut codes = Vec::<String>::new();
        for line in lines {
            codes.push(line.to_string());
        }
        Day21{codes}
    }

    pub fn part1(&self) -> usize {
        let mut sum_of_complexities = 0;
        for code in &self.codes {
            let pt1_complexity_via_pt2 = pt2_complexity_length_n_dir_robots(code, 2);
            let pt1_complexity_len = complexity_length(code);
            println!("{code}: {pt1_complexity_len} {pt1_complexity_via_pt2}");
            sum_of_complexities += pt1_complexity_len * code_as_number(code);
        }
        sum_of_complexities
    }

    pub fn part2(&self) -> usize {
        let mut sum_of_complexities = 0;

        for code in &self.codes {
            let pt2_complexity_len = pt2_complexity_length_n_dir_robots(code, 25);
            println!("{code}: {pt2_complexity_len}");
            sum_of_complexities += pt2_complexity_len * code_as_number(code);
        }
        println!("pt2, first: {sum_of_complexities}");
        sum_of_complexities = 0;
        for code in &self.codes {
            let pt2_complexity_len= pt2_complexity_length(code);
            println!("{code}: {pt2_complexity_len}");
            sum_of_complexities += pt2_complexity_len * code_as_number(code);
        }
        sum_of_complexities
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str =
"029A
980A
179A
456A
379A";

    #[test]
    fn validate_sequence_029A_length() {
        let robot_controls = numpad_sequence("029A");
        assert_eq!(12, robot_controls.len());
    }

    #[test]
    fn validate_sequence_029A_through_two_robots_length() {
        let result_robot_controls = robot_sequence(&mut numpad_sequence("029A"));
        assert_eq!(28, result_robot_controls.len());
    }

    #[test]
    fn validate_sequence_029A_through_three_robots_length() {
        let robot3_controls = robot_sequence(&mut robot_sequence(&mut numpad_sequence("029A")));

        assert_eq!(68, robot3_controls.len());
    }

    #[test]
    fn part1_sample_input_results_in_126384() {
        let day = Day21::new(SAMPLE_INPUT.lines());
        assert_eq!(126384, day.part1());
    }

    #[test]
    fn sample_029A_complexity_is_29() {
        assert_eq!(68, complexity_length("029A"));
    }

    #[test]
    fn sample_980A_complexity_is_60() {
        assert_eq!(60, complexity_length("980A"));
    }

    #[test]
    fn sample_179A_complexity_is_68() {
        assert_eq!(68, complexity_length("179A"));
    }

    #[test]
    fn sample_456A_complexity_is_64() {
        assert_eq!(64, complexity_length("456A"));
    }

    #[test]
    fn sample_379A_complexity_is_64() {
        assert_eq!(64, complexity_length("379A"));
        //               3                                  7              9                       A
        //         ^     A         ^^           <<          A       >>     A           vvv         A
        //    <    A  >  A    <    AA   v  <    AA  >>   ^  A   v   AA  ^  A   v  <    AAA  >   ^  A
        // v<<A >>^A vA ^A v<<A >>^AA v<A <A >>^AA vAA ^<A >A v<A >^AA <A >A v<A <A >>^AAA vA ^<A >A

        //               3                                 7
        //         ^     A               <<         ^^     A
        //    <    A  >  A    v    <<    AA  >   ^  AA  >  A
        // <v<A >>^A vA ^A  <vA   <AA >>^AA vA <^A >AA vA ^A <vA >^AA <A >A <v<A >A >^AAA vA <^A >A
    }
}
