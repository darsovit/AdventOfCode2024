use regex::Regex;

#[derive(Debug)]
pub struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

pub struct Day13 {
    claw_machines: Vec<ClawMachine>,
    /*
    button_a_travel: (usize, usize),
    button_b_travel: (usize, usize),
    prize_position: (usize, usize), */
}

impl Day13 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let button_regex = Regex::new(r"^Button (A|B): X\+(\d+), Y\+(\d+)$").unwrap();
                           //Regex::new(r"^Button (A|B): X+(\d+), Y+(\d+)$").unwrap();
        let prize_regex = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();
        let mut line_iter = lines.into_iter();
        let mut done = false;
        let mut claw_machines = Vec::new();

        while !done {
            let button_a = button_regex.captures(line_iter.next().unwrap()).unwrap();
            let button_b = button_regex.captures(line_iter.next().unwrap()).unwrap();
            let prize    = prize_regex.captures(line_iter.next().unwrap()).unwrap();
            assert_eq!(button_a[1], *"A");
            assert_eq!(button_b[1], *"B");
            let button_a = (button_a[2].parse::<i64>().unwrap(), button_a[3].parse::<i64>().unwrap());
            let button_b = (button_b[2].parse::<i64>().unwrap(), button_b[3].parse::<i64>().unwrap());
            let prize    = (prize[1].parse::<i64>().unwrap(), prize[2].parse::<i64>().unwrap());

            claw_machines.push(ClawMachine{button_a, button_b, prize});
            let blank_line = line_iter.next();
            if let None = blank_line { done = true; }
        }
        Day13{claw_machines}
    }

    fn find_solution_through_a_mult(claw_machine: &ClawMachine) -> Option<(i64, i64)> {
        // A.0 * X + B.0 * Y = C.0
        // A.1 * X + B.1 * Y = C.1
        // A.1 * X + B.1 * Y - (A.0 * X + B.0 * Y) * (B.1/B.0) = C.1 - C.0 * (B.1/B.0)
        // A.1 * X - (B.1/B.0) * A.0 * X = (C.1 - C.0 * (B.1/B.0))
        // X * (A.1 - (B.1/B.0) * A.0) = (C.1 - C.0 * (B.1/B.0))
        // X = (C.1 - C.0 * (B.1/B.0)) / (A.1 - (B.1/B.0) * A.0)
        let b_ratio: f64 = claw_machine.button_b.1 as f64 / claw_machine.button_b.0 as f64;
        let a_mult: f64 = (claw_machine.prize.1 as f64 - b_ratio * claw_machine.prize.0 as f64) / (claw_machine.button_a.1 as f64 - b_ratio * claw_machine.button_a.0 as f64);

        let a_mult_rounded = a_mult.round() as i64;

        let b_mult: i64 = (claw_machine.prize.0 - claw_machine.button_a.0 * a_mult_rounded) / claw_machine.button_b.0;
        let test_1 = claw_machine.button_a.0 * a_mult_rounded + claw_machine.button_b.0 * b_mult == claw_machine.prize.0;
        let test_2 = claw_machine.button_a.1 * a_mult_rounded + claw_machine.button_b.1 * b_mult == claw_machine.prize.1;
        
        if test_1 && test_2 {
            Some((a_mult_rounded, b_mult))
        }
        else {
            None
        }
    }

    fn find_solution_through_b_mult(claw_machine: &ClawMachine) -> Option<(i64, i64)> {
        // A.0 * X + B.0 * Y = C.0
        // A.1 * X + B.1 * Y = C.1
        // A.1 * X + B.1 * Y - A.0 * (A.1/A.0) * X - B.0 * (A.1/A.0) * Y = C.1 - C.0 * (A.1/A.0)
        // B.1 * Y - B.0 * (A.1 / A.0) * Y = C.1 - C.0 * (A.1/A.0)
        // (B.1 - B.0 * (A.1 / A.0)) * Y = (C.1 - C.0 * (A.1 / A.0))
        // Y = (C.1 - C.0 * (A.1 / A.0)) / (B.1 - B.0 * (A.1 / A.0))
        let a_ratio: f64 = claw_machine.button_a.1 as f64 / claw_machine.button_a.0 as f64;
        let b_mult: f64 = (claw_machine.prize.1 as f64 - a_ratio * claw_machine.prize.0 as f64) / (claw_machine.button_b.1 as f64 - claw_machine.button_b.0 as f64 * a_ratio);

        // A.0 * X + B.0 * Y = C.0
        // X = (C.0 - B.0 * Y) / A.0
        let a_mult: f64 = (claw_machine.prize.0 as f64 - b_mult * claw_machine.button_b.0 as f64) / claw_machine.button_a.0 as f64;

        match ((a_mult as i64) as f64 == a_mult && a_mult as i64 >= 0, (b_mult as i64) as f64 == b_mult && b_mult as i64 >= 0) {
            (true, true) => Some((a_mult as i64, b_mult as i64)),
            (_, _) => None
        }
    }

    pub fn find_solution_and_cost(claw_machine: &ClawMachine, costs: (i64, i64)) -> Option<((i64, i64), i64)> {
        match (Self::find_solution_through_b_mult(claw_machine), Self::find_solution_through_a_mult(claw_machine)) {
            (Some((a1, b1)), Some((a2, b2))) => {
                let cost_for_1 = costs.0 * a1 + costs.1 * b1;
                let cost_for_2 = costs.0 * a2 + costs.1 * b2;
                if cost_for_1 < cost_for_2 { Some(((a1, b1), cost_for_1)) } else { Some(((a2, b2), cost_for_2)) }
            },
            (Some((a1, b1)), None) => {
                Some(((a1, b1), costs.0 * a1 + costs.1 * b1))
            },
            (None, Some((a2,b2))) => {
                Some(((a2, b2), costs.0 * a2 + costs.1 * b2))
            },
            (None, None) => None
        }
        // A.0 * X + B.0 * Y = C.0
        // A.1 * X + B.1 * Y = C.1
        // A.1 * X + B.1 * Y - A.0 * (A.1/A.0) * X - B.0 * (A.1/A.0) * Y = C.1 - C.0 * (A.1/A.0)
        // B.1 * Y - B.0 * (A.1 / A.0) * Y = C.1 - C.0 * (A.1/A.0)
        // (B.1 - B.0 * (A.1 / A.0)) * Y = (C.1 - C.0 * (A.1 / A.0))
        // Y = (C.1 - C.0 * (A.1 / A.0)) / (B.1 - B.0 * (A.1 / A.0))
        /* 
        let a_ratio: f64 = claw_machine.button_a.1 as f64 / claw_machine.button_a.0 as f64;
        let b_mult: f64 = (claw_machine.prize.1 as f64 - a_ratio * claw_machine.prize.0 as f64) / (claw_machine.button_b.1 as f64 - claw_machine.button_b.0 as f64 * a_ratio);

        // A.0 * X + B.0 * Y = C.0
        // X = (C.0 - B.0 * Y) / A.0
        let a_mult: f64 = (claw_machine.prize.0 as f64 - b_mult * claw_machine.button_b.0 as f64) / claw_machine.button_a.0 as f64;
        println!("A Button presses: {}, B Button Presses: {}", a_mult, b_mult);

        match ((a_mult as i64) as f64 == a_mult, (b_mult as i64) as f64 == b_mult) {
            (true, true) => Some(((a_mult as i64, b_mult as i64), a_mult as i64 * costs.0 + b_mult as i64 * costs.1)),
            (_, _) => None
        }
        */
    }

    pub fn part1(&self) -> i64 {
        let mut sum_of_costs = 0;
        for claw_machine in &self.claw_machines {
            if let Some(((_a_mult, _b_mult), cost)) = Self::find_solution_and_cost(claw_machine, (3, 1)) {
                assert!(_a_mult <= 100);
                assert!(_b_mult <= 100);
                assert!(cost > 0);
                sum_of_costs += cost;
            }
        }
        sum_of_costs
    }

    pub fn part2(&self) -> i64 {
        let mut sum_of_costs = 0;
        const PRIZE_ADDED_DISTANCE: i64 = 10000000000000;
        for claw_machine in &self.claw_machines {
            let corrected_claw_machine = ClawMachine{button_a: claw_machine.button_a, button_b: claw_machine.button_b, prize: (PRIZE_ADDED_DISTANCE + claw_machine.prize.0, PRIZE_ADDED_DISTANCE + claw_machine.prize.1)};
            if let Some(((_a_mult, _b_mult), cost)) = Self::find_solution_and_cost(&corrected_claw_machine, (3, 1)) {
                sum_of_costs += cost;
            }
        }
        sum_of_costs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str =
"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    #[test]
    fn sample_input_part1_results_in_480() {
        let day = Day13::new(SAMPLE_INPUT.lines());
        assert_eq!(480, day.part1());
    }
}