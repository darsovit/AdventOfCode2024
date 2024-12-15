use std::collections::HashSet;
use std::collections::VecDeque;

pub struct Day12 {
    garden_map: Vec<Vec<char>>,
}

impl Day12 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut garden_map: Vec<Vec<char>> = Vec::new();
        for line in lines {
            let garden_row: Vec<char> = line.chars().collect();
            garden_map.push(garden_row);
        }
        Day12{garden_map}
    }

    fn valid_neighbor(&self, plot_loc: &(usize, usize), offset: (i64, i64)) -> Option<(usize, usize)> {
        let possible_neighbor = (plot_loc.0 as i64 + offset.0, plot_loc.1 as i64 + offset.1);
        if possible_neighbor.0 < 0 || possible_neighbor.0 as usize >= self.garden_map.len() || possible_neighbor.1 < 0 || possible_neighbor.1 as usize >= self.garden_map[0].len() {
            None
        } else {
            Some((possible_neighbor.0 as usize, possible_neighbor.1 as usize))
        }
    }

    fn get_neighbors(&self, plot_loc: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        if let Some(valid_plot) = self.valid_neighbor(plot_loc, (-1, 0)) { neighbors.push(valid_plot); }
        if let Some(valid_plot) = self.valid_neighbor(plot_loc, (0, 1)) { neighbors.push(valid_plot); }
        if let Some(valid_plot) = self.valid_neighbor(plot_loc, (1, 0)) { neighbors.push(valid_plot); }
        if let Some(valid_plot) = self.valid_neighbor(plot_loc, (0, -1)) { neighbors.push(valid_plot); }
        neighbors
    }
    fn find_connected_plots_and_perimeter(&self, garden_plant: char, plot_loc: &(usize, usize)) -> (HashSet<(usize, usize)>, usize) {
        let mut perimeter: usize = 0;  // top side and left side should be a given at the first location, but every subsequent neighbor needs to look left (after we go down)
        let mut plots_to_check: VecDeque<(usize, usize)> = VecDeque::new();
        plots_to_check.push_back(*plot_loc);
        let mut garden_section_plots: HashSet<(usize, usize)> = HashSet::new();

        while plots_to_check.len() > 0 {
            let plot = plots_to_check.pop_front().unwrap();
            if let None = garden_section_plots.get(&plot) {
                garden_section_plots.insert(plot);

                let neighbors = self.get_neighbors(&plot);
                perimeter += 4 - neighbors.len(); // provides for 'perimeter' on edges of map

                for neighbor in neighbors {
                    if self.garden_map[neighbor.0][neighbor.1] != garden_plant {
                        perimeter += 1;
                    } else if let None = garden_section_plots.get(&neighbor) {
                        plots_to_check.push_back(neighbor);
                    }
                }
            }
        }
        (garden_section_plots, perimeter)
    }

    pub fn part1(&self) -> usize {
        let mut used_plots: HashSet<(usize, usize)> = HashSet::new();
        let mut score: usize = 0;

        for (yindex, line) in (&self.garden_map).into_iter().enumerate() {
            for (xindex, garden_plant) in (&line).into_iter().enumerate() {
                let plot_loc = (yindex, xindex);
                if let None = used_plots.get(&plot_loc) {
                    let (garden_section_plots, perimeter) = self.find_connected_plots_and_perimeter(*garden_plant, &plot_loc);
                    let area = garden_section_plots.len();
                    score += area * perimeter;
                    for plot in garden_section_plots {
                        used_plots.insert(plot);
                    }
                }
            }
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_sample_part1_is_140() {
        const SAMPLE_LINES: &str =
"AAAA
BBCD
BBCC
EEEC";
        let day = Day12::new(SAMPLE_LINES.lines());
        assert_eq!(140, day.part1());
    }

    #[test]
    fn second_sample_part1_is_772() {
        const SAMPLE_LINES: &str =
"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        let day = Day12::new(SAMPLE_LINES.lines());
        assert_eq!(772, day.part1());
    }

    #[test]
    fn third_sample_part1_is_1930() {
        const SAMPLE_LINES: &str =
"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let day = Day12::new(SAMPLE_LINES.lines());
        assert_eq!(1930, day.part1());
    }
}
