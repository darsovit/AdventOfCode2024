use std::collections::HashMap;
use std::collections::HashSet;

pub struct Day08 {
    antennas: HashMap<char,Vec<(usize, usize)>>,
    max_y: usize,
    max_x: usize,
}

impl Day08 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
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
        Day08{antennas, max_y, max_x}
    }

    fn calc_antinode(&self, a: &(usize, usize), b: &(usize, usize)) -> Option<(usize, usize)> {
        let ydiff = a.0 as i64 - b.0 as i64;
        let xdiff = a.1 as i64 - b.1 as i64;
        let y = b.0 as i64 - ydiff;
        let x = b.1 as i64 - xdiff;
        if y < 0 || y as usize >= self.max_y || x < 0 || x as usize >= self.max_x {
            None
        } else {
            let antinode = (y as usize, x as usize);
            Some(antinode)
        }
    }
    fn calc_antinodes(&self, a: &(usize, usize), b: &(usize, usize), antinodes: &mut HashSet<(usize, usize)>) {
        if let Some(antinode) = self.calc_antinode(a, b) {
            antinodes.insert(antinode);
        }
        if let Some(antinode) = self.calc_antinode(b, a) {
            antinodes.insert(antinode);
        }
    }

    fn check_antenna_against_others(&self, antenna: &(usize, usize), others: &[(usize, usize)], antinodes: &mut HashSet<(usize, usize)>) {
        for other in others {
            self.calc_antinodes(&antenna, &other, antinodes);
        }
    }

    fn check_antennas_antinodes(&self, antennas: &Vec<(usize, usize)>, antinodes: &mut HashSet<(usize, usize)>) {
        // pick all pairs of the antennas at a time to determine antinodes for that pair
        for (index, antenna) in antennas.into_iter().enumerate() {
            self.check_antenna_against_others(&antenna, &antennas[index+1..], antinodes);
        }

    }

    pub fn part1(&self) -> usize {
        let mut antinodes = HashSet::<(usize, usize)>::new();
        for antenna_type in &self.antennas {
            self.check_antennas_antinodes(antenna_type.1, &mut antinodes);
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
        let day = Day08::new(SAMPLE_LINES.lines());
        assert_eq!(14, day.part1());
    }
}