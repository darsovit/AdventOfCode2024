use std::fs;
use day17::Day17;

fn main() {
    const DATAFILE: &str = "input.txt";
    let file_content = fs::read_to_string(DATAFILE);
    match file_content {
        Ok(line_content) => {
            let day = Day17::new(line_content.lines());
            println!("part1: {}", day.part1());
        },
        Err(e) => { println!("Error reading file: {}, {:?}", DATAFILE, e); }
    }
}
