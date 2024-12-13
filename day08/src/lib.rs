use std::collections::HashMap;
use std::collections::HashSet;

pub struct Day08 {
    antennas: HashMap<char,Vec<(usize, usize)>>,
    max_y: usize,
    max_x: usize,
    consider_harmonics: bool,
}

impl Day08 {
    pub fn new(lines: std::str::Lines<'_>, consider_harmonics: bool) -> Self {
        let mut antennas = HashMap::new();
        let mut max_y = 0;
        let mut max_x = 0;
        for (yindex, line) in lines.enumerate() {
            max_y += 1;
            max_x = line.len();
            for (xindex, a_char) in line.chars().enumerate() {
                match a_char {
                    '.' => { /* no-op */ },
                    val => {
                        antennas.entry(val).or_insert(Vec::<(usize, usize)>::new()).push((yindex, xindex));
                    }
                }
            }
        }
        Day08{antennas, max_y, max_x, consider_harmonics}
    }

    fn is_possible_antinode_in_range(&self, possible_antinode: (i64, i64)) -> bool {
        possible_antinode.0 >= 0 && self.max_y > possible_antinode.0 as usize && possible_antinode.1 >= 0 && self.max_x > possible_antinode.1 as usize
    }

    fn add_possible_antinode(possible_antinode: (i64, i64), antinodes: &mut HashSet<(usize, usize)>) {
        let antinode= (possible_antinode.0 as usize, possible_antinode.1 as usize);
        antinodes.insert(antinode);
    }

    fn calc_antinode_from(&self, a: &(usize, usize), b: &(usize, usize), antinodes: &mut HashSet<(usize, usize)>) {
        let ydiff = b.0 as i64 - a.0 as i64;
        let xdiff = b.1 as i64 - a.1 as i64;
        let mut base_antinode= (b.0 as i64 + ydiff, b.1 as i64 + xdiff);
        if self.is_possible_antinode_in_range(base_antinode) {
            Self::add_possible_antinode(base_antinode, antinodes);
            if self.consider_harmonics
            {
                loop {
                    let possible_antinode = (base_antinode.0 + ydiff, base_antinode.1 + xdiff);
                    if self.is_possible_antinode_in_range(possible_antinode) {
                        Self::add_possible_antinode(possible_antinode, antinodes);
                        base_antinode = possible_antinode;
                    }
                    else {
                        break;
                    }
                }
            }
        }
    }
    fn calc_antinodes(&self, a: &(usize, usize), b: &(usize, usize), antinodes: &mut HashSet<(usize, usize)>) {
        self.calc_antinode_from(a, b, antinodes);
        self.calc_antinode_from(b, a, antinodes);
    }

    fn check_antenna_against_others(&self, antenna: &(usize, usize), others: &[(usize, usize)], antinodes: &mut HashSet<(usize, usize)>) {
        for other in others {
            self.calc_antinodes(&antenna, &other, antinodes);
        }
    }

    fn check_antennas_antinodes(&self, antennas: &Vec<(usize, usize)>, antinodes: &mut HashSet<(usize, usize)>) {
        // pick all pairs of the antennas at a time to determine antinodes for that pair
        for (index, antenna) in antennas.into_iter().enumerate() {
            if self.consider_harmonics {
                antinodes.insert(*antenna);
            }
            self.check_antenna_against_others(&antenna, &antennas[index+1..], antinodes);
        }
    }

    pub fn part1(&self) -> usize {
        let mut antinodes = HashSet::<(usize, usize)>::new();
        for antenna_type in &self.antennas {
            if antenna_type.1.len() > 1 {
                self.check_antennas_antinodes(antenna_type.1, &mut antinodes);
            }
        }
        antinodes.len()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_LINES: &str =
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn sample_with_part1_is_14() {
        let day = Day08::new(SAMPLE_LINES.lines(), false);
        assert_eq!(14, day.part1());
    }

    #[test]
    fn sample_with_part2_is_34() {
        let day  = Day08::new(SAMPLE_LINES.lines(), true);
        assert_eq!(34, day.part1());
    }

    #[test]
    fn part2_sample_is_9() {
        const PART2_SAMPLE_LINES: &str =
"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";
        let day = Day08::new(PART2_SAMPLE_LINES.lines(), true);
        assert_eq!(9, day.part1());
    }
}
