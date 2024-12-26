use std::collections::HashSet;

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

        println!("{:?}", self.network_map);
        println!("Sets of three way connections: {:?}", three_way_connections);
        three_way_connections.len()
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
}
