use regex::Regex;

#[derive(Debug)]
pub struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

pub struct Day14 {
    robots: Vec<Robot>
}

impl Day14 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let robot_re = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();
        let mut robots = Vec::new();
        for line in lines {
            let values = robot_re.captures(line).unwrap();
            robots.push(Robot{position: (values[2].parse::<i64>().unwrap(), values[1].parse::<i64>().unwrap()),
                              velocity: (values[4].parse::<i64>().unwrap(), values[3].parse::<i64>().unwrap())});
        }
        Day14{robots}
    }

    fn find_loc_robot(robot: &Robot, time: usize, height: usize, width: usize) -> Robot {
        let mut new_pos = ((robot.position.0 + time as i64 * robot.velocity.0) % height as i64, (robot.position.1 + time as i64 * robot.velocity.1) % width as i64);
        if 0 > new_pos.0 {
            new_pos = (new_pos.0 + height as i64, new_pos.1);
        }
        if 0 > new_pos.1 {
            new_pos = (new_pos.0, new_pos.1 + width as i64);
        }
        Robot{position: new_pos, velocity: robot.velocity}
    }

    fn part1_sized(&self, height: usize, width: usize) -> usize {
        println!("{:?}", self.robots);
        let mut quadrant1_count = 0;
        let mut quadrant2_count = 0;
        let mut quadrant3_count = 0;
        let mut quadrant4_count = 0;
        const TIME: usize = 100;
        let width_divider = width / 2;
        let height_divider = height / 2;

        for robot in &self.robots {
            let robot_pos = Self::find_loc_robot(robot, TIME, height, width).position;
            let robot_pos = (robot_pos.0 as usize, robot_pos.1 as usize);
            if robot_pos.0 < height_divider && robot_pos.1 < width_divider {
                quadrant1_count += 1;
            }
            else if robot_pos.0 > height_divider && robot_pos.1 < width_divider {
                quadrant3_count += 1;
            }
            else if robot_pos.0 < height_divider && robot_pos.1 > width_divider {
                quadrant2_count += 1;
            }
            else if robot_pos.0 > height_divider && robot_pos.1 > width_divider {
                quadrant4_count += 1;
            }
            else {
                println!("robot found on edge: {:?}", robot_pos);
                assert!(robot_pos.0 == height_divider || robot_pos.1 == width_divider);
            }
        }
        println!("{quadrant1_count} * {quadrant2_count} * {quadrant3_count} * {quadrant4_count}");
        quadrant1_count * quadrant2_count * quadrant3_count * quadrant4_count
    }
    pub fn part1(&self) -> usize {
        self.part1_sized(103, 101)
    }
    pub fn part2(&self) -> usize {
        let mut time: usize = 1;
        loop {
            let mut display = vec![ vec![ '.'; 101]; 103];

            for robot in &self.robots {
                let robot_pos = Self::find_loc_robot(robot, time, 103, 101).position;
                let robot_pos = (robot_pos.0 as usize, robot_pos.1 as usize);
                display[robot_pos.0][robot_pos.1] = '*';
            }
            println!("Display at {time}");
            for line in &display {
                for pos in line {
                    print!("{pos}");
                }
                println!("");
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
            time += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str =
"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn sample_input_part1_results_in_12() {
        let day = Day14::new(SAMPLE_INPUT.lines());
        assert_eq!(12, day.part1_sized(7, 11));
    }

    #[test]
    fn find_loc_robot_1_sec() {
        let robot = Robot{position:(4,2), velocity:(-3,2)};
        assert_eq!((1,4), Day14::find_loc_robot(&robot, 1, 7, 11).position);
    }

    #[test]
    fn find_loc_robot_2_sec() {
        let robot = Robot{position:(4,2), velocity:(-3,2)};
        assert_eq!((5, 6), Day14::find_loc_robot(&robot, 2, 7, 11).position);
    }

    #[test]
    fn find_loc_robot_3_sec() {
        let robot = Robot{position:(4,2), velocity:(-3,2)};
        assert_eq!((2, 8), Day14::find_loc_robot(&robot, 3, 7, 11).position);
    }

    #[test]
    fn find_loc_robot_4_sec() {
        let robot = Robot{position:(4,2), velocity:(-3,2)};
        assert_eq!((6, 10), Day14::find_loc_robot(&robot, 4, 7, 11).position);
    }

    #[test]
    fn find_loc_robot_5_sec() {
        let robot = Robot{position:(4,2), velocity:(-3,2)};
        assert_eq!((3, 1), Day14::find_loc_robot(&robot, 5, 7, 11).position);
    }
}
