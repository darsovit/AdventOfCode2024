use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Day12 {
    garden_map: Vec<Vec<char>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
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

    fn get_neighbor(&self, plot_loc: &(usize, usize), dir: Direction) -> Option<(usize, usize)> {
        match dir {
            Direction::North => { self.valid_neighbor(plot_loc, (-1, 0)) },
            Direction::East  => { self.valid_neighbor(plot_loc, (0, 1)) },
            Direction::South => { self.valid_neighbor(plot_loc, (1, 0)) },
            Direction::West  => { self.valid_neighbor(plot_loc, (0, -1)) },
        }
    }
    fn get_neighbors(&self, plot_loc: &(usize, usize)) -> Vec<(Option<(usize, usize)>, Direction)> {
        let mut neighbors: Vec<(Option<(usize, usize)>, Direction)> = Vec::new();
        for dir in [Direction::North, Direction::East, Direction::South, Direction::West] {
            neighbors.push( (self.get_neighbor(plot_loc, dir), dir) );
        }
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

                let neighbors: Vec<((usize, usize), Direction)> = self.get_neighbors(&plot).iter()
                    .filter_map(|v| -> Option<((usize, usize), Direction)> { match v.0 { Some(pos) => { Some((pos, v.1)) }, None => { None } } }).collect();
                perimeter += 4 - neighbors.len(); // provides for 'perimeter' on edges of map

                for (neighbor, _dir) in neighbors {
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

    fn count_perimeter_sides(&self, plots_with_fences_adjacent: HashMap<(usize, usize), HashSet<Direction>>, plot_loc: &(usize, usize)) -> usize {
        assert!(plots_with_fences_adjacent.get(plot_loc).unwrap().contains(&Direction::North));
        assert!(plots_with_fences_adjacent.get(plot_loc).unwrap().contains(&Direction::West));
    
        struct FenceGraph {
            nodes: HashSet<(usize, usize)>,
            edges: HashSet<((usize, usize), (usize, usize))>,
            nodes_edges: HashMap<(usize, usize), HashSet<((usize, usize), (usize, usize))>>,
        }

        fn get_fence_node(plot: (usize, usize), fence_side: Direction) -> ((usize, usize), (usize, usize)) {
            match fence_side {
                Direction::North => ((plot.0,   plot.1), (plot.0,   plot.1+1)),
                Direction::East  => ((plot.0, plot.1+1), (plot.0+1, plot.1+1)),
                Direction::South => ((plot.0+1,   plot.1), (plot.0+1, plot.1+1)),
                Direction::West  => ((plot.0,   plot.1), (plot.0+1, plot.1))
            }
        }
        fn get_fence_edges(plot: (usize, usize), fence_sides: HashSet<Direction>) -> Vec<((usize, usize), (usize, usize))> {
            let mut fence_edges: Vec<((usize, usize), (usize, usize))> = Vec::new();
            for fence_side in fence_sides {
                fence_edges.push(get_fence_node(plot, fence_side));
            }
            fence_edges
        }
        let mut fence_graph: FenceGraph = FenceGraph{nodes: HashSet::new(), edges: HashSet::new(), nodes_edges: HashMap::new()};

        for (plot, fences_adjacent) in plots_with_fences_adjacent {
            let fence_edges = get_fence_edges(plot, fences_adjacent);
            for fence_edge in fence_edges {
                fence_graph.nodes.insert(fence_edge.0);
                fence_graph.nodes.insert(fence_edge.1);
                fence_graph.edges.insert(fence_edge);
                fence_graph.nodes_edges.entry(fence_edge.0).or_insert(HashSet::new()).insert(fence_edge);
                fence_graph.nodes_edges.entry(fence_edge.1).or_insert(HashSet::new()).insert(fence_edge);
            }
        }

        fn remove_fence_graph_node(fence_graph: &mut FenceGraph, node_to_remove: (usize, usize)) {
            let nodes_edges_to_remove: Vec<((usize, usize), (usize, usize))> = fence_graph.nodes_edges.get(&node_to_remove).unwrap().clone().into_iter().collect();
            let edge1 = nodes_edges_to_remove[0];
            let edge2 = nodes_edges_to_remove[1];
            let new_edge_left = if edge1.0 == node_to_remove { edge1.1 } else { edge1.0 };
            let new_edge_right = if edge2.0 == node_to_remove { edge2.1 } else { edge2.0 };
            let new_edge: ((usize, usize), (usize, usize)) = (new_edge_left, new_edge_right);
            fence_graph.nodes_edges.remove(&node_to_remove);
            fence_graph.edges.remove(&edge1);
            fence_graph.edges.remove(&edge2);
            fence_graph.nodes.remove(&node_to_remove);
            fence_graph.nodes_edges.entry(new_edge_left).and_modify(|v| { v.remove(&edge1); v.insert(new_edge); });
            fence_graph.nodes_edges.entry(new_edge_right).and_modify(|v| { v.remove(&edge2); v.insert(new_edge); });
            fence_graph.edges.insert(new_edge);
        }

        fn attempt_reduce_fence_graph(fence_graph: &mut FenceGraph, node_to_test: (usize, usize)) {
            match fence_graph.nodes_edges.get(&node_to_test) {
                None => { panic!("Missing node edges for node: {:?}", node_to_test); }
                Some(edges) => {
                    assert!(edges.len() == 2 || edges.len() == 4);
                    if edges.len() == 2 {
                        let node_edges_to_test: Vec<((usize, usize), (usize, usize))> = fence_graph.nodes_edges.get(&node_to_test).unwrap().clone().into_iter().collect();
                        let left_node = if node_edges_to_test[0].0 == node_to_test { node_edges_to_test[0].1 } else { node_edges_to_test[0].0 };
                        let right_node = if node_edges_to_test[1].0 == node_to_test { node_edges_to_test[1].1 } else { node_edges_to_test[1].0 };
                        if (left_node.0 == node_to_test.0 && node_to_test.0 == right_node.0)
                           || (left_node.1 == node_to_test.1 && node_to_test.1 == right_node.1 ) {
                            remove_fence_graph_node(fence_graph, node_to_test);
                        }
                    }
                }
            }
        }

        let nodes = fence_graph.nodes.clone();
        for node in nodes {
            attempt_reduce_fence_graph(&mut fence_graph, node);
        }
        fence_graph.edges.len()
    }

    fn find_connected_plots_and_perimeter_fencing(&self, garden_plant: char, plot_loc: &(usize, usize)) -> (HashSet<(usize, usize)>, usize) {
        let mut consumed_garden_plots: HashSet<(usize, usize)> = HashSet::new();
        let mut plots_to_check: VecDeque<(usize, usize)> = VecDeque::new();
        let mut perimeter_fence_sections: HashMap<(usize, usize), HashSet<Direction>> = HashMap::new();

        plots_to_check.push_back(*plot_loc);
        
        while plots_to_check.len() > 0 {
            let plot = plots_to_check.pop_front().unwrap();
            if let None = consumed_garden_plots.get(&plot) {
                consumed_garden_plots.insert(plot);
                let neighbors = self.get_neighbors(&plot);
                for (neighbor, dir) in neighbors {
                    match neighbor {
                        None => { perimeter_fence_sections.entry(plot).or_insert(HashSet::new()).insert(dir); },
                        Some(neighbor_plot) => {
                            if self.garden_map[neighbor_plot.0][neighbor_plot.1] != garden_plant {
                                perimeter_fence_sections.entry(plot).or_insert(HashSet::new()).insert(dir);
                            }
                            else if let None = consumed_garden_plots.get(&neighbor_plot) {
                                plots_to_check.push_back(neighbor_plot);
                            }
                        }
                    }
                }
            }
        }
        let perimeter_sides = self.count_perimeter_sides(perimeter_fence_sections, plot_loc);

        (consumed_garden_plots, perimeter_sides)
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



    pub fn part2(&self) -> usize {
        let mut score: usize = 0;
        let mut used_plots: HashSet<(usize, usize)> = HashSet::new();

        for (yindex, line) in (&self.garden_map).into_iter().enumerate() {
            for (xindex, garden_plant) in (&line).into_iter().enumerate() {
                let plot_loc = (yindex, xindex);
                if let None = used_plots.get(&plot_loc) {
                    let (garden_section_plots, perimeter_sides) = self.find_connected_plots_and_perimeter_fencing(*garden_plant, &plot_loc);
                    let area = garden_section_plots.len();
                    score += area * perimeter_sides;
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
    const SMALL_SAMPLE_LINES: &str =
"AAAA
BBCD
BBCC
EEEC";
    const HOLEY_SAMPLE_LINES: &str =
"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

const FULL_SAMPLE_LINES: &str =
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

    #[test]
    fn first_small_sample_part1_is_140() {
        let day = Day12::new(SMALL_SAMPLE_LINES.lines());
        assert_eq!(140, day.part1());
    }

    #[test]
    fn second_holey_sample_part1_is_772() {
        let day = Day12::new(HOLEY_SAMPLE_LINES.lines());
        assert_eq!(772, day.part1());
    }

    #[test]
    fn third_full_sample_part1_is_1930() {
        let day = Day12::new(FULL_SAMPLE_LINES.lines());
        assert_eq!(1930, day.part1());
    }

    #[test]
    fn small_sample_part2_is_80() {
        let day = Day12::new(SMALL_SAMPLE_LINES.lines());
        assert_eq!(80, day.part2());
    }

    #[test]
    fn holey_sample_part2_is_436() {
        let day = Day12::new(HOLEY_SAMPLE_LINES.lines());
        assert_eq!(436, day.part2());
    }

    #[test]
    fn full_sample_part2_is_1206() {
        let day = Day12::new(FULL_SAMPLE_LINES.lines());
        assert_eq!(1206, day.part2());
    }

    #[test]
    fn e_sample_part2_is_236() {
        const E_SAMPLE_LINES: &str =
"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let day = Day12::new(E_SAMPLE_LINES.lines());
        assert_eq!(236, day.part2());
    }

    #[test]
    fn diagonal_stress_part2_is_368() {
        const DIAGONAL_STRESS_LINES: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        let day = Day12::new(DIAGONAL_STRESS_LINES.lines());
        assert_eq!(368, day.part2());
    }
}
