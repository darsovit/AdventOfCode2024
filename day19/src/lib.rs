use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Default, Debug)]
struct TrieNode {
    is_end_of_word: bool,
    children: HashMap<char, TrieNode>,
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode
}

impl Trie {
    pub fn new() -> Self {
        Self{ root: TrieNode::default() }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.is_end_of_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current_node = &self.root;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        current_node.is_end_of_word
    }

    pub fn find_available_patterns<'a>(&self, smashed_words: &'a str) -> Vec<&'a str> {
        let mut current_node = &self.root;
        let mut available_patterns = Vec::<&str>::new();

        for (length, c) in smashed_words.chars().enumerate() {
            match current_node.children.get(&c) {
                Some(node) => {
                    current_node = node;
                    if current_node.is_end_of_word {
                        available_patterns.push(&smashed_words[..length+1]);
                    }
                }
                None => return available_patterns,
            }
        }
        if current_node.is_end_of_word {
            available_patterns.push(smashed_words);
        }
        available_patterns
    }
}
pub struct Day19 {
    available_towel_patterns: Trie,
    desired_displays: Vec<String>,
}

impl Day19 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut towel_patterns_line: Option<&str> = None;
        let mut displays: Vec<String> = Vec::new();

        for line in lines {
            if let None = towel_patterns_line {
                towel_patterns_line = Some(line);
            }
            else if line != "" {
                displays.push(line.to_string());
            }
        }

        let mut available_towel_patterns = Trie::new();

        if let Some(patterns) = towel_patterns_line {
            for pattern in patterns.split(", ") {
                available_towel_patterns.insert(pattern);
            }
        }
        Day19{available_towel_patterns, desired_displays: displays}
    }

    pub fn part1(&self) -> usize {
        const DEBUG: bool = false;
        let mut possible_displays = 0;

        for desired_display in &self.desired_displays {
            if DEBUG { println!("Searching for available towel patterns in {desired_display}"); }
            let mut more_to_consume = VecDeque::<(Vec<&str>, usize)>::new(); // patterns found, position in desired
            let mut fully_built_display = Vec::<Vec<&str>>::new();

            more_to_consume.push_back((Vec::<&str>::new(), 0));
            while more_to_consume.len() > 0 {
                let (found_patterns, cur_len) = more_to_consume.pop_back().unwrap();
                if cur_len == desired_display.len() {
                    if DEBUG { println!("\tFound: {:?}", found_patterns); }
                    fully_built_display.push(found_patterns);
                    break;
                } else {
                    for next_pattern in self.available_towel_patterns.find_available_patterns(&desired_display[cur_len..]) {
                        let mut found_patterns = found_patterns.clone();
                        found_patterns.push(next_pattern);
                        if DEBUG { println!("\t\tFound so far: {:?}", found_patterns); }
                        more_to_consume.push_back((found_patterns, cur_len + next_pattern.len()));
                    }
                }
            }
            if fully_built_display.len() > 0 { possible_displays += 1; }
        }
        possible_displays
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str =
"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn part1_sample_input_is_6() {
        let day = Day19::new(SAMPLE_INPUT.lines());
        assert_eq!(6, day.part1());
    }
}
