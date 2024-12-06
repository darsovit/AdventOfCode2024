use std::collections::HashSet;

pub struct Day05 {
    page_ordering_rules: Vec::<(u32, u32)>,
    page_lists: Vec<Vec<u32>>,
}

impl Day05 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut empty_line: Option<usize> = None;
        let mut page_ordering_rules: Vec::<(u32, u32)> = Vec::new();
        let mut page_lists: Vec<Vec<u32>> = Vec::new();
        for (index, line) in lines.enumerate() {
            if let None = empty_line {
                if line == "" {
                    empty_line = Some(index);
                } else {
                    let mut items = line.split("|").map(|p|p.parse::<u32>().unwrap());
                    page_ordering_rules.push( (items.next().unwrap(), items.next().unwrap()) );
                    assert!(items.next() == None);
                }
            }
            else if let Some(_) = empty_line {
                let page_list: Vec<u32>  = line.split(",").map(|p|p.parse::<u32>().unwrap()).collect();
                page_lists.push(page_list);
            }
        }
        Day05{page_ordering_rules, page_lists}
    }

    fn determine_middle_page_if_good(page_list: &Vec<u32>, good: bool) -> u32 {
        if good {
            page_list[(page_list.len() - 1) / 2]
        } else {
            0
        }
    }

    fn build_hashset_of_predicates(&self, page: u32) -> HashSet<u32> {
        let mut matches = HashSet::new();
        for matching_rule in self.page_ordering_rules.clone().into_iter().filter(|(_, b)| *b==page) {
            matches.insert(matching_rule.0);
        }
        matches
    }

    fn build_hashset_from_page_list(page_list: &Vec<u32>) -> HashSet::<u32> {
        let mut hash_set = HashSet::new();
        for page in page_list { hash_set.insert(*page); }
        hash_set
    }

    fn does_page_order_follow_rules(&self, page_list: &Vec<u32>) -> bool {
        // Build HashSet from page_list contents
        // iterate through vec
        // remove vec iter from HashSet
        // get hashset of all rules with values that need to show up _before_ vec iter
        // if intersection is not empty, return false
        let mut pages_after = Self::build_hashset_from_page_list(page_list);
        for page in page_list {
            pages_after.remove(page);
            let page_set_required_before = self.build_hashset_of_predicates(*page);
            if page_set_required_before.intersection(&pages_after).count() > 0 {
                return false;
            }
        }

        true
    }

    pub fn part1(&self) -> u32 {
        let mut sum: u32 = 0;
        for page_list in &self.page_lists {
            sum += Self::determine_middle_page_if_good(&page_list, self.does_page_order_follow_rules(page_list));
        }
        sum
    }

    pub fn part2(&self) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_gives_example_out() {
        const SAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let day = Day05::new(SAMPLE_INPUT.lines());
        assert_eq!(143, day.part1());
    }
}