use std::fs;
use day19::Day19;

fn main() {
    const DATAFILE: &str = "input.txt";
    let file_content = fs::read_to_string(DATAFILE);
    match file_content {
        Ok(line_content) => {
            let day = Day19::new(line_content.lines());
            println!("part1: {}", day.part1());
            println!("part2: {}", day.part2());
            //println!("part2: {}", day.part2());
        },
        Err(e) => { println!("Error reading file: {}, {:?}", DATAFILE, e); }
    }
}
