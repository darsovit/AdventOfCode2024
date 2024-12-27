use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Default, Debug)]
struct Graph<'a> {
    nodes: HashSet<&'a str>,
    edges: HashSet<(&'a str, &'a str)>,
}

#[derive(Default, Debug)]
pub struct Day23<'a> {
    network_map: Graph<'a>,
}

impl<'a> Day23<'a> {
    pub fn new(lines: std::str::Lines<'a>) -> Self {
        let mut network_map = Graph::default();
        for line in lines {
            let comps: Vec<&str> = line.split("-").collect();
            network_map.nodes.insert(&comps[0]);
            network_map.nodes.insert(&comps[1]);
            network_map.edges.insert((&comps[0], &comps[1]));
            network_map.edges.insert((&comps[1], &comps[0]));
        }
        Day23::<'a>{network_map}
    }

    pub fn part1(&self) -> usize {
        let mut three_way_connections = HashSet::<(&str, &str, &str)>::new();
        for node in &self.network_map.nodes {
            if node.chars().collect::<Vec<_>>()[0] == 't' {
                for edge in &self.network_map.edges {
                    if edge.0 == *node {
                        for next_edge in &self.network_map.edges {
                            if next_edge.0 == edge.1 && next_edge.1 != *node {
                                if let Some(_) = self.network_map.edges.get(&(&next_edge.1, node)) {
                                    let mut computer_set = vec![node, edge.1, next_edge.1];
                                    computer_set.sort();
                                    three_way_connections.insert((computer_set[0], computer_set[1], computer_set[2]));
                                }
                            }
                        }
                    }
                }
            }
        }

        three_way_connections.len()
    }

    fn node_connected_to_set(&self, node: &str, set_so_far: &HashSet<&str>) -> bool {
        for entry in set_so_far {
            if !self.network_map.edges.contains(&(node, entry)) { return false; }
        }
        true
    }

    fn find_set_in_progress(sets_in_progress: &Vec<HashSet<&str>>, set_to_test: &HashSet<&str>) -> bool {
        for set_in_progress in sets_in_progress {
            if set_to_test.len() == set_in_progress.intersection(set_to_test).count() { return true; }
        }
        false
    }

    fn get_largest_connected_subgraph(&self, node: &'a str, largest_so_far: usize) -> HashSet<&'a str> {
        let mut attempt_to_add_to = VecDeque::<(HashSet<&str>, HashSet<&str>)>::new();
        let mut connected_nodes = HashSet::<&str>::new();

        for edge in &self.network_map.edges {
            if edge.0 == node {
                connected_nodes.insert(edge.1);
                let mut partial_connections = HashSet::<&str>::new();
                partial_connections.insert(node);
                partial_connections.insert(edge.1);
                attempt_to_add_to.push_back((partial_connections, HashSet::<&str>::new()));
            }
        }
        if connected_nodes.len() > largest_so_far {
            let mut sets_in_progress = HashMap::<usize, Vec<HashSet<&str>>>::new();

            while attempt_to_add_to.len() > 0 {
                let (partial_connections, mut known_bad) = attempt_to_add_to.pop_front().unwrap();
                let mut sets_of_this_size = sets_in_progress.entry(partial_connections.len()).or_insert(Vec::<HashSet<&str>>::new());
                if !Self::find_set_in_progress(sets_of_this_size, &partial_connections) {
                    sets_of_this_size.push(partial_connections.clone());
                    let mut next_partials = Vec::<HashSet<&str>>::new();

                    // println!("Testing {:?}, Known Bad: {:?}", partial_connections, known_bad);
                    for node in &connected_nodes {
                        if !partial_connections.contains(node) && !known_bad.contains(node) {
                            if self.node_connected_to_set(node, &partial_connections) {
                                let mut more_work = partial_connections.clone();
                                more_work.insert(node);
                                next_partials.push(more_work);
                            } else {
                                known_bad.insert(node);
                            }
                        }
                    }
                    if next_partials.len() > 0 {
                        for partial in next_partials {
                            attempt_to_add_to.push_back((partial, known_bad.clone()));
                        }
                    }
                }
                if attempt_to_add_to.len() == 0 {
                    return partial_connections;
                }
            }
        }
        HashSet::new()
    }

    fn order_display_output(connected_set: &HashSet<&str>) -> String {
        let mut nodelist: Vec<&str> = connected_set.iter().map(|v| *v).collect();
        nodelist.sort();
        let mut output = format!("{}", nodelist[0]);
        for node in &nodelist[1..] {
            output = format!("{},{}", output, node);
        }
        output
    }

    pub fn part2(&self) -> String {
        let mut largest_connected_subgraph = HashSet::new();
        println!("Number of Nodes: {}", self.network_map.nodes.len());
        for node in &self.network_map.nodes {
            println!("Checking with node: {}", node);
            let node_list = self.get_largest_connected_subgraph(node, largest_connected_subgraph.len());
            if node_list.len() > largest_connected_subgraph.len() {
                println!("Largest so far: {:?}", node_list);
                largest_connected_subgraph = node_list;
            }
        }

        Self::order_display_output(&largest_connected_subgraph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str =
"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part1_with_sample_yields_7_networks() {
        let day = Day23::new(SAMPLE_INPUT.lines());
        assert_eq!(7, day.part1());
    }

    #[test]
    fn part2_with_sample_yields_password() {
        let day = Day23::new(SAMPLE_INPUT.lines());
        assert_eq!("co,de,ka,ta", day.part2());
    }
}
