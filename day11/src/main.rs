use std::fs;
use day11::Day11;

fn main() {
    const DATAFILE: &str = "input.txt";
    let file_content = fs::read_to_string(DATAFILE);
    match file_content {
        Ok(line_content) => {
            let lines: Vec<_> = line_content.lines().collect();
            let day = Day11::new(lines[0]);
            println!("part1: {}", day.part1());
            //println!("part2: {}", day.part2());
        },
        Err(e) => { println!("Error reading file: {}, {:?}", DATAFILE, e); }
    }
}
