pub struct Day04<'a> {
    input: Vec<&'a str>
}

impl<'a> Day04<'a> {
    pub fn new(lines: std::str::Lines<'a>) -> Self {
        Day04{input: lines.collect()}
    }


    fn upwards(&self, row: usize, column: usize) -> u32 {
        let mut xmases = 0;
        if row >= 3 {
            if column >= 3 {
                xmases += match (&self.input[row-1][column-1..column], &self.input[row-2][column-2..column-1], &self.input[row-3][column-3..column-2]) {
                    ("M", "A", "S") => 1,
                    (_, _, _) => 0
                };
            }
            xmases += match (&self.input[row-1][column..column+1], &self.input[row-2][column..column+1], &self.input[row-3][column..column+1]) {
                ("M", "A", "S") => 1,
                (_, _, _) => 0
            };
            if self.input[row-3].len() > column+3 {
                xmases += match (&self.input[row-1][column+1..column+2], &self.input[row-2][column+2..column+3], &self.input[row-3][column+3..column+4]) {
                    ("M", "A", "S") => 1,
                    (_, _, _) => 0
                };
            }
        }
        xmases
    }

    fn downwards(&self, row: usize, column: usize) -> u32 {
        let mut xmases = 0;
        if row + 3 < self.input.len() {
            if column >= 3 {
                xmases += match (&self.input[row+1][column-1..column], &self.input[row+2][column-2..column-1], &self.input[row+3][column-3..column-2]) {
                    ("M", "A", "S") => 1,
                    (_, _, _) => 0
                };
            }

            xmases += match (&self.input[row+1][column..column+1], &self.input[row+2][column..column+1], &self.input[row+3][column..column+1]) {
                ("M", "A", "S") => 1,
                (_, _, _) => 0
            };

            if self.input[row+3].len() > column+3 {
                xmases += match (&self.input[row+1][column+1..column+2], &self.input[row+2][column+2..column+3], &self.input[row+3][column+3..column+4]) {
                    ("M", "A", "S") => 1,
                    (_, _, _) => 0
                };
            }
        }
        xmases
    }

    fn sideways(&self, row: usize, column: usize) -> u32 {
        let mut count: u32 = 0;
        if self.input[row].len() > column+3 {
            if &self.input[row][column..column+4] == "XMAS" { count += 1; }
        }
        if column >= 3 {
            if &self.input[row][column-3..column+1] == "SAMX" { count += 1; }
        }
        count
    }
    pub fn part1(&self) -> u32 {
        let mut sum_of_xmases: u32 = 0;
        for (index, row) in self.input.clone().into_iter().enumerate() {
            for jindex in row.match_indices("X").map(|(i, _)|i) {
                let count_of_xmases_from_here = self.upwards(index, jindex) + self.downwards(index, jindex) + self.sideways(index, jindex);
                sum_of_xmases += count_of_xmases_from_here;
            }
        }
        sum_of_xmases
    }

    fn test_for_mas_cross(&self, a_row: usize, a_col: usize) -> bool {

        if a_row > 0 && a_row+1 < self.input.len() && a_col > 0 && a_col+1 < self.input[a_row].len() {
            match (&self.input[a_row-1][a_col-1..a_col], &self.input[a_row+1][a_col-1..a_col], &self.input[a_row-1][a_col+1..a_col+2], &self.input[a_row+1][a_col+1..a_col+2]) {
                ("M", "M", "S", "S") => true,
                ("M", "S", "M", "S") => true,
                ("S", "M", "S", "M") => true,
                ("S", "S", "M", "M") => true,
                (_, _, _, _) => false
            }
        }
        else {
            false
        }
    }

    pub fn part2(&self) -> u32 {
        let mut sum_of_mas_crosses: u32 = 0;
        for (index, row) in self.input.clone().into_iter().enumerate() {
            for jindex in row.match_indices("A").map(|(i, _)|i) {
                if self.test_for_mas_cross(index, jindex) {
                    sum_of_mas_crosses += 1;
                }
            }
        }
        sum_of_mas_crosses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1_for_part1_is_4() {
        const SAMPLE_INPUT: &str =
"..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        let day = Day04::new(SAMPLE_INPUT.lines());
        assert_eq!(4, day.part1());
    }

    #[test]
    fn sample2_for_part1_is_18() {
        const SAMPLE_INPUT_2: &str =
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let day = Day04::new(SAMPLE_INPUT_2.lines());
        assert_eq!(18, day.part1());
    }

    #[test]
    fn sample3_for_part1_is_18() {
        const SAMPLE_INPUT_3: &str =
"....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";
        let day = Day04::new(SAMPLE_INPUT_3.lines());
        assert_eq!(18, day.part1());
    }

    #[test]
    fn down_extremes() {
        const SAMPLE_INPUT: &str = "...X...
..MMM..
.A.A.A.
S..S..S";
        let day = Day04::new(SAMPLE_INPUT.lines());
        assert_eq!(3, day.part1());
    }

    #[test]
    fn up_extremes() {
        const SAMPLE_INPUT: &str =
"S..S..S
.A.A.A.
..MMM..
...X...";
        let day = Day04::new(SAMPLE_INPUT.lines());
        assert_eq!(3, day.part1());
    }

    #[test]
    fn sideways_extremes() {
        const SAMPLE_INPUT: &str =
"XMASAMX
SAMXMAS";
        let day = Day04::new(SAMPLE_INPUT.lines());
        assert_eq!(4, day.part1());
    }

    #[test]
    fn part2_first_example() {
        const SAMPLE_INPUT: &str =
"M.S
.A.
M.S";
        let day = Day04::new(SAMPLE_INPUT.lines());
        assert_eq!(1, day.part2());
    }

    #[test]
    fn part2_second_example() {
        const SAMPLE_INPUT: &str =
".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        let day = Day04::new(SAMPLE_INPUT.lines());
        assert_eq!(9, day.part2());
    }
}
