use std::fs;
use day08::Day08;

fn main() {
    const DATAFILE: &str = "input.txt";
    let file_content = fs::read_to_string(DATAFILE);
    match file_content {
        Ok(line_content) => {
            let day = Day08::new(line_content.lines(), false);
            println!("part1: {}", day.part1());
            let dayp2 = Day08::new(line_content.lines(), true);
            println!("part2: {}", dayp2.part1());
        },
        Err(e) => { println!("Error reading file: {}, {:?}", DATAFILE, e); }
    }
}
