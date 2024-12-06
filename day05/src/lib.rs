use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Day05 {
    pages_that_come_before: HashMap<u32, HashSet<u32>>,
    page_lists: Vec<Vec<u32>>,
}

impl Day05 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut empty_line: Option<usize> = None;
        
        let mut pages_that_come_before: HashMap<u32, HashSet<u32>> = HashMap::new();
        let mut page_lists: Vec<Vec<u32>> = Vec::new();
        for (index, line) in lines.enumerate() {
            if let None = empty_line {
                if line == "" {
                    empty_line = Some(index);
                } else {
                    let mut items = line.split("|").map(|p|p.parse::<u32>().unwrap());
                    let (before, after) = (items.next().unwrap(), items.next().unwrap());
                    pages_that_come_before.entry(after).or_insert(HashSet::<u32>::new());
                    pages_that_come_before.entry(after).and_modify(|e| { e.insert(before); });

                    assert!(items.next() == None);
                }
            }
            else if let Some(_) = empty_line {
                let page_list: Vec<u32>  = line.split(",").map(|p|p.parse::<u32>().unwrap()).collect();
                page_lists.push(page_list);
            }
        }
        Day05{pages_that_come_before, page_lists}
    }

    fn determine_middle_page_if_good(page_list: &Vec<u32>, good: bool) -> u32 {
        if good {
            page_list[(page_list.len() - 1) / 2]
        } else {
            0
        }
    }

    fn get_hashset_of_predicates(&self, page: u32) -> HashSet<u32> {
        match self.pages_that_come_before.get(&page) {
            Some(v) => v.clone(),
            None => HashSet::new()
        }
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
            let page_set_required_before = self.get_hashset_of_predicates(*page);
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

    fn fix_page_order(&self, page_list: &Vec<u32>) -> Vec<u32> {
        let mut page_rules: HashMap<u32, HashSet<u32>> = HashMap::new();
        let mut pages = HashSet::new();
        for page in page_list {
            page_rules.insert(*page, self.get_hashset_of_predicates(*page));
            pages.insert(*page);
        }

        let mut good_page_order: VecDeque::<u32> = VecDeque::new();

        while page_rules.len() > 0 {
            let mut pages_not_in_other_rules: Vec<u32> = Vec::new();
            for page in &pages {
                let mut found = false;
                for (_, earlier_pages) in &page_rules {
                    if earlier_pages.contains(page) {
                        found = true;
                    }
                }
                if !found { 
                    pages_not_in_other_rules.push(*page);
                }
            }

            for page in pages_not_in_other_rules {
                good_page_order.push_front(page);
                page_rules.remove(&page);
                pages.remove(&page);
            }

        }

        good_page_order.into_iter().collect()

    }

    pub fn part2(&self) -> u32 {
        let mut sum: u32 = 0;
        for page_list in &self.page_lists {
            if !self.does_page_order_follow_rules(page_list) {
                sum += Self::determine_middle_page_if_good(&self.fix_page_order(&page_list), true);
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    #[test]
    fn sample_input_gives_example_out() {
        let day = Day05::new(SAMPLE_INPUT.lines());
        assert_eq!(143, day.part1());
    }

    #[test]
    fn sample_input_gives_part2_example_out() {
        let day: Day05 = Day05::new(SAMPLE_INPUT.lines());
        assert_eq!(123, day.part2());
    }
}